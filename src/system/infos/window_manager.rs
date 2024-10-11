use crate::error::FetchInfosError;
#[cfg(target_family = "unix")]
use {
    crate::utils::env_exist,
    std::env::var,
    sysinfo::{ProcessRefreshKind, RefreshKind, System},
};
#[cfg(target_family = "windows")]
use {crate::utils::return_str_from_command, std::process::Command};

pub fn get_window_manager() -> Result<Option<String>, FetchInfosError> {
    #[cfg(target_family = "unix")]
    {
        if var("DESKTOP_SESSION")
            .ok()
            .filter(|var| var == "i3")
            .is_some()
        {
            return Ok(Some("i3".to_owned()));
        }
        if env_exist("WAYLAND_DISPLAY") || env_exist("DISPLAY") {
            let info = System::new_with_specifics(
                RefreshKind::new()
                    .with_processes(
                        ProcessRefreshKind::new()
                            .without_cpu()
                            .without_memory()
                            .without_disk_usage()
                            .without_environ()
                            .without_exe()
                            .without_root()
                            .without_cwd()
                            .without_user()
                            .without_cmd(),
                    )
                    .without_memory()
                    .without_cpu(),
            );

            const PROCESS_NAMES: [&str; 36] = [
                "arcan",
                "asc",
                "clayland",
                "dwc",
                "fireplace",
                "gnome-shell",
                "greenfield",
                "grefsen",
                "kwin",
                "lipstick",
                "maynard",
                "mazecompositor",
                "motorcar",
                "orbital",
                "orbment",
                "perceptia",
                "rustland",
                "sway",
                "ulubis",
                "velox",
                "wavy",
                "way-cooler",
                "wayfire",
                "wayhouse",
                "westeros",
                "westford",
                "weston",
                "sowm",
                "catwm",
                "fvwm",
                "dwm",
                "2bwm",
                "monsterwm",
                "tinywm",
                "x11fs",
                "xmonad",
            ];

            for process_name in info
                .processes()
                .values()
                .filter_map(|process| process.name().to_str())
            {
                for &name in &PROCESS_NAMES {
                    if process_name.contains(name) {
                        return Ok(Some(name.to_owned()));
                    }
                }
            }
        }
    }

    #[cfg(target_family = "windows")]
    {
        let wm: String = return_str_from_command(&mut Command::new("tasklist"))?;

        const PATTERNS: [&str; 6] = [
            "komorebi",
            "bugn",
            "Windawesome",
            "blackbox",
            "emerge",
            "litestep0",
        ];

        let wm_name = wm
            .lines()
            .find_map(|line| {
                PATTERNS
                    .iter()
                    .find(|&&pattern| line.contains(pattern))
                    .map(|&s| s.to_string())
            })
            .or(Some("DWM".to_owned()));

        return Ok(wm_name);
    }

    Ok(None)
}
