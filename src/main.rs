use afetch::system::infos::Infos;
use afetch::translations::list::{language_code_list, language_list};
use afetch::utils;
use afetch::utils::convert_to_readable_unity;
use afetch_colored::{Colorize, CustomColor};
use image::GenericImageView;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::process::exit;
use sysinfo::{Cpu, CpuExt, DiskExt, NetworkExt, SystemExt};
use viuer::Config as ViuerConfig;
use whoami::{hostname, username};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct LogoConfig {
    status: String,
    char_type: String,
    picture_path: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    language: String,
    logo: LogoConfig,
    text_color: Vec<u8>,
    text_color_header: Vec<u8>,
    disabled_entries: Vec<String>,
}

fn main() {
    let afetch_config_path: std::path::PathBuf = dirs::config_dir().unwrap_or_else(|| {
        println!("An error occurred while retrieving the configuration files folder, please open an issue at: https://github.com/Asthowen/AFetch/issues/new so that we can solve your issue.");
        exit(9);
    }).join("afetch").join("config.yaml");

    if !afetch_config_path
        .parent()
        .unwrap_or(&afetch_config_path)
        .exists()
    {
        std::fs::create_dir_all(&afetch_config_path).unwrap_or_else(|e| {
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
    let logo_color: CustomColor =
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
    let logo_type: i8 = if yaml.logo.status == "enable" {
        i8::from(yaml.logo.char_type != "braille")
    } else {
        2
    };
    let logo: Option<Vec<&str>> = if logo_type == 0 {
        Option::from(infos.get_os_logo().lines().collect::<Vec<&str>>())
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

    if !yaml.disabled_entries.contains(&"os".to_owned()) {
        let system_name: String = infos.sysinfo_obj.name().unwrap_or_else(|| "".to_owned());
        if !system_name.is_empty() {
            if system_name.to_lowercase().contains("windows") {
                infos_to_print.push(
                    format!(
                        "{}{} {}",
                        language["label-os"].bold().custom_color(header_color),
                        system_name,
                        infos
                            .sysinfo_obj
                            .os_version()
                            .unwrap_or_else(|| "   ".to_owned())
                            .split(' ')
                            .collect::<Vec<&str>>()[0]
                    )
                    .custom_color(logo_color)
                    .to_string(),
                );
            } else {
                infos_to_print.push(
                    format!(
                        "{}{}",
                        language["label-os"].bold().custom_color(header_color),
                        system_name
                    )
                    .custom_color(logo_color)
                    .to_string(),
                );
            }
        }
    }
    if !yaml.disabled_entries.contains(&"host".to_owned()) {
        let host: String = infos.get_host();
        if !host.is_empty() {
            infos_to_print.push(format!(
                "{}{}",
                language["label-host"].bold().custom_color(header_color),
                host.custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"kernel".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-kernel"].bold().custom_color(header_color),
            infos
                .sysinfo_obj
                .kernel_version()
                .unwrap_or_else(|| "".to_owned())
                .replace('\n', "")
                .custom_color(logo_color)
        ));
    }
    if !yaml.disabled_entries.contains(&"uptime".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-uptime"].bold().custom_color(header_color),
            utils::format_time(infos.sysinfo_obj.uptime(), &language).custom_color(logo_color)
        ));
    }
    if !yaml.disabled_entries.contains(&"packages".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-packages"].bold().custom_color(header_color),
            infos.get_packages_number().custom_color(logo_color)
        ));
    }
    if !yaml.disabled_entries.contains(&"resolution".to_owned()) {
        let screens_resolution = infos.get_screens_resolution();
        if !screens_resolution.is_empty() {
            infos_to_print.push(format!(
                "{}{}",
                language["label-resolution"]
                    .bold()
                    .custom_color(header_color),
                screens_resolution.custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"desktop".to_owned()) {
        let de_infos: (String, String) = infos.get_de();
        if !de_infos.0.is_empty() {
            infos_to_print.push(format!(
                "{}{}",
                language["label-desktop"].bold().custom_color(header_color),
                format!(
                    "{} {}",
                    de_infos.0,
                    if !yaml
                        .disabled_entries
                        .contains(&"desktop-version".to_owned())
                    {
                        de_infos.1
                    } else {
                        "".to_owned()
                    }
                )
                .custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"shell".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-shell"].bold().custom_color(header_color),
            infos.get_shell().custom_color(logo_color)
        ));
    }
    if !yaml.disabled_entries.contains(&"terminal".to_owned()) {
        let terminal: String = infos.get_terminal();
        if !terminal.is_empty() {
            infos_to_print.push(format!(
                "{}{}",
                language["label-terminal"].bold().custom_color(header_color),
                terminal.custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"memory".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-memory"].bold().custom_color(header_color),
            format!(
                "{}/{}",
                convert_to_readable_unity(infos.sysinfo_obj.used_memory() as f64),
                convert_to_readable_unity(infos.sysinfo_obj.total_memory() as f64)
            )
            .custom_color(logo_color)
        ));
    }
    if !yaml.disabled_entries.contains(&"cpu".to_owned()) {
        let cpu_infos: &Cpu = infos.sysinfo_obj.global_cpu_info();
        let cpu_name: String = if !cpu_infos.brand().is_empty() {
            cpu_infos.brand().to_owned()
        } else if !infos.sysinfo_obj.global_cpu_info().vendor_id().is_empty() {
            cpu_infos.vendor_id().to_owned()
        } else {
            "".to_owned()
        };

        if cpu_name.is_empty() {
            infos_to_print.push(format!(
                "{}{:.5}%",
                language["label-cpu"].bold().custom_color(header_color),
                cpu_infos.cpu_usage().to_string().custom_color(logo_color)
            ));
        } else {
            infos_to_print.push(format!(
                "{}{}",
                language["label-cpu"].bold().custom_color(header_color),
                format!("{} - {:.1}%", cpu_name, cpu_infos.cpu_usage()).custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"network".to_owned()) {
        let (mut network_sent, mut network_recv) = (0, 0);
        for (_, data) in infos.sysinfo_obj.networks() {
            network_sent += data.transmitted();
            network_recv += data.received();
        }
        infos_to_print.push(format!(
            "{}{}",
            language["label-network"].bold().custom_color(header_color),
            format!(
                "{}/s ↘  {}/s ↗",
                convert_to_readable_unity(network_sent as f64),
                convert_to_readable_unity(network_recv as f64)
            )
            .custom_color(logo_color)
        ));
    }
    let print_disk: bool = !yaml.disabled_entries.contains(&"disk".to_owned());
    let print_disks: bool = !yaml.disabled_entries.contains(&"disks".to_owned());

    if print_disks || print_disk {
        let (mut total_disk_used, mut total_disk_total) = (0, 0);
        for disk in infos.sysinfo_obj.disks() {
            let disk_mount_point: String = disk.mount_point().to_str().unwrap().to_owned();
            if !disk_mount_point.contains("/docker") && !disk_mount_point.contains("/boot") {
                total_disk_used += disk.total_space() - disk.available_space();
                total_disk_total += disk.total_space();
                if print_disk {
                    infos_to_print.push(format!(
                        "{}{}{}",
                        language["label-disk"].bold().custom_color(header_color),
                        format!("({})", disk.mount_point().to_str().unwrap_or(""),)
                            .custom_color(header_color),
                        format!(
                            "{}{}/{}",
                            language["label-disk-1"],
                            convert_to_readable_unity(
                                (disk.total_space() - disk.available_space()) as f64
                            ),
                            convert_to_readable_unity(disk.total_space() as f64)
                        )
                        .custom_color(logo_color)
                    ));
                }
            }
        }
        if print_disks {
            infos_to_print.push(format!(
                "{}{}",
                language["label-disks"].bold().custom_color(header_color),
                format!(
                    "{}/{}",
                    convert_to_readable_unity(total_disk_used as f64),
                    convert_to_readable_unity(total_disk_total as f64)
                )
                .custom_color(logo_color)
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"public-ip".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-public-ip"]
                .bold()
                .custom_color(header_color),
            infos.get_public_ip().custom_color(logo_color)
        ));
    }

    if !yaml.disabled_entries.contains(&"battery".to_owned()) {
        let manager_result = battery::Manager::new();
        if let Ok(manager) = manager_result {
            let batteries_infos_result = manager.batteries();
            if let Ok(mut batteries_infos) = batteries_infos_result {
                if let Some(Ok(battery_infos)) = batteries_infos.next() {
                    infos_to_print.push(format!(
                        "{}{:.4}%",
                        language["label-battery"].bold().custom_color(header_color),
                        (battery_infos.state_of_charge().value * 100.0)
                            .to_string()
                            .custom_color(logo_color)
                    ));
                }
            }
        }
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
        println!();
        for info in &infos_to_print {
            output += &format!(" {}\n", info);
        }
        println!("{}", output);
    }
}
