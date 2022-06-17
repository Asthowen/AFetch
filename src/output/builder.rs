use crate::output::entries_generator::OutputEntriesGenerator;
use sysinfo::{ProcessorExt, System, SystemExt, NetworkExt, DiskExt};
use crate::util::utils::convert_to_readable_unity;
use crate::system::os_infos::GetInfos;
use whoami::{username, hostname};
use crate::util::utils;
use colored::*;

pub struct OutputBuilder {
    show_logo: bool, disabled_entries: Vec<String>, fake_logo: String
}

impl OutputBuilder {
    pub fn init() -> OutputBuilder {
        OutputBuilder { show_logo: false, disabled_entries: vec![], fake_logo: String::new() }
    }

    pub fn show_logo(mut self, status: bool) -> OutputBuilder {
        self.show_logo = status;
        self
    }

    pub fn disable_entry(mut self, entry: &str) -> OutputBuilder {
        self.disabled_entries.push(entry.to_owned());
        self
    }

    pub fn fake_logo(mut self, logo_name: &str) -> OutputBuilder {
        self.fake_logo = logo_name.to_owned();
        self
    }

    pub fn disable_entries(mut self, entries: Vec<&str>) -> OutputBuilder {
        for entry in entries {
            self.disabled_entries.push(entry.to_owned());
        }
        self
    }

    pub fn generate_output(self) -> OutputBuilder {
        let get_infos_obj: GetInfos = GetInfos::init(self.fake_logo.clone());
        let mut system: System = System::new_all();
        system.refresh_all();
        let (username, host) = (username().cyan().bold(), hostname().cyan().bold());

        let mut infos: OutputEntriesGenerator = OutputEntriesGenerator::init(self.disabled_entries.clone());
        infos.add_custom_entry(format!("{}@{}", username, host));
        infos.add_custom_entry("\x1b[0m".to_owned() + &"─".repeat(username.len() + host.len() + 1));
        if system.name().unwrap().to_lowercase().contains("windows") {
            infos.add_entry("OS", format!("{} {}", system.name().unwrap(), system.os_version().unwrap().split(' ').collect::<Vec<&str>>()[0]));
        } else {
            infos.add_entry("OS", system.name().unwrap());
        }
        infos.add_entry("Host", get_infos_obj.get_host());
        infos.add_entry("Kernel", system.kernel_version().unwrap().replace('\n', ""));
        infos.add_entry("Uptime", utils::format_time(system.uptime()));
        infos.add_entry("Packages", get_infos_obj.get_packages_number());
        infos.add_entry("Resolution", get_infos_obj.get_screens_resolution());
        infos.add_entry("Shell", get_infos_obj.get_shell());
        infos.add_entry("Memory", format!("{}/{}", convert_to_readable_unity((system.used_memory() * 1000) as f64), convert_to_readable_unity((system.total_memory() * 1000) as f64)));
        let mut cpu_name: String = String::new();
        if system.global_processor_info().brand() != "" {
            cpu_name = system.global_processor_info().brand().to_owned();
        } else if system.global_processor_info().vendor_id() != "" {
            cpu_name = system.global_processor_info().vendor_id().to_owned();
        }
        if cpu_name.is_empty() {
            infos.add_entry("CPU", format!("{:.5}%", system.global_processor_info().cpu_usage().to_string()));
        } else {
            infos.add_entry("CPU", format!("{} - {:.5}%", cpu_name, system.global_processor_info().cpu_usage()));
        }
        let (mut network_sent, mut network_recv) = (0, 0);
        for (_, data) in system.networks() {
            network_sent += data.transmitted();
            network_recv += data.received();
        }
        infos.add_entry("Network", format!("download: {}/s - upload: {}/s", convert_to_readable_unity(network_sent as f64), convert_to_readable_unity(network_recv as f64)));
        let (mut total_disk_used, mut total_disk_total) = (0, 0);
        for disk in system.disks() {
            let disk_mount_point: String = disk.mount_point().to_str().unwrap().to_owned();
            if !disk_mount_point.contains("/docker") && !disk_mount_point.contains("/boot"){
                total_disk_used += disk.total_space() - disk.available_space();
                total_disk_total += disk.total_space();
                infos.add_entry(format!("Disk ({})", disk.mount_point().to_str().unwrap()).as_str(), format!("{}/{}", convert_to_readable_unity((disk.total_space() - disk.available_space()) as f64), convert_to_readable_unity(disk.total_space() as f64)));
            }
        }
        infos.add_entry("Disks", format!("{}/{}", convert_to_readable_unity(total_disk_used as f64), convert_to_readable_unity(total_disk_total as f64)));
        infos.add_entry("Public IP", get_infos_obj.get_public_ip());

        let infos_vector: Vec<String> = infos.get_entries();

        let mut to_print: String = String::new();
        if self.show_logo && get_infos_obj.get_os_logo() != "" {
            for (iteration, line) in get_infos_obj.get_os_logo().lines().enumerate() {
                if iteration < infos_vector.len() {
                    to_print.push_str(&*format!("{}    {}\n", line, infos_vector[iteration]));
                } else {
                    to_print.push_str(&*format!("{}\n", line));
                }
            }
        } else {
            for line in infos_vector {
                to_print.push_str(&*format!("{}\n", line));
            }
        }

        println!("{}", to_print);
        self
    }
}