use crate::config::Config;
use crate::system::infos::Infos;
use crate::utils;
use crate::utils::convert_to_readable_unity;
use afetch_colored::CustomColor;
use afetch_colored::{AnsiOrCustom, Colorize};
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::{Cpu, CpuExt, DiskExt, NetworkExt, SystemExt};

pub async fn get_os(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"os".to_owned()) {
        let system_name: String = infos
            .sysinfo_obj
            .name()
            .unwrap_or_default()
            .trim()
            .to_owned();
        if !system_name.is_empty() {
            return if system_name.to_lowercase().contains("windows") {
                Option::from(format!(
                    "{}{}",
                    language["label-os"]
                        .bold()
                        .custom_color_or_ansi_color_code(*header_color),
                    format!(
                        "{} {}",
                        system_name,
                        infos
                            .sysinfo_obj
                            .os_version()
                            .unwrap_or_default()
                            .split(' ')
                            .collect::<Vec<&str>>()[0]
                    )
                    .custom_color(*logo_color)
                ))
            } else {
                Option::from(format!(
                    "{}{}",
                    language["label-os"]
                        .bold()
                        .custom_color_or_ansi_color_code(*header_color),
                    system_name.custom_color(*logo_color)
                ))
            };
        }
    }
    None
}
pub async fn get_host(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"host".to_owned()) {
        let host: String = infos.get_host();
        if !host.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-host"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                host.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_kernel(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"kernel".to_owned()) {
        let kernel_version: String = infos
            .sysinfo_obj
            .kernel_version()
            .unwrap_or_default()
            .trim()
            .replace('\n', "");
        if !kernel_version.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-kernel"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                kernel_version.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_uptime(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"uptime".to_owned()) {
        let uptime: String = utils::format_time(infos.sysinfo_obj.uptime(), &language);
        if !uptime.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-uptime"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                uptime.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_packages(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"packages".to_owned()) {
        let packages: String = infos.get_packages_number();
        if !packages.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-packages"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                packages.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_resolution(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"resolution".to_owned()) {
        let screens_resolution = infos.get_screens_resolution();
        if !screens_resolution.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-resolution"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                screens_resolution.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_desktop(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"desktop".to_owned()) {
        let de_infos: (String, String) = infos.get_de();
        if !de_infos.0.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-desktop"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                format!(
                    "{} {}",
                    de_infos.0,
                    if !yaml
                        .disabled_entries
                        .contains(&"desktop-version".to_owned())
                        && !de_infos.0.is_empty()
                    {
                        de_infos.1
                    } else {
                        String::default()
                    }
                )
                .custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_shell(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"shell".to_owned()) {
        let shell: String = infos.get_shell();
        if !shell.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-shell"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                shell.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_terminal(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"terminal".to_owned()) {
        let terminal: String = infos.get_terminal();
        if !terminal.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-terminal"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                terminal.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_terminal_font(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"terminal_font".to_owned()) {
        let terminal: String = infos.get_terminal_font();
        if !terminal.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-terminal-font"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                terminal.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_memory(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"memory".to_owned()) {
        return Option::from(format!(
            "{}{}",
            language["label-memory"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            format!(
                "{}/{}",
                convert_to_readable_unity(infos.sysinfo_obj.used_memory() as f64),
                convert_to_readable_unity(infos.sysinfo_obj.total_memory() as f64)
            )
            .custom_color(*logo_color)
        ));
    }
    None
}

pub async fn get_cpu(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"cpu".to_owned()) {
        let cpu_infos: &Cpu = infos.sysinfo_obj.global_cpu_info();
        let cpu_name: String = if !cpu_infos.brand().is_empty() {
            cpu_infos.brand().to_owned()
        } else if !infos.sysinfo_obj.global_cpu_info().vendor_id().is_empty() {
            cpu_infos.vendor_id().to_owned()
        } else {
            String::default()
        };
        if !cpu_name.is_empty() {
            let cpu_percentage: String = if yaml.disabled_entries.contains(&"cpu-usage".to_owned())
            {
                String::default()
            } else {
                format!(" - {:.1}%", cpu_infos.cpu_usage())
            };

            return Option::from(format!(
                "{}{}",
                language["label-cpu"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                format!("{}{}", cpu_name, cpu_percentage).custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_gpus(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<Vec<String>> {
    if !yaml.disabled_entries.contains(&"gpu".to_owned()) {
        let gpu: Vec<String> = infos.get_gpu();
        if !gpu.is_empty() {
            let mut gpus: Vec<String> = Vec::new();
            for gpu in gpu {
                gpus.push(format!(
                    "{}{}",
                    language["label-gpu"]
                        .bold()
                        .custom_color_or_ansi_color_code(*header_color),
                    gpu.custom_color(*logo_color)
                ));
            }
            return Some(gpus);
        }
    }
    None
}

pub async fn get_network(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"network".to_owned()) {
        let (mut network_sent, mut network_recv) = (0, 0);
        for (_, data) in infos.sysinfo_obj.networks() {
            network_sent += data.transmitted();
            network_recv += data.received();
        }
        return Option::from(format!(
            "{}{}",
            language["label-network"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            format!(
                "{}/s ↘  {}/s ↗",
                convert_to_readable_unity(network_sent as f64),
                convert_to_readable_unity(network_recv as f64)
            )
            .custom_color(*logo_color)
        ));
    }
    None
}

pub async fn get_disks(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<Vec<String>> {
    let print_disk: bool = !yaml.disabled_entries.contains(&"disk".to_owned());
    let print_disks: bool = !yaml.disabled_entries.contains(&"disks".to_owned());
    let mut disks: Vec<String> = Vec::new();

    if print_disks || print_disk {
        let (mut total_disk_used, mut total_disk_total) = (0, 0);
        for disk in infos.sysinfo_obj.disks() {
            let disk_mount_point: String = disk.mount_point().to_str().unwrap().to_owned();
            if disk_mount_point.contains("/etc")
                || disk_mount_point.contains("/boot")
                || disk_mount_point.contains("/snapd")
                || disk_mount_point.contains("/docker")
            {
                continue;
            }

            total_disk_used += disk.total_space() - disk.available_space();
            total_disk_total += disk.total_space();

            if print_disk {
                disks.push(format!(
                    "{}{}{}",
                    language["label-disk"]
                        .bold()
                        .custom_color_or_ansi_color_code(*header_color),
                    format!("({})", disk.mount_point().to_str().unwrap_or(""),)
                        .custom_color_or_ansi_color_code(*header_color),
                    format!(
                        "{}{}/{}",
                        language["label-disk-1"],
                        convert_to_readable_unity(
                            (disk.total_space() - disk.available_space()) as f64
                        ),
                        convert_to_readable_unity(disk.total_space() as f64)
                    )
                    .custom_color(*logo_color)
                ));
            }
        }
        if print_disks {
            disks.push(format!(
                "{}{}",
                language["label-disks"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                format!(
                    "{}/{}",
                    convert_to_readable_unity(total_disk_used as f64),
                    convert_to_readable_unity(total_disk_total as f64)
                )
                .custom_color(*logo_color)
            ));
        }
    }
    if disks.is_empty() {
        None
    } else {
        Option::from(disks)
    }
}

pub async fn get_public_ip(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"public-ip".to_owned()) {
        let get_ip: String = infos.get_public_ip();
        if !get_ip.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-public-ip"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                infos.get_public_ip().custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_wm(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"wm".to_owned()) {
        let wm: String = infos.get_wm();
        if !wm.is_empty() {
            return Option::from(format!(
                "{}{}",
                language["label-wm"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                wm.custom_color(*logo_color)
            ));
        }
    }
    None
}

pub async fn get_battery(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if !yaml.disabled_entries.contains(&"battery".to_owned()) {
        let manager_result = starship_battery::Manager::new();
        if let Ok(manager) = manager_result {
            let batteries_infos_result = manager.batteries();
            if let Ok(mut batteries_infos) = batteries_infos_result {
                if let Some(Ok(battery_infos)) = batteries_infos.next() {
                    let battery_value: String =
                        (battery_infos.state_of_charge().value * 100.0).to_string();
                    if !battery_value.is_empty() {
                        return Option::from(format!(
                            "{}{:.4}%",
                            language["label-battery"]
                                .bold()
                                .custom_color_or_ansi_color_code(*header_color),
                            battery_value.custom_color(*logo_color)
                        ));
                    }
                }
            }
        }
    }
    None
}
