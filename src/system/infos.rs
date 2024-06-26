use crate::config::Config;
use crate::logos;
use crate::system::pid::get_ppid;
use crate::utils::{
    command_exist, env_exist, get_env, get_file_content, get_file_content_without_lines,
    return_str_from_command,
};
use std::fs::File;
use std::io::BufReader;
use std::io::{BufRead, Read};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Arc;
use sysinfo::System;
use tokio::task;

pub struct Infos {
    pub sysinfo_obj: System,
    pub custom_logo: Option<String>,
    pub home_dir: PathBuf,
    pub config_dir: PathBuf,
    pub local_dir: PathBuf,
}

impl Infos {
    pub async fn init(custom_logo: Option<String>, config: Arc<Config>) -> Self {
        let mut sysinfo_obj = System::new();
        if !config.disabled_entries.contains(&"memory".to_owned()) {
            sysinfo_obj.refresh_memory();
        }
        if !config.disabled_entries.contains(&"cpu".to_owned()) {
            sysinfo_obj.refresh_cpu();

            if !config.disabled_entries.contains(&"cpu-usage".to_owned()) {
                tokio::time::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL).await;
                sysinfo_obj.refresh_cpu_usage();
            }
        }

        Self {
            sysinfo_obj,
            custom_logo,
            home_dir: dirs::home_dir().unwrap(),
            config_dir: dirs::config_dir().unwrap(),
            local_dir: dirs::data_local_dir().unwrap(),
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

    #[cfg(target_family = "unix")]
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
        } else {
            #[cfg(target_os = "linux")]
            {
                self.get_linux_distribution()
                    .to_lowercase()
                    .replace(' ', "")
            }

            #[cfg(target_os = "freebsd")]
            {
                "freebsd".to_owned()
            }

            #[cfg(target_os = "macos")]
            {
                "macos".to_owned()
            }

            #[cfg(target_os = "windows")]
            {
                let windows_version: String = System::os_version()
                    .unwrap_or_default()
                    .split(' ')
                    .collect::<Vec<&str>>()[0]
                    .to_owned();
                format!(
                    "windows{}",
                    if windows_version.is_empty() {
                        "11".to_owned()
                    } else {
                        windows_version
                    }
                )
            }

            #[cfg(not(any(
                target_os = "windows",
                target_os = "macos",
                target_os = "freebsd",
                target_os = "linux"
            )))]
            {
                String::default()
            }
        }
        .replace(' ', "")
        .to_lowercase();

        match os.as_str() {
            "windows11" => Some(logos::windows_11::WINDOWS11),
            "windows10" => Some(logos::windows_10::WINDOWS10),
            "windows7" => Some(logos::windows_7::WINDOWS7),
            "linux" => Some(logos::linux::LINUX),
            "manjaro" | "manjarolinux" => Some(logos::manjaro::MANJARO),
            "ubuntu" => Some(logos::ubuntu::UBUNTU),
            "archlinux" => Some(logos::arch_linux::ARCH_LINUX),
            "gentoo" => Some(logos::gentoo::GENTOO),
            "fedora" | "fedoralinux" => Some(logos::fedora::FEDORA),
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
        #[cfg(target_os = "linux")]
        {
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
                    get_file_content_without_lines("/sys/devices/virtual/dmi/id/product_version")
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

        #[cfg(target_os = "windows")]
        {
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

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            // TODO - add other OS
            String::default()
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
                format!("{} {}", shell_name, get_env("SHELL_VERSION")).replace('\n', "")
            } else {
                let mut shell_version: String = String::default();
                if shell_name == "fish" {
                    shell_version =
                        return_str_from_command(Command::new(shell_path).arg("--version"))
                            .split("fish, version ")
                            .collect::<Vec<&str>>()[1]
                            .replace('\n', "");
                } else if shell_name == "bash" {
                    shell_version = if env_exist("BASH_VERSION") {
                        get_env("BASH_VERSION")
                    } else {
                        return_str_from_command(
                            Command::new(shell_path).arg("-c").arg("echo $BASH_VERSION"),
                        )
                    };
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
                    shell_name.replace('\n', "")
                } else {
                    format!("{} {}", shell_name, shell_version).replace('\n', "")
                }
            };
        }
        String::default()
    }
    pub fn get_screens_resolution(&self) -> String {
        #[cfg(target_os = "linux")]
        {
            let mut resolution: String = String::default();
            if command_exist("xrandr") && env_exist("DISPLAY") && !env_exist("WAYLAND_DISPLAY") {
                let mut last_line: bool = false;
                let mut temp_resolution: Vec<String> = Vec::new();
                for line in
                    return_str_from_command(Command::new("xrandr").arg("--nograb").arg("--current"))
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
                                .next()
                                .unwrap_or_default()
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

        #[cfg(target_os = "windows")]
        {
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

        #[cfg(not(any(target_os = "windows", target_os = "linux")))]
        {
            // TODO - add other OS
            String::default()
        }
    }
    fn count_lines_in_output(output: String) -> usize {
        output.lines().count()
    }

    pub async fn get_packages_number(&self) -> String {
        let mut packages_string: Vec<String> = Vec::new();

        #[cfg(target_family = "unix")]
        {
            let package_managers = [
                ("pacman", "pacman", vec!["-Qq", "--color", "never"]),
                ("kiss", "kiss", vec!["l"]),
                ("cpt", "cpt-list", Vec::new()),
                ("dpkg", "dpkg-query", vec!["-f", "'.\n'", "-W"]),
                ("xbps-query", "xbps-query", vec!["-l"]),
                ("apk", "apk", vec!["info"]),
                ("opkg", "opkg", vec!["list-installed"]),
                ("pacman-g2", "pacman-g2", vec!["-Q"]),
                ("lvu", "lvu", vec!["installed"]),
                ("tce", "tce-status", vec!["-i"]),
                ("pkg", "pkg_info", Vec::new()),
                ("pkg", "pkg", vec!["info"]),
                ("pkgin", "pkgin", vec!["list"]),
                (
                    "tazpkg",
                    "",
                    vec!["pkgs_h=6", "tazpkg", "list", "&&", "((packages-=6))"],
                ),
                ("sorcery", "gaze", vec!["installed"]),
                ("alps", "alps", vec!["showinstalled"]),
                ("butch", "butch", vec!["list"]),
                ("swupd", "swupd", vec!["bundle-list", "--quiet"]),
                ("pisi", "pisi", vec!["li"]),
                ("pacstall", "pacstall", vec!["-L"]),
                ("flatpak", "flatpak", vec!["list"]),
                ("spm", "spm", vec!["list", "-i"]),
                ("snap", "snap", vec!["list"]),
                ("snap", "mine", vec!["-q"]),
            ];

            let mut handles = Vec::new();
            for (name, command, args) in package_managers {
                if command_exist(command) {
                    let handle = task::spawn(async move {
                        let packages_count: usize = Self::count_lines_in_output(
                            return_str_from_command(Command::new(command).args(args)),
                        );

                        if packages_count != 0 {
                            return Some(format!("{} ({})", packages_count, name));
                        }
                        None
                    });
                    handles.push(handle);
                }
            }

            let handle_rpm = task::spawn(async move {
                if command_exist("dnf")
                    && command_exist("sqlite3")
                    && Path::new("/var/cache/dnf/packages.db").exists()
                {
                    let packages_count = Self::count_lines_in_output(return_str_from_command(
                        Command::new("sqlite3")
                            .arg("/var/cache/dnf/packages.db")
                            .arg(r#""SELECT count(pkg) FROM installed""#),
                    ));
                    if packages_count != 0 {
                        return Some(format!("{} (dnf)", packages_count));
                    }
                } else if command_exist("rpm") {
                    let packages_count = Self::count_lines_in_output(return_str_from_command(
                        Command::new("rpm").arg("-qa"),
                    ));
                    if packages_count != 0 {
                        return Some(format!("{} (dnf)", packages_count));
                    }
                }

                None
            });
            handles.push(handle_rpm);

            for handle in handles {
                match handle.await {
                    Ok(Some(formatted)) => packages_string.push(formatted),
                    Err(error) => {
                        println!("Error while fetching packages number: {}", error);
                    }
                    _ => {}
                }
            }

            packages_string.join(" ")
        }

        #[cfg(target_os = "windows")]
        {
            if command_exist("choco") {
                let choco_output: String =
                    return_str_from_command(Command::new("choco").arg("list").arg("--localonly"));
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

        #[cfg(not(any(target_os = "windows", target_family = "unix")))]
        {
            // TODO - add other OS
            String::default()
        }
    }
    pub fn get_public_ip(&self) -> String {
        match minreq::get("http://ipinfo.io/ip").send() {
            Ok(response) => response.as_str().unwrap_or_default().to_owned(),
            Err(_) => String::default(),
        }
    }

    fn get_qt_bindir_path() -> String {
        let mut path = std::env::var("PATH").unwrap_or_default();
        path.push(':');

        if let Ok(qt_bindir_path) = Command::new("qtpaths").arg("--binaries-dir").output() {
            if let Ok(qt_bindir_output) = String::from_utf8(qt_bindir_path.stdout) {
                path.push_str(qt_bindir_output.trim());
                return path;
            }
        }
        String::default()
    }

    fn get_konsole_instances() -> Vec<String> {
        if let Ok(konsole_instances_output) = Command::new("qdbus")
            .env("PATH", Self::get_qt_bindir_path())
            .output()
        {
            if let Ok(konsole_instances_output_str) =
                String::from_utf8(konsole_instances_output.stdout)
            {
                return konsole_instances_output_str
                    .lines()
                    .filter(|line| {
                        line.contains("org.kde.konsole") || line.contains("org.kde.yakuake")
                    })
                    .map(|line| line.split_whitespace().next().unwrap().to_owned())
                    .collect();
            }
        }
        Vec::new()
    }
    pub fn get_terminal(&self) -> String {
        if env_exist("TERM_PROGRAM") {
            return match get_env("TERM_PROGRAM").trim() {
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
        let pids_names: Vec<String> = match crate::system::pid::get_parent_pid_names() {
            Ok(pids) => pids,
            Err(error) => {
                println!("{error}");
                return String::default();
            }
        };
        let mut term: String = String::default();
        let shell: String = get_env("SHELL");
        for name in pids_names {
            match name.as_str() {
                name if shell == name => {}
                "sh" | "screen" | "su" | "dolphin" | "nautilus" => {}
                "login" | "Login" | "init" | "(init)" => {
                    term = return_str_from_command(&mut Command::new("tty"));
                }
                "ruby" | "1" | "tmux" | "systemd" | "sshd" | "python" | "USER" | "PID"
                | "kdeinit" | "launchd" | "ksmserver" => break,
                _ if name.starts_with("plasma") => break,
                "gnome-terminal-" => term = "gnome-terminal".to_owned(),
                "urxvtd" => term = "urxvt".to_owned(),
                _ if name.contains("nvim") => term = "Neovim Terminal".to_owned(),
                _ if name.contains("NeoVimServer") => term = "VimR Terminal".to_owned(),
                _ => {
                    term = if term.starts_with('.') && term.ends_with("-wrapped") {
                        term.trim_start_matches('.')
                            .trim_end_matches("-wrapped")
                            .to_owned()
                    } else {
                        name.clone()
                    };
                }
            }
        }

        if term.is_empty() {
            return String::default();
        }

        format!("{}{}", &term[..1].to_uppercase(), &term[1..])
    }
    pub fn get_terminal_font(&self) -> String {
        let mut term_font = String::default();

        let terminal_name = if env_exist("TERM") && !get_env("TERM").starts_with("xterm") {
            get_env("TERM")
        } else {
            self.get_terminal()
        };

        if terminal_name.is_empty() {
            return String::default();
        }

        match terminal_name.to_lowercase().as_str() {
            "alacritty" => {
                let mut config_path = Path::new(&self.config_dir)
                    .join("alacritty")
                    .join("alacritty.yml");
                if !config_path.exists() {
                    config_path = Path::new(&self.home_dir).join(".alacritty.yml");
                    if !config_path.exists() {
                        config_path = Path::new(&self.config_dir)
                            .join("alacritty")
                            .join("alacritty.toml");
                        if !config_path.exists() {
                            config_path = Path::new(&self.home_dir).join(".alacritty.toml");
                            if !config_path.exists() {
                                return String::default();
                            }
                        }
                    }
                }

                if let Ok(contents) = std::fs::read_to_string(config_path) {
                    if let Some(line) = contents
                        .lines()
                        .find(|line| line.contains("family:") || line.contains("family = "))
                    {
                        term_font = line
                            .chars()
                            .skip_while(|c| c != &'\"')
                            .skip(1)
                            .take_while(|c| c != &'\"')
                            .collect();
                    }
                }
            }
            "apple_terminal" => {
                term_font = return_str_from_command(
                    Command::new("osascript")
                        .arg("-e")
                        .arg(r#"tell application "Terminal" to font name of window frontmost"#),
                );
            }
            "iterm2" => {
                let current_profile_name = return_str_from_command(Command::new("osascript")
                        .arg("-e")
                        .arg(r#"tell application "iTerm2" to profile name of current session of current window"#)).trim().to_owned();

                let font_file = Path::new(&self.home_dir)
                    .join("Library")
                    .join("Preferences")
                    .join("com.googlecode.iterm2.plist");

                let profiles_count = return_str_from_command(Command::new("PlistBuddy").args([
                    "-c",
                    "Print ':New Bookmarks:'",
                    &font_file.display().to_string(),
                ]))
                .split("Guid")
                .count()
                    - 1;

                for i in 0..profiles_count {
                    let profile_name = return_str_from_command(Command::new("PlistBuddy").args([
                        "-c",
                        &format!("Print ':New Bookmarks:{}:Name:'", i),
                        &font_file.display().to_string(),
                    ]))
                    .trim()
                    .to_owned();

                    if profile_name == current_profile_name {
                        let temp_term_font: String =
                            return_str_from_command(Command::new("PlistBuddy").args([
                                "-c",
                                &format!("Print ':New Bookmarks:{}:Normal Font:'", i),
                                &font_file.display().to_string(),
                            ]))
                            .trim()
                            .to_owned();

                        let diff_font: String =
                            return_str_from_command(Command::new("PlistBuddy").args([
                                "-c",
                                &format!("Print ':New Bookmarks:{}:Use Non-ASCII Font:'", i),
                                &font_file.display().to_string(),
                            ]))
                            .trim()
                            .to_owned();

                        if diff_font == "true" {
                            let non_ascii: String =
                                return_str_from_command(Command::new("PlistBuddy").args([
                                    "-c",
                                    &format!("Print ':New Bookmarks:{}:Non Ascii Font:'", i),
                                    &font_file.display().to_string(),
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
                let config_file = Path::new(&self.config_dir)
                    .join("deepin")
                    .join("deepin-terminal")
                    .join("config.conf");
                if !config_file.exists() {
                    return String::default();
                }

                let mut is_next = false;
                for line in get_file_content(config_file).lines() {
                    if line.contains("[basic.interface.font]") {
                        is_next = true;
                    } else if is_next && line.contains("value=") {
                        term_font.push_str(line.split('=').nth(1).unwrap_or_default().trim());
                        break;
                    }
                }
            }
            "gnustep_terminal" => {
                let config_file = Path::new(&self.home_dir)
                    .join("GNUstep")
                    .join("Defaults")
                    .join("Terminal.plist");
                if !config_file.exists() {
                    return String::default();
                }

                let file_content = get_file_content_without_lines(config_file);
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
                let config_file = Path::new(&self.home_dir).join(".hyper.js");
                if !config_file.exists() {
                    return String::default();
                }

                let file_content = get_file_content_without_lines(config_file);

                let temp_term_font: Option<&str> = match file_content.split("fontFamily\":").nth(1)
                {
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

                let konsole_instances = Self::get_konsole_instances();

                let instance_infos = konsole_instances.iter().find_map(|i| {
                    let konsole_sessions = Command::new("qdbus").arg(i).output().ok().map_or_else(
                        Vec::default,
                        |output| {
                            String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .filter(|line| line.contains("/Sessions/"))
                                .map(ToOwned::to_owned)
                                .collect::<Vec<String>>()
                        },
                    );

                    konsole_sessions.iter().find_map(|session| {
                        let session_process_id = Command::new("qdbus")
                            .arg(i)
                            .arg(session)
                            .arg("processId")
                            .output()
                            .ok()
                            .map(|output| String::from_utf8_lossy(&output.stdout).trim().to_owned())
                            .unwrap_or_default();

                        if child == session_process_id {
                            Some((session.clone(), i.clone()))
                        } else {
                            None
                        }
                    })
                });
                let instance_infos = match instance_infos {
                    None => return String::default(),
                    Some(instance_infos) => instance_infos,
                };

                let mut profile_name = Command::new("qdbus")
                    .arg(&instance_infos.1)
                    .arg(&instance_infos.0)
                    .arg("profile")
                    .output()
                    .ok()
                    .map_or_else(String::default, |output| {
                        String::from_utf8_lossy(&output.stdout).trim().to_owned()
                    });

                if profile_name.is_empty() {
                    profile_name = Command::new("qdbus")
                        .arg(instance_infos.1)
                        .arg(instance_infos.0)
                        .arg("environment")
                        .output()
                        .ok()
                        .map_or_else(String::default, |output| {
                            String::from_utf8_lossy(&output.stdout)
                                .lines()
                                .find_map(|line| {
                                    if line.starts_with("KONSOLE_PROFILE_NAME=") {
                                        Some(
                                            line.trim_start_matches("KONSOLE_PROFILE_NAME=")
                                                .to_owned(),
                                        )
                                    } else {
                                        None
                                    }
                                })
                                .unwrap_or_default()
                        });
                }

                if profile_name.is_empty() {
                    return String::default();
                }

                if profile_name == "Built-in" {
                    return "Monospace".to_owned();
                }

                let konsole_directory = Path::new(&self.local_dir).join("konsole");
                if !konsole_directory.exists() {
                    return String::default();
                }
                let profile_filename = std::fs::read_dir(konsole_directory)
                    .expect("Failed to read profile directory")
                    .filter_map(|entry| {
                        entry.ok().and_then(|e| {
                            let path = e.path();
                            if path.extension().map_or(false, |ext| ext == "profile") {
                                Some(path)
                            } else {
                                None
                            }
                        })
                    })
                    .find(|path| {
                        let mut contents = String::new();
                        File::open(path)
                            .and_then(|mut file| file.read_to_string(&mut contents))
                            .map(|_| contents.contains(&format!("Name={}", profile_name)))
                            .unwrap_or(false)
                    })
                    .unwrap_or_default();

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

        term_font.replace('\n', "")
    }
    pub fn get_de(&self) -> (String, String) {
        #[cfg(target_os = "windows")]
        {
            let windows_version: String = System::os_version()
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
                    if command_exist("qdbus") {
                        let file_to_parse: String = return_str_from_command(
                            Command::new("qdbus")
                                .arg("org.kde.KWin")
                                .arg("/KWin")
                                .arg("supportInformation"),
                        );
                        for line in file_to_parse.lines() {
                            if line.contains("KWin version: ") {
                                version = line.replace("KWin version:", "").trim().to_owned();
                                break;
                            }
                        }
                    }
                    if version.is_empty() {
                        version =
                            return_str_from_command(Command::new("plasmashell").arg("--version"))
                                .replace("plasmashell", "");
                    }
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

    pub fn get_gpus(&self) -> Vec<String> {
        #[cfg(target_os = "macos")]
        return Vec::default();

        #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
        {
            let gpu_cmd: String = match Command::new("lspci").args(["-mm"]).output() {
                Ok(output) => String::from_utf8_lossy(&output.stdout).to_string(),
                Err(_) => return Vec::default(),
            };
            let mut gpus: Vec<String> = Vec::new();
            for line in gpu_cmd.lines().filter(|line| {
                line.contains("Display") || line.contains("3D") || line.contains("VGA")
            }) {
                let parts: Vec<&str> = line
                    .split(|c| c == '"' || c == '(' || c == ')')
                    .filter(|&s| !s.trim().is_empty())
                    .map(|s| s.trim())
                    .collect();
                let gpu: String = format!(
                    "{}{}",
                    parts[2].trim(),
                    if parts[3].is_empty() {
                        String::default()
                    } else {
                        format!(" {}", parts[3])
                    },
                );

                if !gpus.contains(&gpu) {
                    gpus.push(gpu);
                }
            }

            if gpus.first().map_or(false, |gpu| gpu.contains("Intel"))
                && gpus.get(1).map_or(false, |gpu| gpu.contains("Intel"))
            {
                gpus.remove(0);
            }

            let mut gpus_clean: Vec<String> = Vec::new();
            for gpu in gpus {
                gpus_clean.push(match &*gpu {
                    gpu if gpu.contains("Advanced") => {
                        let mut gpu: String = gpu.to_owned();
                        if let Some(start_index) = gpu.find(']') {
                            let temp = &gpu[start_index + 1..];
                            if let Some(end_index) = temp.find('[') {
                                gpu = format!("{}{}", &gpu[0..start_index + 1], &temp[end_index..]);
                            }
                        }
                        gpu = gpu
                            .replace("[AMD/ATI]", "AMD ATI ")
                            .replace("[AMD]", "AMD ")
                            .replace("OEM ", "")
                            .replace("Advanced Micro Devices, Inc.", "");

                        if let Some(start_index) = gpu.find('[') {
                            if let Some(end_index) = gpu.find(']') {
                                gpu = format!(
                                    "{}{}",
                                    &gpu[..start_index],
                                    &gpu[start_index + 1..end_index]
                                );
                            }
                        }
                        gpu.trim().to_owned()
                    }
                    gpu if gpu.contains("NVIDIA") => format!(
                        "NVIDIA {}",
                        gpu.split('[').collect::<Vec<&str>>()[1].replace(']', "")
                    ),
                    gpu if gpu.contains("Intel") => {
                        let gpu: String = gpu
                            .replace("(R)", "")
                            .replace("Corporation ", "")
                            .split(" (")
                            .next()
                            .unwrap()
                            .to_owned()
                            .trim()
                            .to_owned();
                        if gpu.is_empty() {
                            "Intel Integrated Graphics".to_owned()
                        } else {
                            gpu
                        }
                    }
                    gpu if gpu.contains("MCST") => gpu.replace("MCST MGA2", "").to_owned(),
                    gpu if gpu.contains("VirtualBox") => "VirtualBox Graphics Adapter".to_owned(),
                    gpu => gpu.trim().to_owned(),
                });
            }
            gpus_clean
        }

        #[cfg(target_os = "windows")]
        {
            let mut gpus: Vec<String> = Vec::new();

            let wmic_output_result = std::process::Command::new("wmic")
                .args(&["path", "Win32_VideoController", "get", "caption"])
                .output();
            let wmic_output = if let Ok(wmic_output) = wmic_output_result {
                wmic_output
            } else {
                return Vec::new();
            };

            let output_lines = String::from_utf8_lossy(&wmic_output.stdout);
            let mut lines = output_lines.lines();

            while let Some(line) = lines.next() {
                let line: String = line.replace('\n', "").trim().to_owned();
                if line.is_empty() || line == "Caption" {
                    continue;
                }
                gpus.push(line);
            }

            return gpus;
        }
    }
}
