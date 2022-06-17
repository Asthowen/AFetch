use crate::util::utils::{return_str_from_command, get_file_in_one_line, is_command_exist, check_if_env_exist, get_env};
use sysinfo::{System, SystemExt};
use crate::util::os_logos;
use std::process::Command;
use std::path::Path;

pub struct GetInfos {fake_logo: String}

impl Default for GetInfos {
    fn default() -> GetInfos {
        GetInfos::init("".to_string())
    }
}

impl GetInfos {
    pub fn init(fake_logo: String) -> GetInfos {
        GetInfos { fake_logo }
    }

    fn parse_os_release(&self, file_path: &str) -> String {
        let contents: String = std::fs::read_to_string(file_path).unwrap();
        contents.split("NAME=\"").collect::<Vec<&str>>()[1].split("\"\n").collect::<Vec<&str>>()[0].to_string()
    }

    pub fn get_linux_distribution(&self) -> String {
        if Path::new("/etc/os-release").exists() {
            return self.parse_os_release("/etc/os-release");
        } else if Path::new("/usr/lib/os-release").exists() {
            return self.parse_os_release("/usr/lib/os-release");
        } else if Path::new("/etc/openwrt_release").exists() {
            return self.parse_os_release("/etc/openwrt_release");
        } else if Path::new("/etc/lsb-release").exists() {
            return self.parse_os_release("/etc/lsb-release");
        } else if Path::new("/bedrock/etc/bedrock-release").exists() && std::env::var("BEDROCK_RESTRICT").is_ok() {
            return "Bedrock Linux".to_string();
        } else if Path::new("/etc/redstar-release").exists() {
            return "Red Star OS".to_string();
        } else if Path::new("/etc/armbian-release").exists() {
            return "Armbian".to_string();
        } else if Path::new("/etc/siduction-version").exists() {
            return "Siduction".to_string();
        } else if Path::new("/etc/mcst_version").exists() {
            return "OS Elbrus".to_string();
        } else if is_command_exist("pveversion") {
            return "Proxmox VE".to_string();
        } else if is_command_exist("lsb_release") {
            return match get_env("DISTRO_SHORTHAND").as_str() {
                "on" | "off" => return_str_from_command(Command::new("lsb_release").arg("-si")),
                _ => return_str_from_command(Command::new("lsb_release").arg("-sd"))
            };
        } else if Path::new("/etc/GoboLinuxVersion").exists() {
            return "GoboLinux".to_string();
        } else if Path::new("/etc/SDE-VERSION").exists() {
            return get_file_in_one_line("/etc/SDE-VERSION");
        } else if is_command_exist("tazpkg") {
            return "SliTaz".to_string();
        } else if is_command_exist("kpt") && is_command_exist("kpm") {
            return "KSLinux".to_string();
        } else if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
            return "Android".to_string();
        }

        "".to_string()
    }

    pub fn get_os_logo(&self) -> String {
        if !self.fake_logo.is_empty() {
            for (os_name, os_logo) in os_logos::logos_list() {
                if self.fake_logo.to_lowercase() == os_name.to_lowercase() {
                    return os_logo;
                }
            }
        }
        if std::env::consts::OS == "linux" {
            for (os_name, os_logo) in os_logos::logos_list() {
                if os_name.to_lowercase() == self.get_linux_distribution().to_lowercase().replace(' ', "") {
                    return os_logo;
                }
            }
        } else if std::env::consts::OS == "freebsd" {
            let os_logos_list: std::collections::HashMap<String, String> = os_logos::logos_list();
            return (&os_logos_list["FreeBSD"]).to_string();
        } else if std::env::consts::OS == "windows" {
            let os_logos_list: std::collections::HashMap<String, String> = os_logos::logos_list();
            let system: System = System::default();
            let windows_version: String = system.os_version().unwrap().split(' ').collect::<Vec<&str>>()[0].to_string();
            return (&os_logos_list[format!("Windows{}", if !windows_version.is_empty() {windows_version} else {String::from("11")}).as_str()]).to_string();
        }

        "".to_string()
    }

    pub fn get_host(&self) -> String {
        let mut host = String::new();
        match std::env::consts::OS {
            "linux" => {
                if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
                    host = return_str_from_command(Command::new("getprop").arg("ro.product.brand"));
                    host += return_str_from_command(Command::new("getprop").arg("ro.product.model")).as_str();
                } else if Path::new("/sys/devices/virtual/dmi/id/product_name").exists() && Path::new("/sys/devices/virtual/dmi/id/product_version").exists() {
                    host = get_file_in_one_line("/sys/devices/virtual/dmi/id/product_name");
                    host += " ";
                    host += get_file_in_one_line("/sys/devices/virtual/dmi/id/product_version").as_str();
                } else if Path::new("/sys/firmware/devicetree/base/model").exists() {
                    host = get_file_in_one_line("/sys/firmware/devicetree/base/model");
                } else if Path::new("/tmp/sysinfo/model").exists() {
                    host = get_file_in_one_line("/tmp/sysinfo/model");
                }

                if (host.contains("System Product Name") || !host.is_empty()) && Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists() && Path::new("/sys/devices/virtual/dmi/id/board_name").exists() {
                    host = get_file_in_one_line("/sys/devices/virtual/dmi/id/board_vendor");
                    host += " ";
                    host += get_file_in_one_line("/sys/devices/virtual/dmi/id/board_name").as_str();
                }

                host
            },
            "windows" => {
                host = return_str_from_command(Command::new("wmic").arg("computersystem").arg("get").arg("manufacturer,model")).replace("Manufacturer  Model", "").replace("     ", " ").trim().to_string();
                host
            },
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }

    pub fn get_shell(&self) -> String {
        let mut shell_path: String = String::new();
        let mut shell_name: String = String::new();

        if check_if_env_exist("SHELL") {
            shell_path = get_env("SHELL");
            let shell_name_spliced: Vec<&str> = shell_path.split('/').collect::<Vec<&str>>();
            shell_name = shell_name_spliced[shell_name_spliced.len() - 1].to_string();
        }

        if !shell_name.is_empty() {
            return if check_if_env_exist("SHELL_VERSION") {
                format!("{} {}", shell_name, get_env("SHELL_VERSION"))
            } else {
                let mut shell_version: String = String::new();
                if shell_name == "fish" {
                    shell_version = return_str_from_command(Command::new(shell_path).arg("--version")).split("fish, version ").collect::<Vec<&str>>()[1].replace('\n', "");
                } else if shell_name == "bash" {
                    shell_version = return_str_from_command(Command::new(shell_path).arg("-c").arg("echo $BASH_VERSION"));
                } else if shell_name == "sh" {
                    shell_version = return_str_from_command(Command::new("sh").arg("--version")).split("GNU bash, version ").collect::<Vec<&str>>()[1].split(' ').collect::<Vec<&str>>()[0].to_string();
                } else if shell_name == "ksh" {
                    shell_version = return_str_from_command(Command::new("ksh").arg("--version")).split("(AT&T Research) ").collect::<Vec<&str>>()[1].trim().to_string();
                }

                if !shell_version.is_empty() { shell_name } else { format!("{} {}", shell_name, shell_version) }
            }
        }
        "".to_string()
    }
    pub fn get_screens_resolution(&self) -> String {
        match std::env::consts::OS {
            "linux" => {
                let mut resolution: String = String::new();
                if is_command_exist("xrandr") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY") {
                    let mut last_line: bool = false;
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for line in return_str_from_command(Command::new("xrandr").arg("--nograb").arg("--current")).split('\n') {
                        if last_line {
                            temp_resolution.push(line.trim().split(' ').collect::<Vec<&str>>()[0].to_string());
                            last_line = false;
                        } else if line.contains(" connected"){
                            last_line = true;
                        }
                    }
                    resolution = temp_resolution.join(" ");
                } else if is_command_exist("xwininfo") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY") {
                    let command: String = return_str_from_command(Command::new("xwininfo").arg("-root"));
                    resolution = format!(
                        "{}x{}",
                        command.split("Width: ").collect::<Vec<&str>>()[1].split('\n').collect::<Vec<&str>>()[0],
                        command.split("Height: ").collect::<Vec<&str>>()[1].split('\n').collect::<Vec<&str>>()[0]
                    );
                } else if is_command_exist("xdpyinfo") && check_if_env_exist("DISPLAY") && check_if_env_exist("WAYLAND_DISPLAY") {
                    resolution = return_str_from_command(&mut Command::new("xdpyinfo")).split("dimensions: ").collect::<Vec<&str>>()[1].trim().split(' ').collect::<Vec<&str>>()[0].to_string();
                } else if Path::new("/sys/class/drm").exists() {
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for path in std::fs::read_dir("/sys/class/drm/").unwrap() {
                        if path.as_ref().unwrap().path().is_dir() {
                            for sub_path in std::fs::read_dir(path.as_ref().unwrap().path().display().to_string()).unwrap() {
                                if sub_path.as_ref().unwrap().file_name().to_string_lossy().contains("modes")  {
                                    let first_line: String = std::fs::read_to_string(sub_path.as_ref().unwrap().path().display().to_string().as_str()).unwrap().split('\n').collect::<Vec<&str>>()[0].to_string();
                                    if !first_line.is_empty() {
                                        temp_resolution.push(first_line);
                                    }
                                }
                            }
                        }
                    }
                    resolution = temp_resolution.join(", ");
                }

                resolution
            },
            "windows" => {
                let width: String = return_str_from_command(Command::new("wmic").arg("path").arg("Win32_VideoController").arg("get").arg("CurrentHorizontalResolution")).replace("CurrentHorizontalResolution", "").trim().to_string();
                let height: String = return_str_from_command(Command::new("wmic").arg("path").arg("Win32_VideoController").arg("get").arg("CurrentVerticalResolution")).replace("CurrentVerticalResolution", "").trim().to_string();
                format!("{}x{}", width, height)
            },
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }
    fn count_lines_in_output(&self, output: String) -> usize {
        output.split('\n').count()
    }

    pub fn get_packages_number(&self) -> String {
        let mut packages_string: Vec<String> = Vec::new();

        match std::env::consts::OS {
            "linux" | "freebsd" | "solaris" => {
                if is_command_exist("pacman") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pacman").arg("-Qq").arg("--color").arg("never"))).to_string() + " (pacman)");
                }
                if is_command_exist("kiss") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("kiss").arg("l"))).to_string() + " (kiss)");
                }
                if is_command_exist("cpt-list") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(&mut Command::new("cpt-list"))).to_string() + " (kiss)");
                }
                if is_command_exist("dpkg") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("dpkg-query").arg("-f").arg("'.\n'").arg("-W"))).to_string() + " (dpkg)");
                }
                if is_command_exist("xbps-query") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("xbps-query").arg("-l"))).to_string() + " (xbps-query)");
                }
                if is_command_exist("apk") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("apk").arg("info"))).to_string() + " (apk)");
                }
                if is_command_exist("opkg") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("opkg").arg("list-installed"))).to_string() + " (opkg)");
                }
                if is_command_exist("pacman-g2") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pacman-g2").arg("-Q"))).to_string() + " (pacman-g2)");
                }
                if is_command_exist("lvu") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("lvu").arg("installed"))).to_string() + " (lvu)");
                }
                if is_command_exist("tce-status") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("tce-status").arg("-i"))).to_string() + " (tce)");
                }
                if is_command_exist("pkg_info") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(&mut Command::new("pkg_info"))).to_string() + " (pkg)");
                }
                if is_command_exist("pkg") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pkg").arg("info"))).to_string() + " (pkg)");
                }
                if is_command_exist("pkgin") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pkgin").arg("list"))).to_string() + " (pkgin)");
                }
                if is_command_exist("tazpkg") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pkgs_h=6").arg("tazpkg").arg("list").arg("&&").arg("((packages-=6))"))).to_string() + " (tazpkg)");
                }
                if is_command_exist("sorcery") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("gaze").arg("installed"))).to_string() + " (sorcery)");
                }
                if is_command_exist("alps") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("alps").arg("showinstalled"))).to_string() + " (alps)");
                }
                if is_command_exist("butch") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("butch").arg("list"))).to_string() + " (butch)");
                }
                if is_command_exist("swupd") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("swupd").arg("bundle-list").arg("--quiet"))).to_string() + " (swupd)");
                }
                if is_command_exist("pisi") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pisi").arg("li"))).to_string() + " (pisi)");
                }
                if is_command_exist("pacstall") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("pacstall").arg("-L"))).to_string() + " (pacstall)");
                }
                if is_command_exist("dnf") && is_command_exist("sqlite3") && Path::new("/var/cache/dnf/packages.db").exists() {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("sqlite3").arg("/var/cache/dnf/packages.db").arg(r#""SELECT count(pkg) FROM installed""#))).to_string() + " (dnf)");
                } else if is_command_exist("rpm") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("rpm").arg("-qa"))).to_string() + " (dnf)");
                }
                if Path::new("/etc/SDE-VERSION").exists() {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("mine").arg("-q"))).to_string() + " (dnf)");
                }
                if is_command_exist("flatpak") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("flatpak").arg("list"))).to_string() + " (flatpak)");
                }
                if is_command_exist("spm") {
                    packages_string.push(self.count_lines_in_output(return_str_from_command(Command::new("flatpak").arg("list").arg("-i"))).to_string() + " (spm)");
                }
                if is_command_exist("snap") {
                    packages_string.push((self.count_lines_in_output(return_str_from_command(Command::new("snap").arg("list"))) - 1).to_string() + " (snap)");
                }

                packages_string.join(" ")
            },
            "windows" => {
                if is_command_exist("choco") {
                    let choco_output = return_str_from_command(Command::new("choco").arg("list").arg("--localonly"));
                    let choco_output_split: Vec<&str> = choco_output.split(" packages installed").collect::<Vec<&str>>()[0].split('\n').collect::<Vec<&str>>();
                    packages_string.push((choco_output_split[choco_output_split.len() - 1]).to_string() + " (chocolatey)");
                }
                
                packages_string.join(" ")
            },
            _ => {
                // TODO - add other OS
                "".to_string()
            }
        }
    }
    pub fn get_public_ip(&self) -> String {
        match minreq::get("https://ipinfo.io/ip").send() {
            Ok(response) => {
                response.as_str().unwrap().to_string()
            }
            Err(_) => "".to_string()
        }
    }
}