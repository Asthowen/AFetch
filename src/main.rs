use afetch::config::Config;
use afetch::error::FetchInfosError;
use afetch::system::futures::{create_futures, FutureResultType};
use afetch::system::infos::os_logo::get_os_logo;
use afetch::translations::{get_language, language_code_list};
use afetch_colored::{AnsiOrCustom, Colorize, CustomColor};
#[cfg(feature = "image")]
use image::GenericImageView;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;
#[cfg(feature = "image")]
use viuer::Config as ViuerConfig;
use whoami::{fallible::hostname, username};

const DEFAULT_CONFIG: &str = "language: auto # en / fr / auto \nlogo:\n  status: true\n  char_type: braille # braille / picture\n  picture_path: none # `the file path: eg: ~/pictures/some.png` / none\ntext_color:\n  - 255 # r\n  - 255 # g\n  - 255 # b\n# text_color_header:\n#   - 133 # r\n#   - 218 # g\n#   - 249 # b\nentries:\n  os:\n  host:\n  kernel:\n  uptime:\n  packages:\n  resolution:\n  shell:\n  desktop-environment:\n  terminal:\n  terminal-font:\n  cpu:\n  gpus:\n  memory:\n  disks:\n  empty-line:\n  empty-line:\n  color-blocks:";

#[tokio::main]
async fn main() -> Result<(), FetchInfosError> {
    let afetch_config_parent_path: PathBuf = dirs::config_dir()
        .ok_or_else(||
            FetchInfosError::error_exit(
                "An error occurred while retrieving the configuration files folder, please open an issue at: https://github.com/Asthowen/AFetch/issues/new so that we can solve your issue.".to_owned()
            )
        )?.join("afetch");
    let afetch_config_path = afetch_config_parent_path.join("config.yaml");

    if !afetch_config_parent_path.exists() {
        tokio::fs::create_dir_all(&afetch_config_parent_path)
            .await
            .map_err(|e| {
                FetchInfosError::error_exit(format!(
                    "An error occurred while creating the configuration files: {}",
                    e
                ))
            })?;
    }

    let yaml_to_parse: String = if afetch_config_path.exists() {
        tokio::fs::read_to_string(afetch_config_path)
            .await
            .unwrap_or_default()
    } else {
        tokio::fs::write(afetch_config_path, DEFAULT_CONFIG)
            .await
            .map_err(|e| {
                FetchInfosError::error(format!(
                    "An error occurred while creating the configuration file: {}",
                    e
                ))
            })?;
        DEFAULT_CONFIG.to_owned()
    };

    let yaml: Config = serde_yaml::from_str(&yaml_to_parse).unwrap_or_else(|error| match serde_yaml::from_str(DEFAULT_CONFIG) {
        Ok(config) => {
            println!("Your configuration is malformed ({}), I therefore use the default configuration.", error);
            config
        }
        Err(error1) => {
            println!("Your configuration is malformed ({}), unfortunately I couldn't load the default configuration ({}).", error, error1);
            exit(9);
        }
    });
    let text_color: CustomColor =
        CustomColor::new(yaml.text_color[0], yaml.text_color[1], yaml.text_color[2]);

    #[cfg(feature = "image")]
    let picture_path = yaml.logo.picture_path.clone();

    let language: HashMap<&'static str, &'static str> = if yaml.language == "auto" {
        let locale_value_base: String = sys_locale::get_locale()
            .unwrap_or_else(|| String::from("en-US"))
            .replace('_', "-");
        let locale_value: &str = locale_value_base
            .split('-')
            .next()
            .unwrap_or(&locale_value_base);
        if language_code_list().contains(&locale_value) {
            get_language(locale_value)
        } else {
            get_language("en")
        }
    } else if language_code_list().contains(&yaml.language.as_str()) {
        get_language(yaml.language.as_str())
    } else {
        get_language("en")
    };

    let cli_args: Vec<String> = std::env::args().collect();
    let search_logo_arg_opt: Option<usize> =
        cli_args.iter().position(|r| r.to_lowercase() == "--logo");
    let custom_logo: Option<String> = search_logo_arg_opt.and_then(|search_logo_arg| {
        cli_args
            .get(search_logo_arg + 1)
            .map(|logo| logo.to_lowercase())
    });

    let shared_logo_color: Arc<CustomColor> = Arc::new(text_color);
    let logo_type: i8 = if !supports_unicode::on(supports_unicode::Stream::Stdout) {
        3
    } else if yaml.logo.status && cfg!(feature = "image") && yaml.logo.char_type != "braille" {
        1
    } else if yaml.logo.status {
        0
    } else {
        2
    };
    let logo: Option<[&str; 2]> = if logo_type == 0 {
        get_os_logo(custom_logo).await?
    } else {
        None
    };
    let header_color: AnsiOrCustom = if let Some(text_color_header) = &yaml.text_color_header {
        AnsiOrCustom::Custom(CustomColor::new(
            text_color_header[0],
            text_color_header[1],
            text_color_header[2],
        ))
    } else if let Some(logo) = logo {
        AnsiOrCustom::Ansi(logo[0].parse::<u8>().unwrap())
    } else {
        AnsiOrCustom::Ansi(6)
    };

    let shared_header_color = Arc::new(header_color);

    let (username, host): (String, String) = (username(), hostname().unwrap_or_default());
    let mut infos_to_print: Vec<String> = Vec::new();
    let mut output: String = String::default();
    infos_to_print.push(format!(
        "{}{}{}",
        username
            .custom_color_or_ansi_color_code(header_color)
            .bold(),
        "@".custom_color(text_color),
        host.custom_color_or_ansi_color_code(header_color).bold()
    ));
    infos_to_print.push(format!(
        "{}",
        "─"
            .repeat(username.len() + host.len() + 1)
            .custom_color(text_color)
    ));

    let mut futures = create_futures(
        Arc::new(yaml),
        Arc::clone(&shared_header_color),
        Arc::clone(&shared_logo_color),
        Arc::new(language),
    );

    while let Some(Ok(Ok(result))) = futures.join_next().await {
        if let Some(result) = result {
            match result {
                FutureResultType::String(result) => infos_to_print.push(result),
                FutureResultType::List(mut result) => infos_to_print.append(&mut result),
            }
        }
    }

    let logo_lines_option: Option<Vec<&str>> =
        logo.map(|logo| logo[1].lines().collect::<Vec<&str>>());

    if let Some(logo_lines) = logo_lines_option {
        let logo_escape_u8: Vec<u8> = strip_ansi_escapes::strip(logo.unwrap_or_default()[1]);
        let logo_escape = String::from_utf8_lossy(&logo_escape_u8);
        let logo_escape_lines: Vec<&str> = logo_escape.lines().collect::<Vec<&str>>();
        let mut max_line_length: usize = 0;

        for line in logo_escape_lines {
            let line_length: usize = line.graphemes(true).count() + 6;
            if line_length > max_line_length {
                max_line_length = line_length;
            }
        }

        let mut last_index: usize = 0;
        for (i, info) in infos_to_print.into_iter().enumerate() {
            if logo_lines.len() > i {
                writeln!(output, "   {}{}   {}", logo_lines[i], "".white(), info).ok();
            } else {
                writeln!(output, "{}{}", " ".repeat(max_line_length), info).ok();
            }
            last_index += 1;
        }

        if last_index < logo_lines.len() {
            for logo_line in &logo_lines[last_index..] {
                writeln!(output, "   {}{}   ", logo_line, "".white()).ok();
            }
        }

        println!("\n{}", output);
        return Ok(());
    }

    #[cfg(feature = "image")]
    if logo_type == 1 {
        println!();
        for info in &infos_to_print {
            writeln!(output, "{}{}", " ".repeat(47), info).ok();
        }
        print!("{}\x1b[{}A", output, infos_to_print.len());

        let buffer: Vec<u8> = tokio::fs::read(&picture_path).await.map_err(|e| {
            FetchInfosError::error_exit(format!("An error occurred while reading the image: {}", e))
        })?;

        let image = image::io::Reader::new(std::io::Cursor::new(buffer))
            .with_guessed_format()
            .map_err(|e| {
                FetchInfosError::error(format!(
                    "An error occurred while guessing the image format: {}",
                    e
                ))
            })?
            .decode()
            .map_err(|e| {
                FetchInfosError::error(format!("An error occurred while decoding the image: {}", e))
            })?;

        let dimensions: (u32, u32) = image.dimensions();
        let width_ratio: f64 = dimensions.0 as f64 / 44.0;
        let height_ratio: f64 = dimensions.1 as f64 / 44.0;
        let ratio: f64 = width_ratio.max(height_ratio);
        let new_width: u32 = (dimensions.0 as f64 / ratio) as u32;

        let viuer_config: ViuerConfig = ViuerConfig {
            x: 0,
            width: Some(new_width),
            absolute_offset: false,
            ..ViuerConfig::default()
        };
        viuer::print(&image, &viuer_config).map_err(|e| {
            FetchInfosError::error_exit(format!(
                "An error occurred while printing the image: {}",
                e
            ))
        })?;

        println!();
        return Ok(());
    }

    for info in &infos_to_print {
        writeln!(output, " {}", info).ok();
    }
    println!("\n{}", output);

    Ok(())
}
