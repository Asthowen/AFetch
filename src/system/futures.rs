use crate::config::{Config, Entry};
use crate::error::FetchInfosError;
use crate::system::getters::{
    get_battery, get_color_blocks, get_cpu, get_desktop, get_disks, get_empty_line, get_gpus,
    get_host, get_kernel, get_memory, get_network, get_os, get_packages, get_public_ip,
    get_resolution, get_shell, get_terminal, get_terminal_font, get_uptime, get_window_manager,
};
use afetch_colored::{AnsiOrCustom, CustomColor};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::task::JoinSet;

type ResultFuture = Result<Option<FutureResultType>, FetchInfosError>;

pub enum FutureResultType {
    String(String),
    List(Vec<String>),
}

pub fn create_futures(
    shared_yaml: Arc<Config>,
    shared_header_color: Arc<AnsiOrCustom>,
    shared_logo_color: Arc<CustomColor>,
    shared_language: Arc<HashMap<&'static str, &'static str>>,
) -> JoinSet<ResultFuture> {
    let mut futures: JoinSet<ResultFuture> = JoinSet::new();

    for entry in &shared_yaml.entries {
        match entry {
            Entry::Cpu(config) => {
                futures.spawn(get_cpu(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                    config.clone(),
                ));
            }
            Entry::Battery => {
                futures.spawn(get_battery(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::OS => {
                futures.spawn(get_os(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Host => {
                futures.spawn(get_host(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Kernel => {
                futures.spawn(get_kernel(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Uptime => {
                futures.spawn(get_uptime(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Packages => {
                futures.spawn(get_packages(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Shell(config) => {
                futures.spawn(get_shell(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                    config.clone(),
                ));
            }
            Entry::Resolution => {
                futures.spawn(get_resolution(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::DesktopEnvironment(config) => {
                futures.spawn(get_desktop(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                    config.clone(),
                ));
            }
            Entry::WindowManager => {
                futures.spawn(get_window_manager(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Terminal => {
                futures.spawn(get_terminal(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::TerminalFont => {
                futures.spawn(get_terminal_font(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::GPUS => {
                futures.spawn(get_gpus(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Memory => {
                futures.spawn(get_memory(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Network => {
                futures.spawn(get_network(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::Disk(config) => {
                futures.spawn(get_disks(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                    None,
                    Some(config.clone()),
                ));
            }
            Entry::Disks(config) => {
                futures.spawn(get_disks(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                    Some(config.clone()),
                    None,
                ));
            }
            Entry::PublicIP => {
                futures.spawn(get_public_ip(
                    Arc::clone(&shared_header_color),
                    Arc::clone(&shared_logo_color),
                    Arc::clone(&shared_language),
                ));
            }
            Entry::ColorBlocks => {
                futures.spawn(get_color_blocks());
            }
            Entry::EmptyLine => {
                futures.spawn(get_empty_line());
            }
        }
    }

    futures
}
