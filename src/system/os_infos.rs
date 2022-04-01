use crate::util::utils::{return_str_from_command, get_file_in_one_line, is_command_exist, check_if_env_exist, execute_command};
use std::collections::HashMap;
use crate::util::os_logos;
use std::process::Command;
use std::path::Path;

pub struct GetInfos {fake_logo: String}

impl GetInfos {
    pub fn init(fake_logo: String) -> GetInfos {
        GetInfos { fake_logo }
    }

    fn parse_os_release(self, file_path: &str) -> String {
        let contents: String = std::fs::read_to_string(file_path).unwrap();
        contents.split("NAME=\"").collect::<Vec<&str>>()[1].split("\"\n").collect::<Vec<&str>>()[0].to_string()
    }

    pub fn get_linux_distribution(self) -> String {
        if std::path::Path::new("/etc/os-release").exists() {
            return self.parse_os_release("/etc/os-release");
        } else if std::path::Path::new("/usr/lib/os-release").exists() {
            return self.parse_os_release("/usr/lib/os-release");
        } else if std::path::Path::new("/etc/openwrt_release").exists() {
            return self.parse_os_release("/etc/openwrt_release");
        } else if std::path::Path::new("/etc/lsb-release").exists() {
            return self.parse_os_release("/etc/lsb-release");
        } else if std::path::Path::new("/bedrock/etc/bedrock-release").exists() && std::env::var("BEDROCK_RESTRICT").is_ok() {
            return "Bedrock Linux".to_string();
        } else if std::path::Path::new("/etc/redstar-release").exists() {
            return "Red Star OS".to_string();
        } else if std::path::Path::new("/etc/armbian-release").exists() {
            return "Armbian".to_string();
        } else if std::path::Path::new("/etc/siduction-version").exists() {
            return "Siduction".to_string();
        } else if std::path::Path::new("/etc/mcst_version").exists() {
            return "OS Elbrus".to_string();
        } else if std::path::Path::new("/etc/mcst_version").exists() {
            return "OS Elbrus".to_string();
        } else if is_command_exist("pveversion") {
            return "Proxmox VE".to_string();
        } else if is_command_exist("lsb_release") {
            // TODO
        } else if std::path::Path::new("/etc/GoboLinuxVersion").exists() {
            return "GoboLinux".to_string();
        } else if std::path::Path::new("/etc/SDE-VERSION").exists() {
            return get_file_in_one_line("/etc/SDE-VERSION");
        } else if is_command_exist("tazpkg") {
            return "SliTaz".to_string();
        } else if is_command_exist("kpt") && is_command_exist("kpm") {
            return "KSLinux".to_string();
        } else if std::path::Path::new("/system/app/").exists() && std::path::Path::new("/system/priv-app").exists() {
            return "Android".to_string();
        }

        "".to_string()
    }

    pub fn get_os_logo(self) -> String {
        if !self.fake_logo.is_empty() {
            for (os_name, os_logo) in os_logos::logos_list() {
                if self.fake_logo == os_name {
                    return os_logo;
                }
            }
        }

        let os_logos_list: HashMap<String, String> = os_logos::logos_list();
        match std::env::consts::OS {
            "linux" => {
                match &*self.get_linux_distribution().to_lowercase() {
                    "arch linux" => (&os_logos_list["arch_linux"]).to_string(),
                    "debian" => (&os_logos_list["debian"]).to_string(),
                    "ubuntu" => (&os_logos_list["ubuntu"]).to_string(),
                    "kubuntu" => (&os_logos_list["kubuntu"]).to_string(),
                    "xubuntu" => (&os_logos_list["xubuntu"]).to_string(),
                    "lubuntu" => (&os_logos_list["lubuntu"]).to_string(),
                    "manjaro" => (&os_logos_list["manjaro"]).to_string(),
                    "fedora" => (&os_logos_list["fedora"]).to_string(),
                    "linux mint" => (&os_logos_list["linux_mint"]).to_string(),
                    "pop os" => (&os_logos_list["pop_os"]).to_string(),
                    _ => (&os_logos_list["linux"]).to_string()
                }
            },
            "windows" => {
                (&os_logos_list["windows10"]).to_string()
            },
            "macos" => (&os_logos_list["macos"]).to_string(),
            "freebsd" => (&os_logos_list["freebsd"]).to_string(),
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }

    pub fn get_host(&mut self) -> String {
        match std::env::consts::OS {
            "linux" => {
                let mut host = String::new();

                if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
                    host = return_str_from_command(Command::new("getprop").arg("ro.product.brand"));
                    host += &*return_str_from_command(Command::new("getprop").arg("ro.product.model"));
                }
                if Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists() && Path::new("/sys/devices/virtual/dmi/id/board_name").exists() {
                    host = get_file_in_one_line("/sys/devices/virtual/dmi/id/board_vendor");
                    host += " ";
                    host += &*get_file_in_one_line("/sys/devices/virtual/dmi/id/board_name");
                }
                if Path::new("/sys/devices/virtual/dmi/id/product_name").exists() && Path::new("/sys/devices/virtual/dmi/id/product_version").exists() {
                    host = get_file_in_one_line("/sys/devices/virtual/dmi/id/product_name");
                    host += " ";
                    host += &*get_file_in_one_line("/sys/devices/virtual/dmi/id/product_version");
                }
                if Path::new("/sys/firmware/devicetree/base/model").exists() {
                    host = get_file_in_one_line("/sys/firmware/devicetree/base/model");
                }
                if Path::new("/tmp/sysinfo/model").exists() {
                    host = get_file_in_one_line("/tmp/sysinfo/model");
                }

                host
            }
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }
    pub fn get_shell(&mut self) -> String {
        let mut shell_path: String = String::new();
        let mut shell_name: String = String::new();
        match std::env::var("SHELL") {
            Ok(var) => {
                shell_path = String::from(&*var);
                let shell_name_spliced: Vec<&str> = var.split("/").collect::<Vec<&str>>();
                shell_name = shell_name_spliced[shell_name_spliced.len() - 1].to_string();
            },
            Err(_) => {}
        }
        if shell_name != "" {
            return match std::env::var("SHELL_VERSION") {
                Ok(shell_version) => format!("{} {}", shell_name, shell_version),
                _ => {
                    let mut shell_version: String = String::new();
                    if shell_name == "fish" {
                        shell_version = return_str_from_command(Command::new(shell_path).arg("--version")).split("fish, version ").collect::<Vec<&str>>()[1].replace("\n", "");
                    } else if shell_name == "bash" {
                        shell_version = return_str_from_command(Command::new(shell_path).arg("-c").arg("echo $BASH_VERSION"));
                    } else if vec!["sh", "ash", "dash", "es"].contains(&&*shell_name){
                        // TODO
                    } else if shell_name == "osh" {
                        // TODO
                    } else if shell_name == "ksh" {
                        // TODO
                    } else if shell_name == "tcsh" {
                        // TODO
                    } else if shell_name == "yash" {
                        // TODO
                    } else if shell_name == "nu" {
                        // TODO
                    }

                    return if shell_version == "" {shell_name} else {format!("{} {}", shell_name, shell_version)};
                }
            }
        }
        "".to_string()
    }
    pub fn get_screens_resolution(&mut self) -> String {
        match std::env::consts::OS {
            "linux" => {
                let mut resolution: String = String::new();

                if is_command_exist("xrandr") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY")  {
                    match std::env::var("REFRESH_RATE").unwrap().as_str() {
                        "on" => {
                            resolution = execute_command(r#"xrandr --nograb --current | awk 'match($0,/[0-9]*\.[0-9]*\*/) {printf $1 " @ " substr($0,RSTART,RLENGTH) "Hz, "}'"#);
                        },
                        "off" => {
                            resolution = execute_command(r#"xrandr --nograb --current | awk -F 'connected |\\+|\\(/ connected.*[0-9]+x[0-9]+\+/ && $2 {printf $2 ", "}'"#);
                        },
                        _ => {}
                    }
                } else if is_command_exist("xwininfo") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY") {
                    let command: String = execute_command("xwininfo -root");
                    let width = command.split("Width: ").collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>()[0];
                    let height = command.split("Height: ").collect::<Vec<&str>>()[1].split("\n").collect::<Vec<&str>>()[0];
                    resolution = format!("{}x{}", width, height);
                } else if is_command_exist("xdpyinfo") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY") {
                    resolution = execute_command("xdpyinfo | awk '/dimensions:/ {printf $2}'");
                } else if Path::new("/sys/class/drm").exists() {
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for path in std::fs::read_dir("/sys/class/drm/").unwrap() {
                        if path.as_ref().unwrap().path().is_dir() {
                            for sub_path in std::fs::read_dir(path.as_ref().unwrap().path().display().to_string()).unwrap() {
                                if sub_path.as_ref().unwrap().file_name().to_string_lossy().contains("modes")  {
                                    let first_line: String = std::fs::read_to_string(sub_path.as_ref().unwrap().path().display().to_string().as_str()).unwrap().split("\n").collect::<Vec<&str>>()[0].to_string();
                                    if first_line != "" {
                                        temp_resolution.push(first_line);
                                    }
                                }
                            }
                        }
                    }
                    resolution = temp_resolution.join(", ");
                }

                resolution
            }
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }
}