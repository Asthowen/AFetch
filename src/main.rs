use afetch::system::getters::{
    get_battery, get_cpu, get_desktop, get_disks, get_host, get_kernel, get_memory, get_network,
    get_os, get_packages, get_public_ip, get_resolution, get_shell, get_terminal,
    get_terminal_font, get_uptime,
};
use afetch::system::infos::Infos;
use afetch::translations::list::{language_code_list, language_list};
use afetch::utils::Config;
use afetch_colored::{Colorize, CustomColor};
use image::GenericImageView;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::PathBuf;
use std::process::exit;
use std::sync::Arc;
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
        let to_write: String = "language: auto # en / fr / auto \nlogo:\n  status: enable # disable / enable\n  char_type: braille # braille / picture\n  picture_path: none # `the file path: eg: ~/pictures/some.png` / none\ntext_color:\n  - 255 # r\n  - 255 # g\n  - 255 # b\ntext_color_header:\n  - 133 # r\n  - 218 # g\n  - 249 # b\ndisabled_entries:\n  - battery\n  - public-ip\n  - network".to_owned();
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
    let header_color: CustomColor = CustomColor::new(
        yaml.text_color_header[0],
        yaml.text_color_header[1],
        yaml.text_color_header[2],
    );
    let logo_color = CustomColor::new(yaml.text_color[0], yaml.text_color[1], yaml.text_color[2]);

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

    let cli_args: Vec<_> = std::env::args().collect();
    let search_logo_arg_opt = cli_args.iter().position(|r| r.to_lowercase() == "--logo");
    let custom_logo: Option<String> = if let Some(search_logo_arg) = search_logo_arg_opt {
        if let Some(logo) = cli_args.get(search_logo_arg + 1) {
            Option::from(logo.to_lowercase())
        } else {
            None
        }
    } else {
        None
    };

    let infos: Infos = Infos::init(custom_logo);

    let shared_yaml = Arc::new(yaml.clone());
    let shared_header_color = Arc::new(header_color);
    let shared_logo_color = Arc::new(logo_color);
    let shared_language = Arc::new(language.clone());
    let shared_infos = Arc::new(infos);

    let logo_type: i8 = if yaml.logo.status == "enable" {
        i8::from(yaml.logo.char_type != "braille")
    } else {
        2
    };
    let logo: Option<Vec<&str>> = if logo_type == 0 {
        Option::from(shared_infos.get_os_logo().lines().collect::<Vec<&str>>())
    } else {
        None
    };

    let (username, host): (String, String) = (username(), hostname());
    let mut infos_to_print: Vec<String> = Vec::new();
    let mut output: String = "".to_owned();
    infos_to_print.push(format!(
        "{}{}{}",
        username.custom_color(header_color).bold(),
        "@".custom_color(logo_color),
        host.custom_color(header_color).bold()
    ));
    infos_to_print.push(format!(
        "{}",
        "─"
            .repeat(username.len() + host.len() + 1)
            .custom_color(logo_color)
    ));

    let (
        os_result,
        host_result,
        kernel_result,
        uptime_result,
        packages_result,
        resolution_result,
        desktop_result,
        shell_result,
        terminal_result,
        terminal_font_result,
        memory_result,
        cpu_result,
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
        get_shell(
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
        get_memory(
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
    if let Some(resolution) = resolution_result {
        infos_to_print.push(resolution);
    }
    if let Some(desktop) = desktop_result {
        infos_to_print.push(desktop);
    }
    if let Some(shell) = shell_result {
        infos_to_print.push(shell);
    }
    if let Some(terminal) = terminal_result {
        infos_to_print.push(terminal);
    }
    if let Some(terminal_font) = terminal_font_result {
        infos_to_print.push(terminal_font);
    }
    if let Some(memory) = memory_result {
        infos_to_print.push(memory);
    }
    if let Some(cpu) = cpu_result {
        infos_to_print.push(cpu);
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
        let mut first_colors: String = "".to_owned();
        let mut second_colors: String = "".to_owned();
        for i in 0..8 {
            first_colors += &format!("\x1b[4{}m   \x1b[0m", i);
            second_colors += &format!("\x1b[10{}m   \x1b[0m", i);
        }
        infos_to_print.push("".to_owned());
        infos_to_print.push("".to_owned());
        infos_to_print.push(first_colors);
        infos_to_print.push(second_colors);
    }

    if let Some(logo) = logo {
        let mut last_index: usize = 0;

        for (i, info) in infos_to_print.into_iter().enumerate() {
            if logo.len() > i {
                output += &format!("{}{}   {}\n", logo[i], "".white(), info);
            } else {
                output += &format!("{}{}\n", " ".repeat(45), info);
            }
            last_index += 1;
        }

        if last_index < logo.len() {
            for logo_line in &logo[last_index..] {
                output += &format!("{}\n", logo_line);
            }
        }

        println!("{}", output);
    } else if logo_type == 1 {
        println!();
        for info in &infos_to_print {
            output += &format!("{}{}\n", " ".repeat(47), info);
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
            write!(output, " {}\n", info).ok();
        }
        println!("\n{}", output);
    }
}
