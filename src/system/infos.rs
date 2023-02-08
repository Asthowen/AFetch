use crate::system::logos;
use crate::utils::{command_exist, env_exist, get_env, get_file_content, return_str_from_command};
use std::path::Path;
use std::process::Command;
use sysinfo::{System, SystemExt};

pub struct Infos {
    pub sysinfo_obj: System,
}

impl Infos {
    pub fn init() -> Self {
        Self {
            sysinfo_obj: System::new_all(),
        }
    }

    fn parse_os_release(&self, file_path: &str) -> String {
        let contents = std::fs::read_to_string(file_path).unwrap_or_else(|_| "".to_owned());
        let split_lines = contents.split('\n');

        split_lines
            .filter_map(|value| {
                let mut parts = value.splitn(2, '=');
                match (parts.next(), parts.next()) {
                    (Some("ID"), Some(part)) | (Some("NAME"), Some(part)) => Some(part),
                    _ => None,
                }
            })
            .map(|value| value.trim_matches('"'))
            .next()
            .unwrap_or("")
            .to_owned()
    }

    pub fn get_linux_distribution(&self) -> String {
        let mut distribution_name: String = if Path::new("/etc/os-release").exists() {
            self.parse_os_release("/etc/os-release")
        } else if Path::new("/usr/lib/os-release").exists() {
            self.parse_os_release("/usr/lib/os-release")
        } else if Path::new("/etc/openwrt_release").exists() {
            self.parse_os_release("/etc/openwrt_release")
        } else if Path::new("/etc/lsb-release").exists() {
            self.parse_os_release("/etc/lsb-release")
        } else if Path::new("/besdrock/etc/bedrock-release").exists()
            && env_exist("BEDROCK_RESTRICT")
        {
            "Bedrock Linux".to_owned()
        } else if Path::new("/etc/redstar-release").exists() {
            "Red Star OS".to_owned()
        } else if Path::new("/etc/armbian-release").exists() {
            "Armbian".to_owned()
        } else if Path::new("/etc/siduction-version").exists() {
            "Siduction".to_owned()
        } else if Path::new("/etc/mcst_version").exists() {
            "OS Elbrus".to_owned()
        } else if command_exist("pveversion") {
            "Proxmox VE".to_owned()
        } else if command_exist("lsb_release") {
            match get_env("DISTRO_SHORTHAND").as_str() {
                "on" | "off" => return_str_from_command(Command::new("lsb_release").arg("-si")),
                _ => return_str_from_command(Command::new("lsb_release").arg("-sd")),
            }
        } else if Path::new("/etc/GoboLinuxVersion").exists() {
            "GoboLinux".to_owned()
        } else if Path::new("/etc/SDE-VERSION").exists() {
            get_file_content("/etc/SDE-VERSION")
        } else if command_exist("tazpkg") {
            "SliTaz".to_owned()
        } else if command_exist("kpt") && command_exist("kpm") {
            "KSLinux".to_owned()
        } else if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
            "Android".to_owned()
        } else {
            "".to_owned()
        };

        if distribution_name == "Ubuntu" && env_exist("XDG_CONFIG_DIRS") {
            let env_value: String = get_env("XDG_CONFIG_DIRS");
            if env_value.contains("cinnamon") {
                distribution_name = "Ubuntu Cinnamon".to_owned();
            } else if env_value.contains("studio") {
                distribution_name = "Ubuntu Studio".to_owned();
            } else if env_value.contains("plasma") || env_value.contains("xubuntu") {
                distribution_name = "Kubuntu".to_owned();
            } else if env_value.contains("mate") {
                distribution_name = "Ubuntu Mate".to_owned();
            } else if env_value.contains("lubuntu") {
                distribution_name = "Lubuntu".to_owned();
            } else if env_value.contains("budgie") {
                distribution_name = "Ubuntu Budgie".to_owned();
            }
        }

        distribution_name
    }

    pub fn get_os_logo(&self) -> &str {
        if std::env::consts::OS == "linux" {
            let current_os_id: String = self
                .get_linux_distribution()
                .to_lowercase()
                .replace(' ', "");

            for (os_name, os_logo) in logos::logos_list() {
                if os_name.to_lowercase() == current_os_id {
                    return os_logo;
                }
            }
        } else if std::env::consts::OS == "freebsd" {
            let os_logos_list: std::collections::HashMap<&'static str, &'static str> =
                logos::logos_list();
            return os_logos_list["FreeBSD"];
        } else if std::env::consts::OS == "windows" {
            let os_logos_list: std::collections::HashMap<&'static str, &'static str> =
                logos::logos_list();
            let system: System = System::default();
            let windows_version: String = system
                .os_version()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()[0]
                .to_owned();
            return os_logos_list[format!(
                "Windows{}",
                if !windows_version.is_empty() {
                    windows_version
                } else {
                    String::from("11")
                }
            )
            .as_str()];
        }

        ""
    }

    pub fn get_host(&self) -> String {
        let mut host = String::new();
        match std::env::consts::OS {
            "linux" => {
                if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
                    host = return_str_from_command(Command::new("getprop").arg("ro.product.brand"));
                    host +=
                        return_str_from_command(Command::new("getprop").arg("ro.product.model"))
                            .as_str();
                } else if Path::new("/sys/devices/virtual/dmi/id/product_name").exists()
                    && Path::new("/sys/devices/virtual/dmi/id/product_version").exists()
                {
                    host = get_file_content("/sys/devices/virtual/dmi/id/product_name");
                    host += " ";
                    host +=
                        get_file_content("/sys/devices/virtual/dmi/id/product_version").as_str();
                } else if Path::new("/sys/firmware/devicetree/base/model").exists() {
                    host = get_file_content("/sys/firmware/devicetree/base/model");
                } else if Path::new("/tmp/sysinfo/model").exists() {
                    host = get_file_content("/tmp/sysinfo/model");
                }

                if (host.contains("System Product Name") || !host.is_empty())
                    && Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists()
                    && Path::new("/sys/devices/virtual/dmi/id/board_name").exists()
                {
                    host = get_file_content("/sys/devices/virtual/dmi/id/board_vendor");
                    host += " ";
                    host += get_file_content("/sys/devices/virtual/dmi/id/board_name").as_str();
                }

                host
            }
            "windows" => {
                host = return_str_from_command(
                    Command::new("wmic")
                        .arg("computersystem")
                        .arg("get")
                        .arg("manufacturer,model"),
                )
                .replace("Manufacturer  Model", "")
                .replace("     ", " ")
                .trim()
                .to_owned();
                host
            }
            _ => {
                // TODO - add other OS
                "".to_owned()
            }
        }
    }

    pub fn get_shell(&self) -> String {
        let mut shell_path: String = String::new();
        let mut shell_name: String = String::new();

        if env_exist("SHELL") {
            shell_path = get_env("SHELL");
            let shell_name_spliced: Vec<&str> = shell_path.split('/').collect::<Vec<&str>>();
            shell_name = shell_name_spliced[shell_name_spliced.len() - 1].to_owned();
        }

        if !shell_name.is_empty() {
            return if env_exist("SHELL_VERSION") {
                format!("{} {}", shell_name, get_env("SHELL_VERSION"))
            } else {
                let mut shell_version: String = String::new();
                if shell_name == "fish" {
                    shell_version =
                        return_str_from_command(Command::new(shell_path).arg("--version"))
                            .split("fish, version ")
                            .collect::<Vec<&str>>()[1]
                            .replace('\n', "");
                } else if shell_name == "bash" {
                    shell_version = return_str_from_command(
                        Command::new(shell_path).arg("-c").arg("echo $BASH_VERSION"),
                    );
                } else if shell_name == "sh" {
                    shell_version = return_str_from_command(Command::new("sh").arg("--version"))
                        .split("GNU bash, version ")
                        .collect::<Vec<&str>>()[1]
                        .split(' ')
                        .collect::<Vec<&str>>()[0]
                        .to_owned();
                } else if shell_name == "ksh" {
                    shell_version = return_str_from_command(Command::new("ksh").arg("--version"))
                        .split("(AT&T Research) ")
                        .collect::<Vec<&str>>()[1]
                        .trim()
                        .to_owned();
                }

                if shell_version.is_empty() {
                    shell_name
                } else {
                    format!("{} {}", shell_name, shell_version)
                }
            };
        }
        "".to_owned()
    }
    pub fn get_screens_resolution(&self) -> String {
        match std::env::consts::OS {
            "linux" => {
                let mut resolution: String = String::new();
                if command_exist("xrandr") && env_exist("DISPLAY") && env_exist("WAYLAND_DISPLAY") {
                    let mut last_line: bool = false;
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for line in return_str_from_command(
                        Command::new("xrandr").arg("--nograb").arg("--current"),
                    )
                    .split('\n')
                    {
                        if last_line {
                            temp_resolution
                                .push(line.trim().split(' ').collect::<Vec<&str>>()[0].to_owned());
                            last_line = false;
                        } else if line.contains(" connected") {
                            last_line = true;
                        }
                    }
                    resolution = temp_resolution.join(" ");
                } else if command_exist("xwininfo")
                    && env_exist("DISPLAY")
                    && env_exist("WAYLAND_DISPLAY")
                {
                    let command: String =
                        return_str_from_command(Command::new("xwininfo").arg("-root"));
                    resolution = format!(
                        "{}x{}",
                        command.split("Width: ").collect::<Vec<&str>>()[1]
                            .split('\n')
                            .collect::<Vec<&str>>()[0],
                        command.split("Height: ").collect::<Vec<&str>>()[1]
                            .split('\n')
                            .collect::<Vec<&str>>()[0]
                    );
                } else if command_exist("xdpyinfo")
                    && env_exist("DISPLAY")
                    && env_exist("WAYLAND_DISPLAY")
                {
                    resolution = return_str_from_command(&mut Command::new("xdpyinfo"))
                        .split("dimensions: ")
                        .collect::<Vec<&str>>()[1]
                        .trim()
                        .split(' ')
                        .collect::<Vec<&str>>()[0]
                        .to_owned();
                } else if Path::new("/sys/class/drm").exists() {
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for path in std::fs::read_dir("/sys/class/drm/").unwrap() {
                        if path.as_ref().unwrap().path().is_dir() {
                            for sub_path in std::fs::read_dir(
                                path.as_ref().unwrap().path().display().to_string(),
                            )
                            .unwrap()
                            {
                                if sub_path
                                    .as_ref()
                                    .unwrap()
                                    .file_name()
                                    .to_string_lossy()
                                    .contains("modes")
                                {
                                    let first_line: String = std::fs::read_to_string(
                                        sub_path
                                            .as_ref()
                                            .unwrap()
                                            .path()
                                            .display()
                                            .to_string()
                                            .as_str(),
                                    )
                                    .unwrap()
                                    .split('\n')
                                    .collect::<Vec<&str>>()[0]
                                        .to_owned();
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
            }
            "windows" => {
                let width: String = return_str_from_command(
                    Command::new("wmic")
                        .arg("path")
                        .arg("Win32_VideoController")
                        .arg("get")
                        .arg("CurrentHorizontalResolution"),
                )
                .replace("CurrentHorizontalResolution", "")
                .trim()
                .to_owned();
                let height: String = return_str_from_command(
                    Command::new("wmic")
                        .arg("path")
                        .arg("Win32_VideoController")
                        .arg("get")
                        .arg("CurrentVerticalResolution"),
                )
                .replace("CurrentVerticalResolution", "")
                .trim()
                .to_owned();
                format!("{}x{}", width, height)
            }
            _ => {
                // TODO - add other OS
                "".to_owned()
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
                let package_managers = [
                    ("pacman", vec!["pacman", "-Qq", "--color", "never"]),
                    ("kiss", vec!["kiss", "l"]),
                    ("cpt", vec!["cpt-list"]),
                    ("dpkg", vec!["dpkg-query", "-f", "'.\n'", "-W"]),
                    ("xbps-query", vec!["xbps-query", "-l"]),
                    ("apk", vec!["apk", "info"]),
                    ("opkg", vec!["opkg", "list-installed"]),
                    ("pacman-g2", vec!["pacman-g2", "-Q"]),
                    ("lvu", vec!["lvu", "installed"]),
                    ("tce", vec!["tce-status", "-i"]),
                    ("pkg", vec!["pkg_info"]),
                    ("pkg", vec!["pkg", "info"]),
                    ("pkgin", vec!["pkgin", "list"]),
                    (
                        "tazpkg",
                        vec!["pkgs_h=6", "tazpkg", "list", "&&", "((packages-=6))"],
                    ),
                    ("sorcery", vec!["gaze", "installed"]),
                    ("alps", vec!["alps", "showinstalled"]),
                    ("butch", vec!["butch", "list"]),
                    ("swupd", vec!["swupd", "bundle-list", "--quiet"]),
                    ("pisi", vec!["pisi", "li"]),
                    ("pacstall", vec!["pacstall", "-L"]),
                    ("flatpak", vec!["flatpak", "list"]),
                    ("spm", vec!["spm", "list", "-i"]),
                    ("snap", vec!["snap", "list"]),
                    ("snap", vec!["mine", "-q"]),
                ];

                for (name, mut args) in package_managers {
                    let command_name = args[0];
                    if command_exist(command_name) {
                        args.remove(0);
                        packages_string.push(format!(
                            "{} ({})",
                            self.count_lines_in_output(return_str_from_command(
                                Command::new(command_name).args(args)
                            )),
                            name
                        ));
                    }
                }

                if command_exist("dnf")
                    && command_exist("sqlite3")
                    && Path::new("/var/cache/dnf/packages.db").exists()
                {
                    packages_string.push(format!(
                        "{} (dnf)",
                        self.count_lines_in_output(return_str_from_command(
                            Command::new("sqlite3")
                                .arg("/var/cache/dnf/packages.db")
                                .arg(r#""SELECT count(pkg) FROM installed""#),
                        ))
                    ));
                } else if command_exist("rpm") {
                    packages_string.push(format!(
                        "{} (dnf)",
                        self.count_lines_in_output(return_str_from_command(
                            Command::new("rpm").arg("-qa"),
                        ))
                    ));
                }

                packages_string.join(" ")
            }
            "windows" => {
                if command_exist("choco") {
                    let choco_output: String = return_str_from_command(
                        Command::new("choco").arg("list").arg("--localonly"),
                    );
                    let choco_output_split: Vec<&str> = choco_output
                        .split(" packages installed")
                        .collect::<Vec<&str>>()[0]
                        .split('\n')
                        .collect::<Vec<&str>>();
                    packages_string.push(format!(
                        "{} (chocolatey)",
                        choco_output_split[choco_output_split.len() - 1],
                    ));
                }

                packages_string.join(" ")
            }
            _ => {
                // TODO - add other OS
                "".to_owned()
            }
        }
    }
    pub fn get_public_ip(&self) -> String {
        match minreq::get("http://ipinfo.io/ip").send() {
            Ok(response) => response.as_str().unwrap_or("").to_owned(),
            Err(_) => "".to_owned(),
        }
    }
    pub fn get_terminal(&self) -> String {
        if env_exist("TERM_PROGRAM") {
            return match get_env("TERM_PROGRAM").as_str() {
                "iTerm.app" => "iTerm2".to_owned(),
                "Terminal.app" => "Apple Terminal".to_owned(),
                "Hyper" => "HyperTerm".to_owned(),
                "vscode" => "VSCode".to_owned(),
                value => value.to_owned(),
            };
        }
        if env_exist("TERM") {
            let term: String = get_env("TERM");
            if term == "tw52" || term == "tw100" {
                return "TosWin2".to_owned();
            }
        }
        if env_exist("SSH_CONNECTION") {
            return get_env("SSH_TTY");
        }
        if env_exist("WT_SESSION") {
            return "Windows Terminal".to_owned();
        }

        let pids: Vec<u32> =
            if let Ok(pids) = crate::system::pid::get_parent_pids(std::process::id()) {
                pids
            } else {
                return "".to_owned();
            };
        let pids_name: Vec<String> = if let Ok(pids_name) = crate::system::pid::get_pid_names(pids)
        {
            pids_name
        } else {
            return "".to_owned();
        };
        let clean_pid_names: Vec<String> = crate::system::pid::clean_pid_names(pids_name);
        if clean_pid_names.len() != 1 {
            return "".to_owned();
        }
        let terminal_name: String = clean_pid_names[0].clone();

        format!(
            "{}{}",
            &terminal_name[..1].to_uppercase(),
            &terminal_name[1..]
        )
    }
    pub fn get_de(&self) -> (String, String) {
        if std::env::consts::OS == "windows" && env_exist("distro") {
            let system: System = System::default();

            let windows_version: String = system
                .os_version()
                .unwrap()
                .split(' ')
                .collect::<Vec<&str>>()[0]
                .to_owned();
            if windows_version == "10" {
                ("Fluent".to_owned(), "".to_owned())
            } else if windows_version == "8" {
                ("Metro".to_owned(), "".to_owned())
            } else {
                ("Aero".to_owned(), "".to_owned())
            }
        } else if std::env::consts::OS == "macos" {
            ("Aqua".to_owned(), "".to_owned())
        } else {
            let mut de_name: String = "".to_owned();
            if env_exist("DESKTOP_SESSION") && get_env("DESKTOP_SESSION") == "regolith" {
                de_name = "Regolith".to_owned();
            } else if env_exist("XDG_CURRENT_DESKTOP") {
                de_name = get_env("XDG_CURRENT_DESKTOP")
                    .replace("X-", "")
                    .replace("Gnome", "Budgie")
                    .replace("Budgie:GNOME", "Budgie");
            } else if env_exist("DESKTOP_SESSION") {
                de_name = get_env("DESKTOP_SESSION");
            } else if env_exist("GNOME_DESKTOP_SESSION_ID") {
                de_name = "Gnome".to_owned();
            } else if env_exist("MATE_DESKTOP_SESSION_ID") {
                de_name = "Mate".to_owned();
            } else if env_exist("TDE_FULL_SESSION") {
                de_name = "Trinity".to_owned();
            }

            match de_name.as_str() {
                "KDE_SESSION_VERSION" => de_name = "KDE".to_owned(),
                "xfce4" => de_name = "Xfce4".to_owned(),
                "xfce5" => de_name = "Xfce5".to_owned(),
                "xfce" => de_name = "Xfce".to_owned(),
                "mate" => de_name = "Mate".to_owned(),
                "GNOME" => de_name = "Gnome".to_owned(),
                "MUFFIN" => de_name = "Cinnamon".to_owned(),
                &_ => {}
            }
            let mut version: String = "".to_owned();
            match de_name.as_str() {
                "Plasma" | "KDE" => {
                    version = return_str_from_command(Command::new("plasmashell").arg("--version"));
                }
                "Mate" => {
                    version =
                        return_str_from_command(Command::new("mate-session").arg("--version"));
                }
                "Gnome" => {
                    version = return_str_from_command(Command::new("gnome-shell").arg("--version"));
                }
                "Xfce" => {
                    version =
                        return_str_from_command(Command::new("xfce4-session").arg("--version"));
                }
                "Deepin" => {
                    version = return_str_from_command(
                        Command::new("awk")
                            .arg("-F'='")
                            .arg("'/MajorVersion/ {print $2}'")
                            .arg("/etc/os-version"),
                    );
                }
                "Cinnamon" => {
                    version = return_str_from_command(Command::new("cinnamon").arg("--version"));
                }
                "Budgie" => {
                    version =
                        return_str_from_command(Command::new("budgie-desktop").arg("--version"));
                }
                "LXQt" => {
                    version =
                        return_str_from_command(Command::new("lxqt-session").arg("--version"));
                }
                "Lumina" => {
                    version =
                        return_str_from_command(Command::new("lumina-desktop").arg("--version"));
                }
                "Trinity" => {
                    version = return_str_from_command(Command::new("tde-config").arg("--version"));
                }
                "Unity" => {
                    version = return_str_from_command(Command::new("unity").arg("--version"));
                }
                &_ => {}
            }
            version = version
                .replace(['\n', '-'], "")
                .replace([')', '('], "")
                .replace(r#"\""#, "")
                .replace(' ', "");
            version = version
                .chars()
                .filter(|c| c.is_ascii_digit() || c == &'.')
                .collect();

            (de_name, version)

            // todo hide VM if VM == WM
        }
    }
}
