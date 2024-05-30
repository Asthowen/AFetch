use crate::error::FetchInfosError;
use crate::logos;
#[cfg(target_os = "linux")]
use {
    crate::utils::{
        command_exist, env_exist, get_file_content, get_file_content_without_lines,
        return_str_from_command,
    },
    std::env::var,
    std::path::Path,
    std::process::Command,
};

#[cfg(target_os = "windows")]
use sysinfo::System;

#[cfg(target_os = "linux")]
async fn parse_os_release(file_path: &str) -> Result<String, FetchInfosError> {
    let contents: String = get_file_content(file_path).await?;
    Ok(contents
        .lines()
        .find_map(|line| {
            if let Some(("ID", part)) | Some(("NAME", part)) = line.split_once('=') {
                Some(part.trim_matches('"').to_owned())
            } else {
                None
            }
        })
        .unwrap_or_default())
}

#[cfg(target_os = "linux")]
async fn get_linux_distribution() -> Result<String, FetchInfosError> {
    let mut distribution_name: String = if Path::new("/etc/os-release").exists() {
        parse_os_release("/etc/os-release").await?
    } else if Path::new("/usr/lib/os-release").exists() {
        parse_os_release("/usr/lib/os-release").await?
    } else if Path::new("/etc/openwrt_release").exists() {
        parse_os_release("/etc/openwrt_release").await?
    } else if Path::new("/etc/lsb-release").exists() {
        parse_os_release("/etc/lsb-release").await?
    } else if Path::new("/besdrock/etc/bedrock-release").exists() && env_exist("BEDROCK_RESTRICT") {
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
        match var("DISTRO_SHORTHAND").unwrap_or_default().as_str() {
            "on" | "off" => return_str_from_command(Command::new("lsb_release").arg("-si"))?,
            _ => return_str_from_command(Command::new("lsb_release").arg("-sd"))?,
        }
    } else if Path::new("/etc/GoboLinuxVersion").exists() {
        "GoboLinux".to_owned()
    } else if Path::new("/etc/SDE-VERSION").exists() {
        get_file_content_without_lines("/etc/SDE-VERSION").await?
    } else if command_exist("tazpkg") {
        "SliTaz".to_owned()
    } else if command_exist("kpt") && command_exist("kpm") {
        "KSLinux".to_owned()
    } else if Path::new("/system/app/").exists() && Path::new("/system/priv-app").exists() {
        "Android".to_owned()
    } else {
        String::default()
    };

    if distribution_name == "Ubuntu" {
        if let Ok(var) = var("XDG_CONFIG_DIRS") {
            distribution_name = match &*var {
                v if v.contains("cinnamon") => "Ubuntu Cinnamon".to_owned(),
                v if v.contains("studio") => "Ubuntu Studio".to_owned(),
                v if v.contains("plasma") || v.contains("kubuntu") => "Kubuntu".to_owned(),
                v if v.contains("xubuntu") => "Xubuntu".to_owned(),
                v if v.contains("mate") => "Ubuntu Mate".to_owned(),
                v if v.contains("lubuntu") => "Lubuntu".to_owned(),
                v if v.contains("budgie") => "Ubuntu Budgie".to_owned(),
                _ => distribution_name,
            };
        }
    }

    Ok(distribution_name)
}

pub async fn get_os_logo(
    custom_logo: Option<String>,
) -> Result<Option<[&'static str; 2]>, FetchInfosError> {
    let os: String = if let Some(logo) = custom_logo {
        logo
    } else {
        #[cfg(target_os = "linux")]
        {
            get_linux_distribution()
                .await?
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
        "windows11" => Ok(Some(logos::windows_11::WINDOWS11)),
        "windows10" => Ok(Some(logos::windows_10::WINDOWS10)),
        "windows7" => Ok(Some(logos::windows_7::WINDOWS7)),
        "linux" => Ok(Some(logos::linux::LINUX)),
        "manjaro" | "manjarolinux" => Ok(Some(logos::manjaro::MANJARO)),
        "ubuntu" => Ok(Some(logos::ubuntu::UBUNTU)),
        "archlinux" => Ok(Some(logos::arch_linux::ARCH_LINUX)),
        "gentoo" => Ok(Some(logos::gentoo::GENTOO)),
        "fedora" | "fedoralinux" => Ok(Some(logos::fedora::FEDORA)),
        "zorinos" => Ok(Some(logos::zorin_os::ZORIN_OS)),
        "linuxmint" => Ok(Some(logos::linux_mint::LINUX_MINT)),
        "macos" | "apple" | "osx" => Ok(Some(logos::mac_os::MAC_OS)),
        "opensuse" => Ok(Some(logos::open_suse::OPEN_SUSE)),
        "freebsd" => Ok(Some(logos::freebsd::FREEBSD)),
        "kubuntu" => Ok(Some(logos::kubuntu::KUBUNTU)),
        "lubuntu" => Ok(Some(logos::lubuntu::LUBUNTU)),
        "xubuntu" => Ok(Some(logos::xubuntu::XUBUNTU)),
        "raspbian" => Ok(Some(logos::raspbian::RASPBIAN)),
        "popos" => Ok(Some(logos::pop_os::POP_OS)),
        "endeavour" => Ok(Some(logos::endeavour::ENDEAVOUR)),
        "centos" => Ok(Some(logos::cent_os::CENT_OS)),
        "rhel" => Ok(Some(logos::rhel::RHEL)),
        "mageia" => Ok(Some(logos::mageia::MAGEIA)),
        "ubuntumate" => Ok(Some(logos::ubuntu_mate::UBUNTU_MATE)),
        "elementaryos" => Ok(Some(logos::elementary_os::ELEMENTARY_OS)),
        "solaris" => Ok(Some(logos::solaris::SOLARIS)),
        "alpine" => Ok(Some(logos::alpine::ALPINE)),
        "debian" | "debiangnu/linux" => Ok(Some(logos::debian::DEBIAN)),
        _ => Ok(None),
    }
}
