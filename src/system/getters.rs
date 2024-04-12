use crate::config::Config;
use crate::system::infos::Infos;
use crate::utils;
use crate::utils::convert_to_readable_unity;
use afetch_colored::CustomColor;
use afetch_colored::{AnsiOrCustom, Colorize};
use std::collections::HashMap;
use std::sync::Arc;
use sysinfo::{Cpu, Disks, Networks};

pub async fn get_os(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"os".to_owned()) {
        return None;
    }

    let system_name: String = sysinfo::System::name()
        .unwrap_or_default()
        .trim()
        .to_owned();
    if system_name.is_empty() {
        return None;
    }

    if system_name.to_lowercase().contains("windows") {
        Some(format!(
            "{}{}",
            language["label-os"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            format!(
                "{} {}",
                system_name,
                sysinfo::System::os_version()
                    .unwrap_or_default()
                    .split(' ')
                    .collect::<Vec<&str>>()[0]
            )
            .custom_color(*logo_color)
        ))
    } else {
        Some(format!(
            "{}{}",
            language["label-os"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            system_name.custom_color(*logo_color)
        ))
    }
}
pub async fn get_host(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"host".to_owned()) {
        return None;
    }

    match infos.get_host().as_str() {
        "" => None,
        host => Some(format!(
            "{}{}",
            language["label-host"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            host.custom_color(*logo_color)
        )),
    }
}

pub async fn get_kernel(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"kernel".to_owned()) {
        return None;
    }

    let kernel_version: String = sysinfo::System::kernel_version()
        .unwrap_or_default()
        .trim()
        .replace('\n', "");
    match kernel_version.as_str() {
        "" => None,
        kernel_version => Some(format!(
            "{}{}",
            language["label-kernel"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            kernel_version.custom_color(*logo_color)
        )),
    }
}

pub async fn get_uptime(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"uptime".to_owned()) {
        return None;
    }

    match utils::format_time(sysinfo::System::uptime(), &language).as_str() {
        "" => None,
        uptime => Some(format!(
            "{}{}",
            language["label-uptime"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            uptime.custom_color(*logo_color)
        )),
    }
}

pub async fn get_packages(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"packages".to_owned()) {
        return None;
    }

    match infos.get_packages_number().await.as_str() {
        "" => None,
        packages_number => Some(format!(
            "{}{}",
            language["label-packages"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            packages_number.custom_color(*logo_color)
        )),
    }
}

pub async fn get_resolution(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"resolution".to_owned()) {
        return None;
    }

    match infos.get_screens_resolution().as_str() {
        "" => None,
        screens_resolution => Some(format!(
            "{}{}",
            language["label-resolution"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            screens_resolution.custom_color(*logo_color)
        )),
    }
}

pub async fn get_desktop(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"desktop".to_owned()) {
        return None;
    }

    let (de_name, mut de_version): (String, String) = infos.get_de();
    if de_name.is_empty() {
        return None;
    }

    if yaml
        .disabled_entries
        .contains(&"desktop-version".to_owned())
    {
        de_version = String::default();
    }

    Some(format!(
        "{}{}",
        language["label-desktop"]
            .bold()
            .custom_color_or_ansi_color_code(*header_color),
        format!("{} {}", de_name, de_version).custom_color(*logo_color)
    ))
}

pub async fn get_shell(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"shell".to_owned()) {
        return None;
    }

    match infos.get_shell().as_str() {
        "" => None,
        shell => Some(format!(
            "{}{}",
            language["label-shell"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            shell.custom_color(*logo_color)
        )),
    }
}

pub async fn get_terminal(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"terminal".to_owned()) {
        return None;
    }

    match infos.get_terminal().as_str() {
        "" => None,
        terminal => Some(format!(
            "{}{}",
            language["label-terminal"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            terminal.custom_color(*logo_color)
        )),
    }
}

pub async fn get_terminal_font(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"terminal-font".to_owned()) {
        return None;
    }

    match infos.get_terminal_font().as_str() {
        "" => None,
        terminal_font => Some(format!(
            "{}{}",
            language["label-terminal-font"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            terminal_font.custom_color(*logo_color)
        )),
    }
}

pub async fn get_memory(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"memory".to_owned()) {
        return None;
    }

    Some(format!(
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
    ))
}

pub async fn get_cpu(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"cpu".to_owned()) {
        return None;
    }

    let cpu_infos: &Cpu = match infos.sysinfo_obj.cpus().first() {
        None => return None,
        Some(cpu) => cpu,
    };

    let cpu_name: String = if !cpu_infos.brand().is_empty() {
        cpu_infos.brand().to_owned()
    } else if !infos.sysinfo_obj.global_cpu_info().vendor_id().is_empty() {
        cpu_infos.vendor_id().to_owned()
    } else {
        return None;
    };

    let cpu_percentage: String = if yaml.disabled_entries.contains(&"cpu-usage".to_owned()) {
        String::default()
    } else {
        format!(" - {:.1}%", cpu_infos.cpu_usage())
    };

    Some(format!(
        "{}{}",
        language["label-cpu"]
            .bold()
            .custom_color_or_ansi_color_code(*header_color),
        format!("{}{}", cpu_name, cpu_percentage).custom_color(*logo_color)
    ))
}

pub async fn get_gpus(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<Vec<String>> {
    if yaml.disabled_entries.contains(&"gpu".to_owned()) {
        return None;
    }

    let gpus_list: Vec<String> = infos.get_gpus();
    if gpus_list.is_empty() {
        return None;
    }

    let gpus: Vec<String> = gpus_list
        .iter()
        .map(|gpu| {
            format!(
                "{}{}",
                language["label-gpu"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                gpu.custom_color(*logo_color)
            )
        })
        .collect();
    Some(gpus)
}

pub async fn get_network(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"network".to_owned()) {
        return None;
    }

    let (mut network_sent, mut network_recv) = (0, 0);
    for data in Networks::new_with_refreshed_list().list().values() {
        network_sent += data.transmitted();
        network_recv += data.received();
    }
    Some(format!(
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
    ))
}

pub async fn get_disks(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<Vec<String>> {
    let print_disk: bool = !yaml.disabled_entries.contains(&"disk".to_owned());
    let print_disks: bool = !yaml.disabled_entries.contains(&"disks".to_owned());
    if !print_disk && print_disks {
        return None;
    }

    let mut disks: Vec<String> = Vec::new();
    let (mut total_disk_used, mut total_disk_total) = (0, 0);

    for disk in Disks::new_with_refreshed_list().list() {
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
                    language["label-disk-1"].custom_color_or_ansi_color_code(*header_color),
                    convert_to_readable_unity((disk.total_space() - disk.available_space()) as f64),
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

    if disks.is_empty() {
        None
    } else {
        Some(disks)
    }
}

pub async fn get_public_ip(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"public-ip".to_owned()) {
        return None;
    }

    match infos.get_public_ip().as_str() {
        "" => None,
        get_ip => Some(format!(
            "{}{}",
            language["label-public-ip"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            get_ip.custom_color(*logo_color)
        )),
    }
}

pub async fn get_wm(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    infos: Arc<Infos>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"wm".to_owned()) {
        return None;
    }

    match infos.get_wm().as_str() {
        "" => None,
        wm => Some(format!(
            "{}{}",
            language["label-wm"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            wm.custom_color(*logo_color)
        )),
    }
}

pub async fn get_battery(
    yaml: Arc<Config>,
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Option<String> {
    if yaml.disabled_entries.contains(&"battery".to_owned()) {
        return None;
    }

    if let Ok(Some(battery_infos)) = starship_battery::Manager::new()
        .and_then(|manager| manager.batteries())
        .and_then(|mut batteries_infos| batteries_infos.next().transpose())
    {
        let battery_value: String = (battery_infos.state_of_charge().value * 100.0).to_string();
        if battery_value.is_empty() {
            return None;
        }

        return Some(format!(
            "{}{:.4}%",
            language["label-battery"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            battery_value.custom_color(*logo_color)
        ));
    }

    None
}
