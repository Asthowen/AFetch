#[cfg(any(target_os = "linux", target_os = "windows"))]
use sysinfo::System;

pub mod alpine;
pub mod arch_linux;
pub mod cent_os;
pub mod computer;
pub mod debian;
pub mod deepin;
pub mod elementary_os;
pub mod endeavour;
pub mod fedora;
pub mod freebsd;
pub mod gentoo;
pub mod kde_neon;
pub mod kubuntu;
pub mod linux;
pub mod linux_mint;
pub mod lubuntu;
pub mod mac_os;
pub mod mageia;
pub mod manjaro;
pub mod open_suse;
pub mod pop_os;
pub mod raspbian;
pub mod rhel;
pub mod rocky_linux;
pub mod solaris;
pub mod ubuntu;
pub mod ubuntu_mate;
pub mod windows_10;
pub mod windows_11;
pub mod windows_7;
pub mod xubuntu;
pub mod zorin_os;

pub fn get_logo(force_os: Option<String>) -> (usize, u8, &'static str) {
    let os: String = if let Some(os) = force_os {
        os
    } else {
        #[cfg(target_os = "linux")]
        {
            System::distribution_id()
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
            let windows_version = System::os_version()
                .and_then(|v| v.split_whitespace().next().map(str::to_owned))
                .filter(|v| !v.is_empty())
                .unwrap_or_else(|| "11".to_owned());
            format!("windows{windows_version}")
        }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "linux"
        )))]
        {
            return Ok(computer::COMPUTER);
        }
    }
    .replace(' ', "")
    .to_lowercase();

    match os.as_str() {
        "windows11" => windows_11::WINDOWS11,
        "windows10" => windows_10::WINDOWS10,
        "windows7" => windows_7::WINDOWS7,
        "linux" => linux::LINUX,
        "manjaro" | "manjarolinux" => manjaro::MANJARO,
        "ubuntu" => ubuntu::UBUNTU,
        "archlinux" | "arch" => arch_linux::ARCH_LINUX,
        "gentoo" => gentoo::GENTOO,
        "fedora" | "fedoralinux" => fedora::FEDORA,
        "zorinos" => zorin_os::ZORIN_OS,
        "linuxmint" => linux_mint::LINUX_MINT,
        "macos" | "apple" | "osx" => mac_os::MAC_OS,
        "opensuse" => open_suse::OPEN_SUSE,
        "freebsd" => freebsd::FREEBSD,
        "kubuntu" => kubuntu::KUBUNTU,
        "lubuntu" => lubuntu::LUBUNTU,
        "xubuntu" => xubuntu::XUBUNTU,
        "raspbian" => raspbian::RASPBIAN,
        "popos" => pop_os::POP_OS,
        "endeavour" => endeavour::ENDEAVOUR,
        "centos" => cent_os::CENT_OS,
        "rhel" => rhel::RHEL,
        "mageia" => mageia::MAGEIA,
        "ubuntumate" => ubuntu_mate::UBUNTU_MATE,
        "elementaryos" => elementary_os::ELEMENTARY_OS,
        "elementaryos_old" => elementary_os::ELEMENTARY_OS_OLD,
        "solaris" => solaris::SOLARIS,
        "alpine" => alpine::ALPINE,
        "debian" | "debiangnu/linux" => debian::DEBIAN,
        "deepin" => deepin::DEEPIN,
        "rocky" => rocky_linux::ROCKY_LINUX,
        "kde_neon" | "neon" => kde_neon::KDE_NEON,
        _ => computer::COMPUTER,
    }
}
