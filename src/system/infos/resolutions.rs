use crate::error::FetchInfosError;
use crate::utils::{command_exist, env_exist, get_file_content, return_str_from_command};
use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn get_resolutions() -> Result<Option<String>, FetchInfosError> {
    #[cfg(target_os = "linux")]
    {
        if command_exist("xrandr") && env_exist("DISPLAY") && !env_exist("WAYLAND_DISPLAY") {
            let mut last_line: bool = false;
            let mut resolution: Vec<String> = Vec::new();
            for line in
                return_str_from_command(Command::new("xrandr").arg("--nograb").arg("--current"))?
                    .lines()
            {
                if last_line {
                    resolution.push(line.trim().split(' ').collect::<Vec<&str>>()[0].to_owned());
                    last_line = false;
                } else if line.contains(" connected") {
                    last_line = true;
                }
            }
            return Ok(Some(resolution.join(" ")));
        }

        if command_exist("xwininfo") && env_exist("DISPLAY") && !env_exist("WAYLAND_DISPLAY") {
            let command: String = return_str_from_command(Command::new("xwininfo").arg("-root"))?;
            let resolution = format!(
                "{}x{}",
                command.split("Width: ").collect::<Vec<&str>>()[1]
                    .lines()
                    .collect::<Vec<&str>>()[0],
                command.split("Height: ").collect::<Vec<&str>>()[1]
                    .lines()
                    .collect::<Vec<&str>>()[0]
            );
            return Ok(Some(resolution));
        }

        if command_exist("xdpyinfo") && env_exist("DISPLAY") && !env_exist("WAYLAND_DISPLAY") {
            let resolution = return_str_from_command(&mut Command::new("xdpyinfo"))?
                .split("dimensions: ")
                .collect::<Vec<&str>>()[1]
                .trim()
                .split(' ')
                .collect::<Vec<&str>>()[0]
                .to_owned();
            return Ok(Some(resolution));
        }

        if Path::new("/sys/class/drm").exists() {
            let mut resolution: Vec<String> = Vec::new();

            let mut read_dir = if let Ok(read_dir) = tokio::fs::read_dir("/sys/class/drm/").await {
                read_dir
            } else {
                return Ok(None);
            };

            while let Ok(Some(path)) = read_dir.next_entry().await {
                let path: PathBuf = path.path();
                if !path.is_dir() {
                    continue;
                }

                let mut read_sub_dir = if let Ok(read_dir) = tokio::fs::read_dir(path).await {
                    read_dir
                } else {
                    return Ok(None);
                };
                while let Ok(Some(sub_path)) = read_sub_dir.next_entry().await {
                    let sub_path = sub_path.path();
                    if !sub_path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .contains("modes")
                    {
                        continue;
                    }

                    let first_line: String = get_file_content(sub_path)
                        .await?
                        .lines()
                        .next()
                        .unwrap_or_default()
                        .to_owned();

                    if !first_line.is_empty() {
                        resolution.push(first_line);
                    }
                }
            }

            return Ok(Some(resolution.join(", ")));
        }

        Ok(None)
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
        Ok(Some(format!("{}x{}", width, height)))
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        // TODO - add other OS
        Ok(None)
    }
}
