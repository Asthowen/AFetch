use crate::config::{DesktopEnvironment, Shell};
use crate::error::FetchInfosError;
use crate::system::futures::FutureResultType;
use crate::utils::convert_to_readable_unity;
use crate::{config, utils};
use afetch_colored::CustomColor;
use afetch_colored::{AnsiOrCustom, Colorize};
use std::collections::HashMap;
use std::fmt::Write;
use std::process::exit;
use std::sync::Arc;
use sysinfo::{Cpu, CpuRefreshKind, Disks, MemoryRefreshKind, Networks, RefreshKind, System};

pub async fn get_os(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let system_name: String = System::name().unwrap_or_default().trim().to_owned();
    if system_name.is_empty() {
        return Ok(None);
    }

    if system_name.to_lowercase().contains("windows") {
        Ok(Some(FutureResultType::String(format!(
            "{}{}",
            language["label-os"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            format!(
                "{} {}",
                system_name,
                System::os_version()
                    .unwrap_or_default()
                    .split(' ')
                    .collect::<Vec<&str>>()[0]
            )
            .custom_color(*logo_color)
        ))))
    } else {
        Ok(Some(FutureResultType::String(format!(
            "{}{}",
            language["label-os"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            system_name.custom_color(*logo_color)
        ))))
    }
}
pub async fn get_host(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::host::get_host().await?.map(|host| {
        FutureResultType::String(format!(
            "{}{}",
            language["label-host"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            host.custom_color(*logo_color)
        ))
    }))
}

pub async fn get_kernel(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let kernel_version: String = System::kernel_version()
        .unwrap_or_default()
        .trim()
        .replace('\n', "");
    match kernel_version.as_str() {
        "" => Ok(None),
        kernel_version => Ok(Some(FutureResultType::String(format!(
            "{}{}",
            language["label-kernel"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            kernel_version.custom_color(*logo_color)
        )))),
    }
}

pub async fn get_uptime(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    match utils::format_time(System::uptime(), &language).as_str() {
        "" => Ok(None),
        uptime => Ok(Some(FutureResultType::String(format!(
            "{}{}",
            language["label-uptime"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            uptime.custom_color(*logo_color)
        )))),
    }
}

pub async fn get_packages(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::packages::get_packages_infos()
        .await?
        .map(|packages| {
            FutureResultType::String(format!(
                "{}{}",
                language["label-packages"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                packages.custom_color(*logo_color)
            ))
        }))
}

pub async fn get_resolution(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::resolutions::get_resolutions()
        .await?
        .map(|resolutions| {
            FutureResultType::String(format!(
                "{}{}",
                language["label-resolution"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                resolutions.custom_color(*logo_color)
            ))
        }))
}

pub async fn get_desktop(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    config: DesktopEnvironment,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(
        crate::system::infos::desktop_environment::get_desktop_environment(config)
            .await?
            .map(|(name, version)| {
                FutureResultType::String(format!(
                    "{}{}",
                    language["label-desktop"]
                        .bold()
                        .custom_color_or_ansi_color_code(*header_color),
                    format!("{} {}", name, version.unwrap_or_default()).custom_color(*logo_color)
                ))
            }),
    )
}

pub async fn get_shell(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    config: Shell,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::shell::get_shell(config)
        .await?
        .map(|(shell, version)| {
            FutureResultType::String(format!(
                "{}{} {}",
                language["label-shell"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                shell.custom_color(*logo_color),
                version.unwrap_or_default()
            ))
        }))
}

pub async fn get_terminal(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::terminal::get_terminal()
        .await?
        .map(|terminal| {
            FutureResultType::String(format!(
                "{}{}",
                language["label-terminal"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                terminal.custom_color(*logo_color)
            ))
        }))
}

pub async fn get_terminal_font(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::terminal_font::get_terminal_font()
        .await?
        .map(|terminal_font| {
            FutureResultType::String(format!(
                "{}{}",
                language["label-terminal-font"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                terminal_font.custom_color(*logo_color)
            ))
        }))
}

pub async fn get_memory(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let infos: System = System::new_with_specifics(
        RefreshKind::new().with_memory(MemoryRefreshKind::new().with_ram()),
    );
    Ok(Some(FutureResultType::String(format!(
        "{}{}",
        language["label-memory"]
            .bold()
            .custom_color_or_ansi_color_code(*header_color),
        format!(
            "{}/{}",
            convert_to_readable_unity(infos.used_memory() as f64),
            convert_to_readable_unity(infos.total_memory() as f64)
        )
        .custom_color(*logo_color)
    ))))
}

pub async fn get_cpu(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    config: config::Cpu,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let mut infos: System =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));
    if config.percentage {
        tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
        infos.refresh_cpu_usage();
    }

    let cpu_infos: &Cpu = match infos.cpus().first() {
        None => return Ok(None),
        Some(cpu) => cpu,
    };

    let cpu_name: String = if !cpu_infos.brand().is_empty() {
        cpu_infos.brand().to_owned()
    } else if !infos.global_cpu_info().vendor_id().is_empty() {
        cpu_infos.vendor_id().to_owned()
    } else {
        return Ok(None);
    };

    let cpu_percentage: String = if config.percentage {
        format!(" - {:.1}%", cpu_infos.cpu_usage())
    } else {
        String::default()
    };

    Ok(Some(FutureResultType::String(format!(
        "{}{}",
        language["label-cpu"]
            .bold()
            .custom_color_or_ansi_color_code(*header_color),
        format!("{}{}", cpu_name, cpu_percentage).custom_color(*logo_color)
    ))))
}

pub async fn get_gpus(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let gpus_list_opt = crate::system::infos::gpus::get_gpus()?;
    let gpus_list = match gpus_list_opt {
        None => return Ok(None),
        Some(gpus_list) => gpus_list,
    };

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
    Ok(Some(FutureResultType::List(gpus)))
}

pub async fn get_network(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let (mut network_sent, mut network_recv) = (0, 0);
    for data in Networks::new_with_refreshed_list().list().values() {
        network_sent += data.transmitted();
        network_recv += data.received();
    }
    Ok(Some(FutureResultType::String(format!(
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
    ))))
}

fn should_exclude_disk(disk_mount_point: &str, exclude: &Option<Vec<String>>) -> bool {
    disk_mount_point.contains("/etc")
        || disk_mount_point.contains("/boot")
        || disk_mount_point.contains("/snapd")
        || disk_mount_point.contains("/docker")
        || exclude.as_ref().map_or(false, |exclude| {
            exclude.contains(&disk_mount_point.to_owned())
        })
}

pub async fn get_disks(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
    config_disks: Option<config::Disks>,
    config_disk: Option<config::Disk>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    let mut disks: Vec<String> = Vec::new();
    let (mut total_disk_used, mut total_disk_total) = (0, 0);

    let hide_individual_disks: bool = config_disk.is_none();
    let show_disks_total: bool = config_disks.is_some();
    let to_exclude = if let Some(config) = config_disks {
        config.exclude
    } else if let Some(config) = config_disk {
        config.hide
    } else {
        None
    };

    for disk in Disks::new_with_refreshed_list().list() {
        let disk_mount_point: String = disk.mount_point().to_str().unwrap().to_owned();

        if should_exclude_disk(&disk_mount_point, &to_exclude) {
            continue;
        }

        total_disk_used += disk.total_space() - disk.available_space();
        total_disk_total += disk.total_space();

        if hide_individual_disks {
            continue;
        }

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
    if show_disks_total {
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
        Ok(None)
    } else {
        Ok(Some(FutureResultType::List(disks)))
    }
}

pub async fn get_public_ip(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::ip::get_public_ip()?.map(|ip| {
        FutureResultType::String(format!(
            "{}{}",
            language["label-public-ip"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            ip.custom_color(*logo_color)
        ))
    }))
}

pub async fn get_window_manager(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(crate::system::infos::window_manager::get_window_manager()
        .await?
        .map(|wm| {
            FutureResultType::String(format!(
                "{}{}",
                language["label-wm"]
                    .bold()
                    .custom_color_or_ansi_color_code(*header_color),
                wm.custom_color(*logo_color)
            ))
        }))
}

pub async fn get_battery(
    header_color: Arc<AnsiOrCustom>,
    logo_color: Arc<CustomColor>,
    language: Arc<HashMap<&'static str, &'static str>>,
) -> Result<Option<FutureResultType>, FetchInfosError> {
    if let Ok(Some(battery_infos)) = starship_battery::Manager::new()
        .and_then(|manager| manager.batteries())
        .and_then(|mut batteries_infos| batteries_infos.next().transpose())
    {
        let battery_value: String = (battery_infos.state_of_charge().value * 100.0).to_string();
        if battery_value.is_empty() {
            return Ok(None);
        }

        return Ok(Some(FutureResultType::String(format!(
            "{}{:.4}%",
            language["label-battery"]
                .bold()
                .custom_color_or_ansi_color_code(*header_color),
            battery_value.custom_color(*logo_color)
        ))));
    }

    Ok(None)
}
pub async fn get_color_blocks() -> Result<Option<FutureResultType>, FetchInfosError> {
    let first_colors: String = (0..8).fold(String::default(), |mut acc, i| {
        write!(&mut acc, "\x1b[4{}m   \x1b[0m", i).unwrap_or_else(|error| {
            println!("Failed to write to string for color blocks: {}.", error);
            exit(9);
        });
        acc
    });

    let second_colors: String = (0..8).fold(String::default(), |mut acc, i| {
        write!(&mut acc, "\x1b[10{}m   \x1b[0m", i).unwrap_or_else(|error| {
            println!("Failed to write to string for color blocks: {}.", error);
            exit(9);
        });
        acc
    });
    Ok(Some(FutureResultType::List(vec![
        first_colors,
        second_colors,
    ])))
}

pub async fn get_empty_line() -> Result<Option<FutureResultType>, FetchInfosError> {
    Ok(Some(FutureResultType::String(String::default())))
}
