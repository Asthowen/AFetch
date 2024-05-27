use crate::config::{Config, Entry};
use crate::error::FetchInfosError;
use afetch_colored::{AnsiOrCustom, CustomColor};
use dbus::nonblock::SyncConnection;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task;

use crate::system::getters::{
    get_battery, get_color_blocks, get_cpu, get_desktop, get_disks, get_empty_line, get_gpus,
    get_host, get_kernel, get_memory, get_network, get_os, get_packages, get_public_ip,
    get_resolution, get_shell, get_terminal, get_terminal_font, get_uptime, get_window_manager,
};

type ResultFuture = task::JoinHandle<Result<Option<FutureResultType>, FetchInfosError>>;

pub enum FutureResultType {
    String(String),
    List(Vec<String>),
}

pub fn create_futures(
    shared_yaml: Arc<Config>,
    shared_header_color: Arc<AnsiOrCustom>,
    shared_logo_color: Arc<CustomColor>,
    shared_language: Arc<HashMap<&'static str, &'static str>>,
    conn: Arc<SyncConnection>,
) -> Vec<ResultFuture> {
    let mut futures: Vec<ResultFuture> = Vec::new();

    for entry in shared_yaml.entries.clone() {
        match entry {
            Entry::Cpu(config) => {
                futures.push(task::spawn(get_cpu(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    config,
                )));
            }
            Entry::Battery => {
                futures.push(task::spawn(get_battery(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::OS => {
                futures.push(task::spawn(get_os(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Host => {
                futures.push(task::spawn(get_host(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Kernel => {
                futures.push(task::spawn(get_kernel(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Uptime => {
                futures.push(task::spawn(get_uptime(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Packages => {
                futures.push(task::spawn(get_packages(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Shell(config) => {
                futures.push(task::spawn(get_shell(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    config,
                )));
            }
            Entry::Resolution => {
                futures.push(task::spawn(get_resolution(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::DesktopEnvironment(config) => {
                futures.push(task::spawn(get_desktop(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    config,
                    conn.clone(),
                )));
            }
            Entry::WindowManager => {
                futures.push(task::spawn(get_window_manager(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Terminal => {
                futures.push(task::spawn(get_terminal(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::TerminalFont => {
                futures.push(task::spawn(get_terminal_font(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    conn.clone(),
                )));
            }
            Entry::GPUS => {
                futures.push(task::spawn(get_gpus(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Memory => {
                futures.push(task::spawn(get_memory(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Network => {
                futures.push(task::spawn(get_network(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::Disk(config) => {
                futures.push(task::spawn(get_disks(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    None,
                    Some(config),
                )));
            }
            Entry::Disks(config) => {
                futures.push(task::spawn(get_disks(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                    Some(config),
                    None,
                )));
            }
            Entry::PublicIP => {
                futures.push(task::spawn(get_public_ip(
                    shared_header_color.clone(),
                    shared_logo_color.clone(),
                    shared_language.clone(),
                )));
            }
            Entry::ColorBlocks => {
                futures.push(task::spawn(get_color_blocks()));
            }
            Entry::EmptyLine => {
                futures.push(task::spawn(get_empty_line()));
            }
        }
    }

    futures
}
