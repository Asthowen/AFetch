use crate::logos;
use crate::system::pid::get_ppid;
use crate::utils::{
    command_exist, env_exist, get_env, get_file_content_without_lines, return_str_from_command,
};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use std::process::Command;
use sysinfo::CpuRefreshKind;
use sysinfo::{System, SystemExt};

pub struct Infos {
    pub sysinfo_obj: System,
    pub custom_logo: Option<String>,
    pub home_dir: String,
}

impl Infos {
    pub fn init(custom_logo: Option<String>) -> Self {
        let mut sysinfo_obj = System::new();
        sysinfo_obj.refresh_disks_list();
        sysinfo_obj.refresh_memory();
        sysinfo_obj.refresh_cpu_specifics(CpuRefreshKind::everything());

        Self {
            sysinfo_obj,
            custom_logo,
            home_dir: dirs::home_dir().unwrap().display().to_string(),
        }
    }

    fn parse_os_release(&self, file_path: &str) -> String {
        let contents: String = std::fs::read_to_string(file_path).unwrap_or_default();
        contents
            .lines()
            .find_map(|line| {
                if let Some(("ID", part)) | Some(("NAME", part)) = line.split_once('=') {
                    Some(part.trim_matches('"').to_owned())
                } else {
                    None
                }
            })
            .unwrap_or_default()
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
            get_file_content_without_lines("/etc/SDE-VERSION")
        } else if command_exist("tazpkg") {
            "SliTaz".to_owned()
        } else if command_exist("kpt") && command_exist("kpm") {
            "KSLinux".to_owned()
        } else if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
            "Android".to_owned()
        } else {
            String::default()
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

    pub fn get_os_logo(&self) -> Option<[&str; 2]> {
        let os: String = if let Some(logo) = &self.custom_logo {
            logo.to_owned()
        } else if std::env::consts::OS == "linux" {
            self.get_linux_distribution()
                .to_lowercase()
                .replace(' ', "")
        } else if std::env::consts::OS == "freebsd" {
            "FreeBSD".to_owned()
        } else if std::env::consts::OS == "macos" {
            "MacOS".to_owned()
        } else if std::env::consts::OS == "windows" {
            let windows_version: String = self
                .sysinfo_obj
                .os_version()
                .unwrap_or_default()
                .split(' ')
                .collect::<Vec<&str>>()[0]
                .to_owned();
            format!(
                "Windows{}",
                if !windows_version.is_empty() {
                    windows_version
                } else {
                    String::from("11")
                }
            )
        } else {
            String::default()
        }
        .replace(' ', "");

        match os.as_str() {
            "windows11" => Some(logos::windows_11::WINDOWS11),
            "windows10" => Some(logos::windows_10::WINDOWS10),
            "windows7" => Some(logos::windows_7::WINDOWS7),
            "linux" => Some(logos::linux::LINUX),
            "manjaro" => Some(logos::manjaro::MANJARO),
            "ubuntu" => Some(logos::ubuntu::UBUNTU),
            "archlinux" => Some(logos::arch_linux::ARCH_LINUX),
            "gentoo" => Some(logos::gentoo::GENTOO),
            "fedora" => Some(logos::fedora::FEDORA),
            "zorinos" => Some(logos::zorin_os::ZORIN_OS),
            "linuxmint" => Some(logos::linux_mint::LINUX_MINT),
            "macos" | "apple" | "osx" => Some(logos::mac_os::MAC_OS),
            "opensuse" => Some(logos::open_suse::OPEN_SUSE),
            "freebsd" => Some(logos::freebsd::FREEBSD),
            "kubuntu" => Some(logos::kubuntu::KUBUNTU),
            "lubuntu" => Some(logos::lubuntu::LUBUNTU),
            "xubuntu" => Some(logos::xubuntu::XUBUNTU),
            "raspbian" => Some(logos::raspbian::RASPBIAN),
            "popos" => Some(logos::pop_os::POP_OS),
            "endeavour" => Some(logos::endeavour::ENDEAVOUR),
            "centos" => Some(logos::cent_os::CENT_OS),
            "rhel" => Some(logos::rhel::RHEL),
            "mageia" => Some(logos::mageia::MAGEIA),
            "ubuntumate" => Some(logos::ubuntu_mate::UBUNTU_MATE),
            "elementaryos" => Some(logos::elementary_os::ELEMENTARY_OS),
            "solaris" => Some(logos::solaris::SOLARIS),
            "alpine" => Some(logos::alpine::ALPINE),
            "debian" => Some(logos::debian::DEBIAN),
            _ => None,
        }
    }

    pub fn get_host(&self) -> String {
        let mut host = String::default();
        match std::env::consts::OS {
            "linux" => {
                if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
                    host = format!(
                        "{}{}",
                        return_str_from_command(Command::new("getprop").arg("ro.product.brand")),
                        return_str_from_command(Command::new("getprop").arg("ro.product.model"))
                    );
                } else if Path::new("/sys/devices/virtual/dmi/id/product_name").exists()
                    && Path::new("/sys/devices/virtual/dmi/id/product_version").exists()
                {
                    host = format!(
                        "{} {}",
                        get_file_content_without_lines("/sys/devices/virtual/dmi/id/product_name"),
                        get_file_content_without_lines(
                            "/sys/devices/virtual/dmi/id/product_version"
                        )
                    );
                } else if Path::new("/sys/firmware/devicetree/base/model").exists() {
                    host = get_file_content_without_lines("/sys/firmware/devicetree/base/model");
                } else if Path::new("/tmp/sysinfo/model").exists() {
                    host = get_file_content_without_lines("/tmp/sysinfo/model");
                }

                if (host.contains("System Product Name") || host.is_empty())
                    && Path::new("/sys/devices/virtual/dmi/id/board_vendor").exists()
                    && Path::new("/sys/devices/virtual/dmi/id/board_name").exists()
                {
                    host = format!(
                        "{} {}",
                        get_file_content_without_lines("/sys/devices/virtual/dmi/id/board_vendor"),
                        get_file_content_without_lines("/sys/devices/virtual/dmi/id/board_name")
                            .as_str(),
                    )
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
                String::default()
            }
        }
    }

    pub fn get_shell(&self) -> String {
        let mut shell_path: String = String::default();
        let mut shell_name: String = String::default();

        if env_exist("SHELL") {
            shell_path = get_env("SHELL");
            let shell_name_spliced: Vec<&str> = shell_path.split('/').collect::<Vec<&str>>();
            shell_name = shell_name_spliced[shell_name_spliced.len() - 1].to_owned();
        }

        if !shell_name.is_empty() {
            return if env_exist("SHELL_VERSION") {
                format!("{} {}", shell_name, get_env("SHELL_VERSION"))
            } else {
                let mut shell_version: String = String::default();
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
        String::default()
    }
    pub fn get_screens_resolution(&self) -> String {
        match std::env::consts::OS {
            "linux" => {
                let mut resolution: String = String::default();
                if command_exist("xrandr") && env_exist("DISPLAY") && !env_exist("WAYLAND_DISPLAY")
                {
                    let mut last_line: bool = false;
                    let mut temp_resolution: Vec<String> = Vec::new();
                    for line in return_str_from_command(
                        Command::new("xrandr").arg("--nograb").arg("--current"),
                    )
                    .lines()
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
                    && !env_exist("WAYLAND_DISPLAY")
                {
                    let command: String =
                        return_str_from_command(Command::new("xwininfo").arg("-root"));
                    resolution = format!(
                        "{}x{}",
                        command.split("Width: ").collect::<Vec<&str>>()[1]
                            .lines()
                            .collect::<Vec<&str>>()[0],
                        command.split("Height: ").collect::<Vec<&str>>()[1]
                            .lines()
                            .collect::<Vec<&str>>()[0]
                    );
                } else if command_exist("xdpyinfo")
                    && env_exist("DISPLAY")
                    && !env_exist("WAYLAND_DISPLAY")
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

                    let read_dir = if let Ok(read_dir) = std::fs::read_dir("/sys/class/drm/") {
                        read_dir
                    } else {
                        return String::default();
                    };

                    for path in read_dir.filter_map(Result::ok) {
                        let path: PathBuf = path.path();
                        if !path.is_dir() {
                            continue;
                        }

                        for sub_path in std::fs::read_dir(path).unwrap().filter_map(Result::ok) {
                            let sub_path = sub_path.path();
                            if sub_path
                                .file_name()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .contains("modes")
                            {
                                let first_line: String = std::fs::read_to_string(sub_path)
                                    .unwrap_or_default()
                                    .lines()
                                    .collect::<Vec<&str>>()[0]
                                    .to_owned();

                                if !first_line.is_empty() {
                                    temp_resolution.push(first_line);
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
                String::default()
            }
        }
    }
    fn count_lines_in_output(&self, output: String) -> usize {
        output.lines().count()
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
            #[cfg(target_os = "windows")]
            "windows" => {
                if command_exist("choco") {
                    let choco_output: String = return_str_from_command(
                        Command::new("choco").arg("list").arg("--localonly"),
                    );
                    let choco_output_split: Vec<&str> = choco_output
                        .split(" packages installed")
                        .collect::<Vec<&str>>()[0]
                        .lines()
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
                String::default()
            }
        }
    }
    pub fn get_public_ip(&self) -> String {
        match minreq::get("http://ipinfo.io/ip").send() {
            Ok(response) => response.as_str().unwrap_or_default().to_owned(),
            Err(_) => String::default(),
        }
    }
    pub fn get_terminal(&self) -> String {
        if env_exist("TERM_PROGRAM") {
            return match get_env("TERM_PROGRAM").as_str() {
                "iTerm.app" => "iTerm2".to_owned(),
                "Terminal.app" => "Apple Terminal".to_owned(),
                "Hyper" => "HyperTerm".to_owned(),
                "vscode " => "VSCode".to_owned(),
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
                return String::default();
            };
        let pids_name: Vec<String> = if let Ok(pids_name) = crate::system::pid::get_pid_names(pids)
        {
            pids_name
        } else {
            return String::default();
        };
        let clean_pid_names: Vec<String> = crate::system::pid::clean_pid_names(pids_name);
        if clean_pid_names.len() != 1 {
            return String::default();
        }
        let terminal_name: &str = &clean_pid_names[0];

        format!(
            "{}{}",
            &terminal_name[..1].to_uppercase(),
            &terminal_name[1..]
        )
    }
    pub fn get_terminal_font(&self) -> String {
        let mut term_font = String::default();

        let terminal_name = if env_exist("TERM") && !get_env("TERM").starts_with("xterm") {
            get_env("TERM")
        } else {
            self.get_terminal()
        };

        if !terminal_name.is_empty() {
            match terminal_name.to_lowercase().as_str() {
                "alacritty" => {
                    let xdg_config_home = get_env("XDG_CONFIG_HOME");
                    let confs = vec![
                        format!("{}/alacritty.yml", xdg_config_home),
                        format!("{}/alacritty.yml", self.home_dir),
                        format!("{}/.alacritty.yml", xdg_config_home),
                        format!("{}/.alacritty.yml", self.home_dir),
                    ];

                    for conf in confs {
                        if let Ok(contents) = std::fs::read_to_string(&conf) {
                            if let Some(line) = contents
                                .lines()
                                .find(|line| line.contains("normal:") && line.contains("family:"))
                            {
                                term_font = line
                                    .chars()
                                    .skip_while(|c| c != &'\"')
                                    .skip(1)
                                    .take_while(|c| c != &'\"')
                                    .collect();
                                break;
                            }
                        }
                    }
                }
                "apple_terminal" => {
                    term_font =
                        return_str_from_command(Command::new("osascript").arg("-e").arg(
                            r#"tell application "Terminal" to font name of window frontmost"#,
                        ));
                }
                "iterm2" => {
                    let current_profile_name = return_str_from_command(Command::new("osascript")
                        .arg("-e")
                        .arg(r#"tell application "iTerm2" to profile name of current session of current window"#)).trim().to_owned();

                    let font_file = format!(
                        "{}/Library/Preferences/com.googlecode.iterm2.plist",
                        self.home_dir
                    );

                    let profiles_count =
                        return_str_from_command(Command::new("PlistBuddy").args([
                            "-c",
                            "Print ':New Bookmarks:'",
                            &font_file,
                        ]))
                        .split("Guid")
                        .count()
                            - 1;

                    for i in 0..profiles_count {
                        let profile_name =
                            return_str_from_command(Command::new("PlistBuddy").args([
                                "-c",
                                &format!("Print ':New Bookmarks:{}:Name:'", i),
                                &font_file,
                            ]))
                            .trim()
                            .to_owned();

                        if profile_name == current_profile_name {
                            let temp_term_font: String =
                                return_str_from_command(Command::new("PlistBuddy").args([
                                    "-c",
                                    &format!("Print ':New Bookmarks:{}:Normal Font:'", i),
                                    &font_file,
                                ]))
                                .trim()
                                .to_owned();

                            let diff_font: String =
                                return_str_from_command(Command::new("PlistBuddy").args([
                                    "-c",
                                    &format!("Print ':New Bookmarks:{}:Use Non-ASCII Font:'", i),
                                    &font_file,
                                ]))
                                .trim()
                                .to_owned();

                            if diff_font == "true" {
                                let non_ascii: String =
                                    return_str_from_command(Command::new("PlistBuddy").args([
                                        "-c",
                                        &format!("Print ':New Bookmarks:{}:Non Ascii Font:'", i),
                                        &font_file,
                                    ]))
                                    .trim()
                                    .to_owned();

                                if temp_term_font != non_ascii {
                                    term_font = format!(
                                        "{} (normal) / {} (non-ascii)",
                                        temp_term_font, non_ascii
                                    );
                                }
                            }
                        }
                    }
                }
                "deepin-terminal" => {
                    let config_file = format!(
                        "{}/deepin/deepin-terminal/config.conf",
                        std::env::var("XDG_CONFIG_HOME")
                            .unwrap_or_else(|_| format!("{}/.config", self.home_dir))
                    );

                    for line in get_file_content_without_lines(&config_file).lines() {
                        if line.contains("font=") {
                            term_font.push_str(line.split('=').nth(1).unwrap_or("").trim());
                            term_font.push(' ');
                        }
                        if line.contains("font_size=") {
                            term_font.push_str(line.split('=').nth(1).unwrap_or("").trim());
                            break;
                        }
                    }
                }
                "gnustep_terminal" => {
                    let file_content = get_file_content_without_lines(&format!(
                        "{}/GNUstep/Defaults/Terminal.plist",
                        self.home_dir
                    ));
                    term_font = file_content
                        .lines()
                        .filter(|line| {
                            line.contains("TerminalFont") || line.contains("TerminalFontSize")
                        })
                        .map(|line| line.trim_matches(|c| c == '<' || c == '>' || c == '/'))
                        .collect::<Vec<&str>>()
                        .join(" ");
                }
                "hyper" => {
                    let content =
                        get_file_content_without_lines(&format!("{}/.hyper.js", self.home_dir));

                    let temp_term_font: Option<&str> = match content.split("fontFamily\":").nth(1) {
                        Some(s) => s.split(',').next(),
                        None => None,
                    };

                    term_font = match temp_term_font {
                        Some(s) => s.trim_matches('"').to_owned(),
                        None => String::default(),
                    };
                }
                "kitty" | "xterm-kitty" => {
                    term_font = return_str_from_command(Command::new("kitty").arg("+runpy").arg(
                        "from kitty.cli import *; o = create_default_opts(); \
                print(f'{o.font_family} {o.font_size}')",
                    ));
                }
                "konsole" | "yakuake" => {
                    let child = get_ppid(&format!("{}", std::process::id())).unwrap_or_default();

                    let qt_bindir_result = Command::new("qtpaths").arg("--binaries-dir").output();
                    let qt_bindir = if let Ok(qt_bindir) = qt_bindir_result {
                        qt_bindir
                    } else {
                        return String::default();
                    };
                    let qt_bindir_output = String::from_utf8_lossy(&qt_bindir.stdout);
                    let qt_bindir_path = qt_bindir_output.trim();
                    let mut path = std::env::var("PATH").unwrap_or_default();
                    path.push(':');
                    path.push_str(qt_bindir_path);
                    std::env::set_var("PATH", path);

                    let konsole_instances_output_result = Command::new("qdbus").output();
                    let konsole_instances_output =
                        if let Ok(konsole_instances_output) = konsole_instances_output_result {
                            konsole_instances_output
                        } else {
                            return String::default();
                        };

                    let konsole_instances_output_str =
                        String::from_utf8_lossy(&konsole_instances_output.stdout);
                    let konsole_instances = konsole_instances_output_str
                        .lines()
                        .filter(|line| line.contains(&format!("org.kde.{}", get_env("term"))))
                        .map(|line| line.split_whitespace().next().unwrap())
                        .collect::<Vec<&str>>();

                    let mut profile = String::default();

                    for i in &konsole_instances {
                        let konsole_sessions_output_result = Command::new("qdbus").arg(i).output();
                        let konsole_sessions_output =
                            if let Ok(konsole_sessions_output) = konsole_sessions_output_result {
                                konsole_sessions_output
                            } else {
                                return String::default();
                            };
                        let konsole_sessions_output_str =
                            String::from_utf8_lossy(&konsole_sessions_output.stdout);
                        let konsole_sessions = konsole_sessions_output_str
                            .lines()
                            .filter(|line| line.contains("/Sessions/"))
                            .collect::<Vec<&str>>();

                        for session in &konsole_sessions {
                            let process_id_output_result = Command::new("qdbus")
                                .arg(i)
                                .arg(session)
                                .arg("processId")
                                .output();
                            let process_id_output =
                                if let Ok(process_id_output) = process_id_output_result {
                                    process_id_output
                                } else {
                                    return String::default();
                                };
                            let process_id_output_str =
                                String::from_utf8_lossy(&process_id_output.stdout);
                            let session_process_id = process_id_output_str.trim();

                            if child == session_process_id {
                                let environment_output_result = Command::new("qdbus")
                                    .arg(i)
                                    .arg(session)
                                    .arg("environment")
                                    .output();

                                let environment_output =
                                    if let Ok(environment_output) = environment_output_result {
                                        environment_output
                                    } else {
                                        return String::default();
                                    };

                                let environment_output_str =
                                    String::from_utf8_lossy(&environment_output.stdout);
                                let profile_name = environment_output_str
                                    .lines()
                                    .find(|line| line.starts_with("KONSOLE_PROFILE_NAME="))
                                    .map_or("", |line| {
                                        line.trim_start_matches("KONSOLE_PROFILE_NAME=")
                                    });

                                profile = if profile_name.is_empty() {
                                    let profile_output_result = Command::new("qdbus")
                                        .arg(i)
                                        .arg(session)
                                        .arg("profile")
                                        .output();

                                    let profile_output =
                                        if let Ok(profile_output) = profile_output_result {
                                            profile_output
                                        } else {
                                            return String::default();
                                        };
                                    let profile_output_str =
                                        String::from_utf8_lossy(&profile_output.stdout);
                                    profile_output_str.trim().to_owned()
                                } else {
                                    profile_name.to_owned()
                                };

                                break;
                            }
                        }

                        if !profile.is_empty() {
                            break;
                        }
                    }

                    if profile.is_empty() {
                        return String::default();
                    }

                    if profile == "Built-in" {
                        return "Monospace".to_owned();
                    }

                    let profile_filename_result = std::fs::read_dir(Path::new(&format!(
                        "{}/.local/share/konsole/",
                        dirs::home_dir().unwrap().display()
                    )))
                    .expect("Failed to read profile directory")
                    .filter_map(|entry| {
                        if let Ok(entry) = entry {
                            let path = entry.path();
                            if let Some(file_name) = path.file_name().and_then(|name| name.to_str())
                            {
                                if file_name.ends_with(".profile") {
                                    Some(path)
                                } else {
                                    None
                                }
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .filter(|path| {
                        let file_contents =
                            std::fs::read_to_string(path).expect("Failed to read profile file");
                        file_contents.contains(&format!("Name={}", profile))
                    })
                    .collect::<Vec<_>>();

                    let profile_filename =
                        if let Some(profile_filename) = profile_filename_result.first() {
                            profile_filename
                        } else {
                            return String::default();
                        };
                    let profile_file_result = File::open(profile_filename);
                    let profile_file = if let Ok(profile_file) = profile_file_result {
                        profile_file
                    } else {
                        return String::default();
                    };

                    let reader = BufReader::new(profile_file);

                    for line in reader.lines() {
                        let line = line.unwrap_or_default();

                        if line.starts_with("Font=") {
                            let fields: Vec<&str> = line.split('=').collect();
                            if let Some(font) = fields.get(1) {
                                let font_fields: Vec<&str> = font.split(',').collect();
                                if let Some(font_name) = font_fields.first() {
                                    term_font = font_name.trim().to_owned();
                                    break;
                                }
                            }
                        }
                    }
                }

                &_ => {}
            }
        }

        term_font.replace('\n', "")
    }
    pub fn get_de(&self) -> (String, String) {
        #[cfg(target_os = "windows")]
        if env_exist("distro") {
            let windows_version: String = self
                .sysinfo_obj
                .os_version()
                .unwrap_or_default()
                .split(' ')
                .collect::<Vec<&str>>()[0]
                .to_owned();
            return if windows_version == "10" {
                ("Fluent".to_owned(), String::default())
            } else if windows_version == "8" {
                ("Metro".to_owned(), String::default())
            } else {
                ("Aero".to_owned(), String::default())
            };
        } else {
            return (String::default(), String::default());
        }

        #[cfg(target_os = "macos")]
        return ("Aqua".to_owned(), String::default());

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let mut de_name: String = String::default();
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
            let mut version: String = String::default();
            match de_name.as_str() {
                "Plasma" | "KDE" => {
                    version = return_str_from_command(Command::new("plasmashell").arg("--version"))
                        .replace("plasmashell", "");
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
                    version = get_file_content_without_lines("/etc/os-version")
                        .lines()
                        .find(|line| line.starts_with("MajorVersion="))
                        .map(|line| line.split('=').nth(1).unwrap_or(""))
                        .unwrap_or("")
                        .to_owned();
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
                .matches(|c: char| !c.is_ascii_digit() || c != '.')
                .collect();

            (de_name, version)

            // todo hide VM if VM == WM
        }
    }

    pub fn get_wm(&self) -> String {
        String::default()
    }
}
