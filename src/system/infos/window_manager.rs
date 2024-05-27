use crate::error::FetchInfosError;
use crate::utils::env_exist;
use std::env::var;
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub async fn get_window_manager() -> Result<Option<String>, FetchInfosError> {
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

            for process_name in info.processes().values().map(|process| process.name()) {
                for &name in &PROCESS_NAMES {
                    if process_name.contains(name) {
                        return Ok(Some(name.to_owned()));
                    }
                }
            }
        }
    }

    Ok(None)
}
