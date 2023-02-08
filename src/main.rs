use afetch::system::infos::Infos;
use afetch::translations::list::{language_code_list, language_list};
use afetch::utils;
use afetch::utils::convert_to_readable_unity;
use colored::Colorize;
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
    color: Vec<u8>,
    color_header: Vec<u8>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Config {
    language: String,
    logo: LogoConfig,
    disabled_entries: Vec<String>,
}

fn main() {
    let afetch_config_path: std::path::PathBuf = dirs::config_dir().unwrap_or_else(|| {
        println!("An error occurred while retrieving the configuration files folder, please open an issue at: https://github.com/Asthowen/AFetch/issues/new so that we can solve your issue.");
        exit(9);
    }).join("afetch").join("config.yaml");

    if !afetch_config_path.parent().unwrap().exists() {
        std::fs::create_dir_all(&afetch_config_path).unwrap_or_else(|e| {
            println!(
                "An error occurred while creating the configuration files: {}",
                e.to_string()
            );
            exit(9);
        });
    }

    let yaml_to_parse: String = if afetch_config_path.exists() {
        std::fs::read_to_string(afetch_config_path).unwrap()
    } else {
        let to_write: String = "language: auto # en / fr / auto \nlogo:\n  status: enable # disable / enable\n  char_type: braille # braille / picture\n  picture_path: none # `the file path: eg: ~/pictures/some.png` / none\n  color:\n    - 255\n    - 255\n    - 255\n  color_header:\n    - 133\n    - 218\n    - 249\ndisabled_entries:\n  - public-ip\n  - battery".to_owned();
        std::fs::write(afetch_config_path, to_write.clone()).unwrap();
        to_write
    };
    let yaml: Config = serde_yaml::from_str(&yaml_to_parse).unwrap_or_else(|e| {
        println!("Your configuration is malformed ({})", e);
        exit(9);
    });

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

    let infos: Infos = Infos::init();
    let logo_type: i8 = if yaml.logo.status == "enable" {
        if yaml.logo.char_type == "braille" {
            0
        } else {
            1
        }
    } else {
        2
    };
    let logo: Option<Vec<&str>> = if logo_type == 0 {
        Option::from(infos.get_os_logo().lines().collect::<Vec<&str>>())
    } else {
        None
    };

    let (username, host) = (username(), hostname());
    let mut infos_to_print: Vec<String> = Vec::new();
    let mut output: String = "".to_owned();
    infos_to_print.push(format!(
        "{}{}{}",
        username
            .truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2],
            )
            .bold(),
        "@".truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2]),
        host.truecolor(
            yaml.logo.color_header[0],
            yaml.logo.color_header[1],
            yaml.logo.color_header[2],
        )
        .bold()
    ));
    infos_to_print.push(format!(
        "{}",
        "─".repeat(username.len() + host.len() + 1).truecolor(
            yaml.logo.color[0],
            yaml.logo.color[1],
            yaml.logo.color[2]
        )
    ));

    if !yaml.disabled_entries.contains(&"os".to_owned()) {
        let system_name: String = infos.sysinfo_obj.name().unwrap_or_else(|| "".to_owned());
        if !system_name.is_empty() {
            if system_name.to_lowercase().contains("windows") {
                infos_to_print.push(
                    format!(
                        "{}{} {}",
                        language["label-os"].bold().truecolor(
                            yaml.logo.color_header[0],
                            yaml.logo.color_header[1],
                            yaml.logo.color_header[2]
                        ),
                        system_name,
                        infos
                            .sysinfo_obj
                            .os_version()
                            .unwrap()
                            .split(' ')
                            .collect::<Vec<&str>>()[0]
                    )
                    .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
                    .to_string(),
                );
            } else {
                infos_to_print.push(
                    format!(
                        "{}{}",
                        language["label-os"].bold().truecolor(
                            yaml.logo.color_header[0],
                            yaml.logo.color_header[1],
                            yaml.logo.color_header[2]
                        ),
                        system_name
                    )
                    .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
                    .to_string(),
                );
            }
        }
    }
    if !yaml.disabled_entries.contains(&"host".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-host"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            infos
                .get_host()
                .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
        ));
    }
    if !yaml.disabled_entries.contains(&"kernel".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-kernel"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            infos
                .sysinfo_obj
                .kernel_version()
                .unwrap_or_else(|| "".to_owned())
                .replace('\n', "")
                .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
        ));
    }
    if !yaml.disabled_entries.contains(&"uptime".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-uptime"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            utils::format_time(infos.sysinfo_obj.uptime()).truecolor(
                yaml.logo.color[0],
                yaml.logo.color[1],
                yaml.logo.color[2]
            )
        ));
    }
    if !yaml.disabled_entries.contains(&"packages".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-packages"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2],
            ),
            infos.get_packages_number().truecolor(
                yaml.logo.color[0],
                yaml.logo.color[1],
                yaml.logo.color[2]
            )
        ));
    }
    if !yaml.disabled_entries.contains(&"resolution".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-resolution"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2],
            ),
            infos.get_screens_resolution().truecolor(
                yaml.logo.color[0],
                yaml.logo.color[1],
                yaml.logo.color[2]
            )
        ));
    }
    if !yaml.disabled_entries.contains(&"desktop".to_owned()) {
        let de_infos: (String, String) = infos.get_de();
        infos_to_print.push(format!(
            "{}{}",
            language["label-desktop"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2],
            ),
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
            .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
        ));
    }
    if !yaml.disabled_entries.contains(&"shell".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-shell"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            infos
                .get_shell()
                .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
        ));
    }
    if !yaml.disabled_entries.contains(&"terminal".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-terminal"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            infos.get_terminal().truecolor(
                yaml.logo.color[0],
                yaml.logo.color[1],
                yaml.logo.color[2]
            )
        ));
    }
    if !yaml.disabled_entries.contains(&"memory".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-memory"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            format!(
                "{}/{}",
                convert_to_readable_unity(infos.sysinfo_obj.used_memory() as f64),
                convert_to_readable_unity(infos.sysinfo_obj.total_memory() as f64)
            )
            .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
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
                language["label-cpu"].bold().truecolor(
                    yaml.logo.color_header[0],
                    yaml.logo.color_header[1],
                    yaml.logo.color_header[2]
                ),
                cpu_infos.cpu_usage().to_string().truecolor(
                    yaml.logo.color[0],
                    yaml.logo.color[1],
                    yaml.logo.color[2]
                )
            ));
        } else {
            infos_to_print.push(format!(
                "{}{}",
                language["label-cpu"].bold().truecolor(
                    yaml.logo.color_header[0],
                    yaml.logo.color_header[1],
                    yaml.logo.color_header[2]
                ),
                format!("{} - {:.1}%", cpu_name, cpu_infos.cpu_usage()).truecolor(
                    yaml.logo.color[0],
                    yaml.logo.color[1],
                    yaml.logo.color[2]
                )
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
            language["label-network"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            format!(
                "{}/s ↘  {}/s ↗",
                convert_to_readable_unity(network_sent as f64),
                convert_to_readable_unity(network_recv as f64)
            )
            .truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2])
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
                        language["label-disk"].bold().truecolor(
                            yaml.logo.color_header[0],
                            yaml.logo.color_header[1],
                            yaml.logo.color_header[2]
                        ),
                        format!("({})", disk.mount_point().to_str().unwrap_or(""),).truecolor(
                            yaml.logo.color_header[0],
                            yaml.logo.color_header[1],
                            yaml.logo.color_header[2]
                        ),
                        format!(
                            "{}{}/{}",
                            language["label-disk-1"],
                            convert_to_readable_unity(
                                (disk.total_space() - disk.available_space()) as f64
                            ),
                            convert_to_readable_unity(disk.total_space() as f64)
                        )
                        .truecolor(
                            yaml.logo.color[0],
                            yaml.logo.color[1],
                            yaml.logo.color[2]
                        )
                    ));
                }
            }
        }
        if print_disks {
            infos_to_print.push(format!(
                "{}{}{}{}",
                language["label-disks"].bold().truecolor(
                    yaml.logo.color_header[0],
                    yaml.logo.color_header[1],
                    yaml.logo.color_header[2]
                ),
                convert_to_readable_unity(total_disk_used as f64).truecolor(
                    yaml.logo.color[0],
                    yaml.logo.color[1],
                    yaml.logo.color[2]
                ),
                "/".truecolor(yaml.logo.color[0], yaml.logo.color[1], yaml.logo.color[2]),
                convert_to_readable_unity(total_disk_total as f64).truecolor(
                    yaml.logo.color[0],
                    yaml.logo.color[1],
                    yaml.logo.color[2]
                )
            ));
        }
    }
    if !yaml.disabled_entries.contains(&"public-ip".to_owned()) {
        infos_to_print.push(format!(
            "{}{}",
            language["label-public-ip"].bold().truecolor(
                yaml.logo.color_header[0],
                yaml.logo.color_header[1],
                yaml.logo.color_header[2]
            ),
            infos.get_public_ip().truecolor(
                yaml.logo.color[0],
                yaml.logo.color[1],
                yaml.logo.color[2]
            )
        ));
    }

    if !yaml.disabled_entries.contains(&"battery".to_owned()) {
        let manager_result = battery::Manager::new();
        if let Ok(manager) = manager_result {
            let batteries_infos_result = manager.batteries();
            if let Ok(mut batteries_infos) = batteries_infos_result {
                if let Some(Ok(battery_infos)) = batteries_infos.next() {
                    infos_to_print.push(format!(
                        "{}{}%",
                        language["label-battery"].bold().truecolor(
                            yaml.logo.color_header[0],
                            yaml.logo.color_header[1],
                            yaml.logo.color_header[2]
                        ),
                        battery_infos.state_of_charge().value.to_string().truecolor(
                            yaml.logo.color[0],
                            yaml.logo.color[1],
                            yaml.logo.color[2]
                        )
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
        let mut last_index = 0;

        for (i, info) in infos_to_print.into_iter().enumerate() {
            if logo.len() > i {
                output += &format!("{}   {}\n", logo[i], info);
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
