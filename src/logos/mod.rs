mod alma_linux;
mod alpine;
mod arch_linux;
mod aurora;
mod bazzite;
mod black_arch;
mod cachy;
mod calculate_linux;
mod cent_os;
mod computer;
mod debian;
mod deepin;
mod elementary_os;
mod endeavour;
mod fedora;
mod freebsd;
mod garuda;
mod gentoo;
mod ka_os;
mod kali_linux;
mod kde_neon;
mod kubuntu;
mod linux;
mod linux_mint;
mod lubuntu;
mod mac_os;
mod mageia;
mod manjaro;
mod mx_linux;
mod nix_os;
mod nobara;
mod omv;
mod open_suse;
mod openbsd;
mod pardus;
mod parrot;
mod pop_os;
mod raspbian;
mod rhel;
mod rocky_linux;
mod slackware;
mod solaris;
mod solus;
mod steam_os;
mod tails;
mod true_nas;
mod ubuntu;
mod ubuntu_mate;
mod venom_linux;
mod void_linux;
mod windows_10;
mod windows_11;
mod windows_7;
mod xubuntu;
mod zorin_os;

pub fn system_logo(force_os: Option<String>) -> (usize, u8, &'static str) {
    let os: String = if let Some(os) = force_os {
        os
    } else {
        #[cfg(target_os = "linux")]
        {
            sysinfo::System::distribution_id()
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
            let windows_version = sysinfo::System::os_version()
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
        "almalinux" => alma_linux::ALMA_LINUX,
        "alpine" => alpine::ALPINE,
        "archlinux" | "arch" => arch_linux::ARCH_LINUX,
        "aurora" => aurora::AURORA,
        "bazzite" => bazzite::BAZZITE,
        "blackarch" => black_arch::BLACK_ARCH,
        "cachyos" => cachy::CACHY,
        "calculate" | "calculate_linux" => calculate_linux::CALCULATE_LINUX,
        "centos" => cent_os::CENTOS,
        "centos_old" => cent_os::CENTOS_OLD,
        "debian" | "debiangnu/linux" => debian::DEBIAN,
        "deepin" => deepin::DEEPIN,
        "elementaryos" => elementary_os::ELEMENTARY_OS,
        "elementaryos_old" => elementary_os::ELEMENTARY_OS_OLD,
        "endeavour" => endeavour::ENDEAVOUR,
        "fedora" | "fedoralinux" => fedora::FEDORA,
        "freebsd" => freebsd::FREEBSD,
        "garuda" => garuda::GARUDA,
        "gentoo" => gentoo::GENTOO,
        "kali" => kali_linux::KALI_LINUX,
        "kaos" => ka_os::KA_OS,
        "kde_neon" | "neon" => kde_neon::KDE_NEON,
        "kubuntu" => kubuntu::KUBUNTU,
        "linux" | "tux" => linux::LINUX,
        "linuxmint" => linux_mint::LINUX_MINT,
        "lubuntu" => lubuntu::LUBUNTU,
        "macos" | "apple" | "osx" => mac_os::MAC_OS,
        "mageia" => mageia::MAGEIA,
        "manjaro" | "manjarolinux" => manjaro::MANJARO,
        "mx_linux" => mx_linux::MX_LINUX,
        "nixos" => nix_os::NIX_OS,
        "nobara" => nobara::NOBARA,
        "omv" | "open_media_vault" => omv::OPEN_MEDIA_VAULT,
        "openbsd" => openbsd::OPENBSD,
        "opensuse" => open_suse::OPEN_SUSE,
        "pardus" => pardus::PARDUS,
        "parrot" => parrot::PARROT,
        "popos" => pop_os::POP_OS,
        "raspbian" => raspbian::RASPBIAN,
        "rhel" => rhel::RHEL,
        "rocky" => rocky_linux::ROCKY_LINUX,
        "slackware" => slackware::SLACKWARE,
        "solaris" => solaris::SOLARIS,
        "solus" => solus::SOLUS,
        "steam_os" => steam_os::STEAM_OS,
        "tails" => tails::TAILS,
        "truenas" | "truenas_core" => true_nas::TRUENAS_CORE,
        "truenas_enterprise" => true_nas::TRUENAS_ENTERPRISE,
        "truenas_scale" => true_nas::TRUENAS_SCALE,
        "ubuntu" => ubuntu::UBUNTU,
        "ubuntumate" => ubuntu_mate::UBUNTU_MATE,
        "venom" | "venom_linux" => venom_linux::VENOM_LINUX,
        "void" | "void_linux" => void_linux::VOID_LINUX,
        "windows10" => windows_10::WINDOWS10,
        "windows11" => windows_11::WINDOWS11,
        "windows7" => windows_7::WINDOWS7,
        "xubuntu" => xubuntu::XUBUNTU,
        "zorinos" => zorin_os::ZORIN_OS,
        _ => computer::COMPUTER,
    }
}
