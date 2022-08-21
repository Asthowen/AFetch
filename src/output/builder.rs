use sysinfo::{System, SystemExt, NetworkExt, DiskExt, CpuExt, Cpu};
use crate::output::entries_generator::OutputEntriesGenerator;
use crate::util::utils::convert_to_readable_unity;
use crate::system::os_infos::GetInfos;
use whoami::{username, hostname};
use crate::util::utils;
use yaml_rust::Yaml;
use colored::*;

#[derive(Clone, Debug)]
pub struct OutputBuilder {
    show_logo: bool, disabled_entries: Vec<String>, fake_logo: String, pub config: Yaml
}

impl OutputBuilder {
    pub const fn init(config: Yaml) -> Self {
        Self { show_logo: false, disabled_entries: Vec::new(), fake_logo: String::new(), config }
    }

    pub const fn show_logo(mut self, status: bool) -> Self {
        self.show_logo = status;
        self
    }

    pub fn disable_entry(mut self, entry: &str) -> Self {
        self.disabled_entries.push(entry.to_owned().replace(' ', "").to_lowercase());
        self
    }

    pub fn fake_logo(mut self, logo_name: &str) -> Self {
        self.fake_logo = logo_name.to_owned();
        self
    }

    pub fn disable_entries(mut self, entries: Vec<String>) -> Self {
        for entry in entries {
            self.disabled_entries.push(entry);
        }
        self
    }

    pub fn generate_output(self) -> Self {
        let get_infos: GetInfos = GetInfos::init(self.fake_logo.clone());
        let mut system: System = System::new_all();
        system.refresh_all();
        let (username, host) = (username(), hostname());

        let mut infos: OutputEntriesGenerator = OutputEntriesGenerator::init();
        infos.add_custom_entry(format!("{}@{}", username, host).cyan().bold().to_string());
        infos.add_custom_entry(format!("\x1b[0m{}", "─".repeat(username.len() + host.len() + 1)));

        if !self.disabled_entries.contains(&"os".to_owned()) {
            if system.name().unwrap().to_lowercase().contains("windows") {
                infos.add_entry("OS", format!("{} {}", system.name().unwrap(), system.os_version().unwrap().split(' ').collect::<Vec<&str>>()[0]));
            } else {
                infos.add_entry("OS", system.name().unwrap());
            }
        }
        if !self.disabled_entries.contains(&"host".to_owned()) {
            infos.add_entry("Host", get_infos.get_host());
        }
        if !self.disabled_entries.contains(&"kernel".to_owned()) {
            infos.add_entry("Kernel", system.kernel_version().unwrap().replace('\n', ""));
        }
        if !self.disabled_entries.contains(&"uptime".to_owned()) {
            infos.add_entry("Uptime", utils::format_time(system.uptime()));
        }
        if !self.disabled_entries.contains(&"packages".to_owned()) {
            infos.add_entry("Packages", get_infos.get_packages_number());
        }
        if !self.disabled_entries.contains(&"resolution".to_owned()) {
            infos.add_entry("Resolution", get_infos.get_screens_resolution());
        }
        if !self.disabled_entries.contains(&"shell".to_owned()) {
            infos.add_entry("Shell", get_infos.get_shell());
        }
        if !self.disabled_entries.contains(&"memory".to_owned()) {
            infos.add_entry("Memory", format!("{}/{}", convert_to_readable_unity((system.used_memory() * 1000) as f64), convert_to_readable_unity((system.total_memory() * 1000) as f64)));
        }
        if !self.disabled_entries.contains(&"cpu".to_owned()) {
            let cpu_infos: &Cpu = system.global_cpu_info();
            let cpu_name: String = if !cpu_infos.brand().is_empty() {
                cpu_infos.brand().to_owned()
            } else if !system.global_cpu_info().vendor_id().is_empty() {
                cpu_infos.vendor_id().to_owned()
            } else {
                "".to_owned()
            };

            if cpu_name.is_empty() {
                infos.add_entry("CPU", format!("{:.5}%", cpu_infos.cpu_usage().to_string()));
            } else {
                infos.add_entry("CPU", format!("{} - {:.5}%", cpu_name, cpu_infos.cpu_usage()));
            }
        }
        if !self.disabled_entries.contains(&"network".to_owned()) {
            let (mut network_sent, mut network_recv) = (0, 0);
            for (_, data) in system.networks() {
                network_sent += data.transmitted();
                network_recv += data.received();
            }
            infos.add_entry("Network", format!("download: {}/s - upload: {}/s", convert_to_readable_unity(network_sent as f64), convert_to_readable_unity(network_recv as f64)));
        }
        let print_disk: bool = !self.disabled_entries.contains(&"disk".to_owned());
        let print_disks: bool = !self.disabled_entries.contains(&"disks".to_owned());

        if print_disks || print_disk {
            let (mut total_disk_used, mut total_disk_total) = (0, 0);
            for disk in system.disks() {
                let disk_mount_point: String = disk.mount_point().to_str().unwrap().to_owned();
                if !disk_mount_point.contains("/docker") && !disk_mount_point.contains("/boot") {
                    total_disk_used += disk.total_space() - disk.available_space();
                    total_disk_total += disk.total_space();
                    if print_disk {
                        infos.add_entry(format!("Disk ({})", disk.mount_point().to_str().unwrap_or("")).as_str(), format!("{}/{}", convert_to_readable_unity((disk.total_space() - disk.available_space()) as f64), convert_to_readable_unity(disk.total_space() as f64)));
                    }
                }
            }
            if print_disks {
                infos.add_entry("Disks", format!("{}/{}", convert_to_readable_unity(total_disk_used as f64), convert_to_readable_unity(total_disk_total as f64)));
            }
        }
        if !self.disabled_entries.contains(&"publicip".to_owned()) {
            infos.add_entry("Public IP", get_infos.get_public_ip());
        }

        let infos_vector: Vec<String> = infos.get_entries();
        let mut to_print: String = String::new();

        if self.show_logo && !get_infos.get_os_logo().is_empty() {
            for (iteration, line) in get_infos.get_os_logo().lines().enumerate() {
                if iteration < infos_vector.len() {
                    to_print.push_str(&*format!("{}    {}\n", line, infos_vector[iteration]));
                } else {
                    to_print.push_str(&*format!("{}\n", line));
                }
            }
            let lines_count: usize = get_infos.get_os_logo().lines().count();
            if lines_count < infos_vector.len() {
                for iteration in infos_vector.iter().skip(lines_count)  {
                    to_print.push_str(&*format!("{}\n", iteration));
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