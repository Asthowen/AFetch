use crate::error::FetchInfosError;
use sysinfo::System;

pub mod alpine;
pub mod arch_linux;
pub mod cent_os;
pub mod debian;
pub mod elementary_os;
pub mod endeavour;
pub mod fedora;
pub mod freebsd;
pub mod gentoo;
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
pub mod solaris;
pub mod ubuntu;
pub mod ubuntu_mate;
pub mod windows_10;
pub mod windows_11;
pub mod windows_7;
pub mod xubuntu;
pub mod zorin_os;

pub fn get_logo(
    force_os: Option<String>,
) -> Result<Option<(usize, u8, &'static str)>, FetchInfosError> {
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
            format!("windows{}", windows_version)
        }

        #[cfg(not(any(
            target_os = "windows",
            target_os = "macos",
            target_os = "freebsd",
            target_os = "linux"
        )))]
        {
            return Ok(None);
        }
    }
    .replace(' ', "")
    .to_lowercase();

    match os.as_str() {
        "windows11" => Ok(Some(windows_11::WINDOWS11)),
        "windows10" => Ok(Some(windows_10::WINDOWS10)),
        "windows7" => Ok(Some(windows_7::WINDOWS7)),
        "linux" => Ok(Some(linux::LINUX)),
        "manjaro" | "manjarolinux" => Ok(Some(manjaro::MANJARO)),
        "ubuntu" => Ok(Some(ubuntu::UBUNTU)),
        "archlinux" => Ok(Some(arch_linux::ARCH_LINUX)),
        "gentoo" => Ok(Some(gentoo::GENTOO)),
        "fedora" | "fedoralinux" => Ok(Some(fedora::FEDORA)),
        "zorinos" => Ok(Some(zorin_os::ZORIN_OS)),
        "linuxmint" => Ok(Some(linux_mint::LINUX_MINT)),
        "macos" | "apple" | "osx" => Ok(Some(mac_os::MAC_OS)),
        "opensuse" => Ok(Some(open_suse::OPEN_SUSE)),
        "freebsd" => Ok(Some(freebsd::FREEBSD)),
        "kubuntu" => Ok(Some(kubuntu::KUBUNTU)),
        "lubuntu" => Ok(Some(lubuntu::LUBUNTU)),
        "xubuntu" => Ok(Some(xubuntu::XUBUNTU)),
        "raspbian" => Ok(Some(raspbian::RASPBIAN)),
        "popos" => Ok(Some(pop_os::POP_OS)),
        "endeavour" => Ok(Some(endeavour::ENDEAVOUR)),
        "centos" => Ok(Some(cent_os::CENT_OS)),
        "rhel" => Ok(Some(rhel::RHEL)),
        "mageia" => Ok(Some(mageia::MAGEIA)),
        "ubuntumate" => Ok(Some(ubuntu_mate::UBUNTU_MATE)),
        "elementaryos" => Ok(Some(elementary_os::ELEMENTARY_OS)),
        "solaris" => Ok(Some(solaris::SOLARIS)),
        "alpine" => Ok(Some(alpine::ALPINE)),
        "debian" | "debiangnu/linux" => Ok(Some(debian::DEBIAN)),
        _ => Ok(None),
    }
}
