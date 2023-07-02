use afetch::config::Config;
use afetch::system::getters::{
    get_battery, get_cpu, get_desktop, get_disks, get_host, get_kernel, get_memory, get_network,
    get_os, get_packages, get_public_ip, get_resolution, get_shell, get_terminal,
    get_terminal_font, get_uptime, get_wm,
};
use afetch::system::infos::Infos;
use afetch::translations::list::{language_code_list, language_list};
use afetch_colored::{AnsiOrCustom, Colorize, CustomColor};
use image::GenericImageView;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
use unicode_segmentation::UnicodeSegmentation;
use viuer::Config as ViuerConfig;
use whoami::{hostname, username};

#[tokio::main]
async fn main() {
    let afetch_config_parent_path: PathBuf = dirs::config_dir().unwrap_or_else(|| {
        println!("An error occurred while retrieving the configuration files folder, please open an issue at: https://github.com/Asthowen/AFetch/issues/new so that we can solve your issue.");
        exit(9);
    }).join("afetch");
    let afetch_config_path = afetch_config_parent_path.join("config.yaml");

    if !afetch_config_parent_path.exists() {
        std::fs::create_dir_all(&afetch_config_parent_path).unwrap_or_else(|e| {
            println!(
                "An error occurred while creating the configuration files: {}",
                e
            );
            exit(9);
        });
    }

    let yaml_to_parse: String = if afetch_config_path.exists() {
        std::fs::read_to_string(afetch_config_path).unwrap_or_default()
    } else {
        let to_write: String = "language: auto # en / fr / auto \nlogo:\n  status: enable # disable / enable\n  char_type: braille # braille / picture\n  picture_path: none # `the file path: eg: ~/pictures/some.png` / none\ntext_color:\n  - 255 # r\n  - 255 # g\n  - 255 # b\n# text_color_header:\n#   - 133 # r\n#   - 218 # g\n#   - 249 # b\ndisabled_entries:\n  - battery\n  - public-ip\n  - cpu-usage\n  - network".to_owned();
        if let Err(e) = std::fs::write(afetch_config_path, &to_write) {
            println!(
                "An error occurred while creating the configuration file: {}",
                e
            );
            exit(9);
        }
        to_write
    };
    let yaml: Config = serde_yaml::from_str(&yaml_to_parse).unwrap_or_else(|e| {
        println!("Your configuration is malformed ({})", e);
        exit(9);
    });
    let text_color: CustomColor =
        CustomColor::new(yaml.text_color[0], yaml.text_color[1], yaml.text_color[2]);

    let language: HashMap<&'static str, &'static str> = if yaml.language == "auto" {
        let locale_value_base: String = sys_locale::get_locale()
            .unwrap_or_else(|| String::from("en-US"))
            .replace('_', "-");
        let locale_value_split: Vec<&str> = locale_value_base.split('-').collect::<Vec<&str>>();
        let locale_value: String = if locale_value_split.is_empty() {
            locale_value_base
        } else {
            locale_value_split[0].to_owned()
        };
        if language_code_list().contains(&locale_value.as_str()) {
            language_list()[locale_value.as_str()].clone()
        } else {
            language_list()["en"].clone()
        }
    } else if language_code_list().contains(&yaml.language.as_str()) {
        language_list()[yaml.language.as_str()].clone()
    } else {
        language_list()["en"].clone()
    };

    let cli_args: Vec<String> = std::env::args().collect();
    let search_logo_arg_opt: Option<usize> =
        cli_args.iter().position(|r| r.to_lowercase() == "--logo");
    let custom_logo: Option<String> = search_logo_arg_opt.and_then(|search_logo_arg| {
        cli_args
            .get(search_logo_arg + 1)
            .map(|logo| logo.to_lowercase())
    });

    let infos: Infos = Infos::init(custom_logo);

    let shared_yaml: Arc<Config> = Arc::new(yaml.clone());
    let shared_logo_color: Arc<CustomColor> = Arc::new(text_color);
    let shared_language = Arc::new(language.clone());
    let shared_infos: Arc<Infos> = Arc::new(infos);

    let logo_type: i8 = if yaml.logo.status == "enable" {
        i8::from(yaml.logo.char_type != "braille")
    } else {
        2
    };
    let logo: Option<[&str; 2]> = if logo_type == 0 {
        shared_infos.get_os_logo()
    } else {
        None
    };
    let logo_lines_option: Option<Vec<&str>> =
        logo.map(|logo| logo[1].lines().collect::<Vec<&str>>());

    let header_color: AnsiOrCustom = if let Some(text_color_header) = yaml.text_color_header {
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

    let (username, host): (String, String) = (username(), hostname());
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

    let (
        os_result,
        host_result,
        kernel_result,
        uptime_result,
        packages_result,
        shell_result,
        resolution_result,
        desktop_result,
        wm_result,
        terminal_result,
        terminal_font_result,
        cpu_result,
        memory_result,
        network_result,
        disks_result,
        public_ip_result,
        battery_result,
    ) = tokio::join!(
        get_os(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_host(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_kernel(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_uptime(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_packages(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_shell(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_resolution(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_desktop(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_wm(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_terminal(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_terminal_font(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_cpu(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_memory(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_network(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_disks(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_public_ip(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
            Arc::clone(&shared_infos)
        ),
        get_battery(
            Arc::clone(&shared_yaml),
            Arc::clone(&shared_header_color),
            Arc::clone(&shared_logo_color),
            Arc::clone(&shared_language),
        )
    );

    if let Some(os) = os_result {
        infos_to_print.push(os);
    }
    if let Some(host) = host_result {
        infos_to_print.push(host);
    }
    if let Some(kernel) = kernel_result {
        infos_to_print.push(kernel);
    }
    if let Some(uptime) = uptime_result {
        infos_to_print.push(uptime);
    }
    if let Some(packages) = packages_result {
        infos_to_print.push(packages);
    }
    if let Some(shell) = shell_result {
        infos_to_print.push(shell);
    }
    if let Some(resolution) = resolution_result {
        infos_to_print.push(resolution);
    }
    if let Some(desktop) = desktop_result {
        infos_to_print.push(desktop);
    }
    if let Some(wm) = wm_result {
        infos_to_print.push(wm);
    }
    if let Some(terminal) = terminal_result {
        infos_to_print.push(terminal);
    }
    if let Some(terminal_font) = terminal_font_result {
        infos_to_print.push(terminal_font);
    }
    if let Some(cpu) = cpu_result {
        infos_to_print.push(cpu);
    }
    if let Some(memory) = memory_result {
        infos_to_print.push(memory);
    }
    if let Some(network) = network_result {
        infos_to_print.push(network);
    }
    if let Some(mut disks) = disks_result {
        infos_to_print.append(&mut disks);
    }
    if let Some(public_ip) = public_ip_result {
        infos_to_print.push(public_ip);
    }
    if let Some(battery) = battery_result {
        infos_to_print.push(battery);
    }

    if !yaml.disabled_entries.contains(&"color-blocks".to_owned()) {
        let first_colors: String = (0..8).map(|i| format!("\x1b[4{}m   \x1b[0m", i)).collect();
        let second_colors: String = (0..8).map(|i| format!("\x1b[10{}m   \x1b[0m", i)).collect();
        infos_to_print.extend(vec![
            String::default(),
            String::default(),
            first_colors,
            second_colors,
        ]);
    }

    if let Some(logo_lines) = logo_lines_option {
        let logo_escape_u8: Vec<u8> =
            strip_ansi_escapes::strip(logo.unwrap_or_default()[1]).unwrap();
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
    } else if logo_type == 1 {
        println!();
        for info in &infos_to_print {
            writeln!(output, "{}{}", " ".repeat(47), info).ok();
        }
        print!("{}\x1b[{}A", output, infos_to_print.len());

        let image = match image::open(&yaml.logo.picture_path) {
            Ok(image) => image,
            Err(e) => {
                println!("An error occurred while loading the image: {}", e);
                exit(9);
            }
        };
        let dimensions: (u32, u32) = image.dimensions();
        let width_ratio: f64 = dimensions.0 as f64 / 44.0;
        let height_ratio: f64 = dimensions.1 as f64 / 44.0;
        let ratio: f64 = width_ratio.max(height_ratio);
        let new_width: u32 = (dimensions.0 as f64 / ratio) as u32;

        let viuer_config: ViuerConfig = ViuerConfig {
            x: 0,
            width: Some(new_width),
            absolute_offset: false,
            ..Default::default()
        };
        viuer::print_from_file(yaml.logo.picture_path, &viuer_config).ok();

        println!();
    } else {
        for info in &infos_to_print {
            writeln!(output, " {}", info).ok();
        }
        println!("\n{}", output);
    }
}
