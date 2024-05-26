use crate::error::FetchInfosError;
use crate::system::infos::terminal::get_terminal;
use crate::system::pid::get_ppid;
use crate::utils::{get_file_content, get_file_content_without_lines, return_str_from_command};
use std::env::var;
use std::path::{Path, PathBuf};
use std::process::Command;

pub async fn get_alacritty_font(
    home_dir: &PathBuf,
    config_dir: &PathBuf,
) -> Result<Option<String>, FetchInfosError> {
    let mut config_path = Path::new(&config_dir)
        .join("alacritty")
        .join("alacritty.yml");
    if !config_path.exists() {
        config_path = Path::new(&home_dir).join(".alacritty.yml");
        if !config_path.exists() {
            config_path = Path::new(&config_dir)
                .join("alacritty")
                .join("alacritty.toml");
            if !config_path.exists() {
                config_path = Path::new(&home_dir).join(".alacritty.toml");
                if !config_path.exists() {
                    return Ok(None);
                }
            }
        }
    }

    if let Ok(contents) = get_file_content(config_path).await {
        if let Some(line) = contents
            .lines()
            .find(|line| line.contains("family:") || line.contains("family = "))
        {
            return Ok(Some(
                line.chars()
                    .skip_while(|c| c != &'\"')
                    .skip(1)
                    .take_while(|c| c != &'\"')
                    .collect(),
            ));
        }
    }

    Ok(None)
}

pub fn get_iterm2_font(home_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let current_profile_name =
        return_str_from_command(Command::new("osascript").arg("-e").arg(
            r#"tell application "iTerm2" to profile name of current session of current window"#,
        ))?
        .trim()
        .to_owned();

    let font_file = Path::new(&home_dir)
        .join("Library")
        .join("Preferences")
        .join("com.googlecode.iterm2.plist");

    let profiles_count = return_str_from_command(Command::new("PlistBuddy").args([
        "-c",
        "Print ':New Bookmarks:'",
        &font_file.display().to_string(),
    ]))?
    .split("Guid")
    .count()
        - 1;

    for i in 0..profiles_count {
        let profile_name = return_str_from_command(Command::new("PlistBuddy").args([
            "-c",
            &format!("Print ':New Bookmarks:{}:Name:'", i),
            &font_file.display().to_string(),
        ]))?
        .trim()
        .to_owned();

        if profile_name != current_profile_name {
            continue;
        }

        let temp_term_font: String = return_str_from_command(Command::new("PlistBuddy").args([
            "-c",
            &format!("Print ':New Bookmarks:{}:Normal Font:'", i),
            &font_file.display().to_string(),
        ]))?
        .trim()
        .to_owned();

        let diff_font: String = return_str_from_command(Command::new("PlistBuddy").args([
            "-c",
            &format!("Print ':New Bookmarks:{}:Use Non-ASCII Font:'", i),
            &font_file.display().to_string(),
        ]))?
        .trim()
        .to_owned();

        if diff_font != "true" {
            continue;
        }

        let non_ascii: String = return_str_from_command(Command::new("PlistBuddy").args([
            "-c",
            &format!("Print ':New Bookmarks:{}:Non Ascii Font:'", i),
            &font_file.display().to_string(),
        ]))?
        .trim()
        .to_owned();

        if temp_term_font != non_ascii {
            return Ok(Some(format!(
                "{} (normal) / {} (non-ascii)",
                temp_term_font, non_ascii
            )));
        }
    }
    Ok(None)
}

pub async fn get_deepin_font(config_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let config_file = Path::new(&config_dir)
        .join("deepin")
        .join("deepin-terminal")
        .join("config.conf");
    if !config_file.exists() {
        return Ok(None);
    }

    let mut is_next = false;
    for line in get_file_content(config_file).await?.lines() {
        if line.contains("[basic.interface.font]") {
            is_next = true;
        } else if is_next && line.contains("value=") {
            return Ok(Some(
                line.split('=').nth(1).unwrap_or_default().trim().to_owned(),
            ));
        }
    }

    Ok(None)
}

pub async fn get_gnustep_font(home_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let config_file = Path::new(&home_dir)
        .join("GNUstep")
        .join("Defaults")
        .join("Terminal.plist");
    if !config_file.exists() {
        return Ok(None);
    }

    let file_content = get_file_content_without_lines(config_file).await?;
    Ok(Some(
        file_content
            .lines()
            .filter(|line| line.contains("TerminalFont") || line.contains("TerminalFontSize"))
            .map(|line| line.trim_matches(|c| c == '<' || c == '>' || c == '/'))
            .collect::<Vec<&str>>()
            .join(" "),
    ))
}

pub async fn get_hyper_font(home_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let config_file = Path::new(&home_dir)
        .join("GNUstep")
        .join("Defaults")
        .join("Terminal.plist");
    if !config_file.exists() {
        return Ok(None);
    }

    let file_content = get_file_content_without_lines(config_file).await?;

    Ok(Some(
        file_content
            .lines()
            .filter(|line| line.contains("TerminalFont") || line.contains("TerminalFontSize"))
            .map(|line| line.trim_matches(|c| c == '<' || c == '>' || c == '/'))
            .collect::<Vec<&str>>()
            .join(" "),
    ))
}

fn get_qt_bindir_path() -> String {
    let mut path = var("PATH").unwrap_or_default();
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
    println!("{}", get_qt_bindir_path());
    if let Ok(konsole_instances_output) = Command::new("qdbus")
        .env("PATH", get_qt_bindir_path())
        .output()
    {
        if let Ok(konsole_instances_output_str) = String::from_utf8(konsole_instances_output.stdout)
        {
            return konsole_instances_output_str
                .lines()
                .filter(|line| line.contains("org.kde.konsole") || line.contains("org.kde.yakuake"))
                .map(|line| line.split_whitespace().next().unwrap().to_owned())
                .collect();
        }
    }
    Vec::new()
}

pub async fn get_konsole_font(local_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let child = get_ppid(&format!("{}", std::process::id()))
        .await
        .unwrap_or_default();

    let konsole_instances = get_konsole_instances();
    println!("{:?}", konsole_instances);

    let instance_infos = konsole_instances.iter().find_map(|i| {
        let konsole_sessions =
            Command::new("qdbus")
                .arg(i)
                .output()
                .ok()
                .map_or_else(Vec::default, |output| {
                    String::from_utf8_lossy(&output.stdout)
                        .lines()
                        .filter(|line| line.contains("/Sessions/"))
                        .map(ToOwned::to_owned)
                        .collect::<Vec<String>>()
                });

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
        None => return Ok(None),
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
                            Some(line.trim_start_matches("KONSOLE_PROFILE_NAME=").to_owned())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default()
            });
    }

    if profile_name.is_empty() {
        return Ok(None);
    }

    if profile_name == "Built-in" {
        return Ok(Some("Monospace".to_owned()));
    }

    let konsole_directory = Path::new(&local_dir).join("konsole");
    if !konsole_directory.exists() {
        return Ok(None);
    }
    let mut read_dir = tokio::fs::read_dir(konsole_directory).await?;

    let mut profile_filename: Option<PathBuf> = None;
    while let Some(entry) = read_dir.next_entry().await? {
        let path = entry.path();
        if path.extension().map_or(false, |ext| ext == "profile") {
            let file_content: String = if let Ok(file) = get_file_content(&path).await {
                file
            } else {
                continue;
            };

            if file_content.contains(&format!("Name={}", profile_name)) {
                profile_filename = Some(path);
                break;
            }
        }
    }

    let profile_filename = match profile_filename {
        None => return Ok(None),
        Some(profile_filename) => profile_filename,
    };

    for line in get_file_content(&profile_filename).await?.lines() {
        if line.starts_with("Font=") {
            let fields: Vec<&str> = line.split('=').collect();
            if let Some(font) = fields.get(1) {
                let font_fields: Vec<&str> = font.split(',').collect();
                if let Some(font_name) = font_fields.first() {
                    return Ok(Some(font_name.trim().to_owned()));
                }
            }
        }
    }

    Ok(None)
}

pub async fn get_terminal_font() -> Result<Option<String>, FetchInfosError> {
    let mut term_font: Option<String> = None;
    let home_dir = dirs::home_dir().unwrap();
    let config_dir = dirs::config_dir().unwrap();
    let local_dir = dirs::data_local_dir().unwrap();

    let terminal_name: String = get_terminal().await?.unwrap();

    match terminal_name.to_lowercase().as_str() {
        "alacritty" => {
            term_font = get_alacritty_font(&home_dir, &config_dir).await?;
        }
        "apple_terminal" => {
            term_font = Some(return_str_from_command(
                Command::new("osascript")
                    .arg("-e")
                    .arg(r#"tell application "Terminal" to font name of window frontmost"#),
            )?);
        }
        "iterm2" => {
            term_font = get_iterm2_font(&home_dir)?;
        }
        "deepin-terminal" => {
            term_font = get_deepin_font(&config_dir).await?;
        }
        "gnustep_terminal" => {
            term_font = get_gnustep_font(&home_dir).await?;
        }
        "hyper" => {
            term_font = get_hyper_font(&home_dir).await?;
        }
        "kitty" | "xterm-kitty" => {
            term_font = Some(return_str_from_command(
                Command::new("kitty").arg("+runpy").arg(
                    "from kitty.cli import *; o = create_default_opts(); \
                print(f'{o.font_family} {o.font_size}')",
                ),
            )?);
        }
        "konsole" | "yakuake" => {
            term_font = get_konsole_font(&local_dir).await?;
        }
        &_ => {}
    }

    Ok(term_font.map(|font| font.replace('\n', "")))
}
