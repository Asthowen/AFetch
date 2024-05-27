use crate::error::FetchInfosError;
use crate::system::infos::terminal::get_terminal;
use crate::utils::{get_file_content, get_file_content_without_lines, return_str_from_command};
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(all(unix, not(target_os = "macos")))]
use {
    crate::system::pid::get_ppid,
    crate::utils::{get_conn, DBUS_TIMEOUT},
    dbus::nonblock::stdintf::org_freedesktop_dbus::Introspectable,
    dbus::nonblock::Proxy,
};

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

#[cfg(all(unix, not(target_os = "macos")))]
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
    let config_file = Path::new(&home_dir).join(".hyper.js");
    if !config_file.exists() {
        return Ok(None);
    }

    let file_content: String = get_file_content_without_lines(config_file).await?;

    for line in file_content.lines() {
        if let Some(start_index) = line.find("fontFamily") {
            if let Some(start_quote_index) = line[start_index..].find(|c| c == '\'' || c == '"') {
                let start = start_index
                    + start_quote_index
                    + if line.as_bytes()[start_index + start_quote_index + 1] as char == '"' {
                        2
                    } else {
                        1
                    };
                let end_index = start
                    + line[start..]
                        .find(|c| c == '\'' || c == '"' || c == ',')
                        .unwrap_or_else(|| line.len() - start);
                return Ok(Some(line[start..end_index].trim().to_owned()));
            }
        }
    }

    Ok(None)
}

#[cfg(all(unix, not(target_os = "macos")))]
fn extract_node_values(xml: String) -> Vec<String> {
    xml.lines()
        .filter_map(|line| {
            line.split_once("<node name=\"")
                .and_then(|(_, rest)| rest.split_once('"'))
                .map(|(value, _)| format!("/Sessions/{}", value))
        })
        .collect()
}

#[cfg(all(unix, not(target_os = "macos")))]
pub async fn get_konsole_font(local_dir: &PathBuf) -> Result<Option<String>, FetchInfosError> {
    let child = get_ppid(&format!("{}", std::process::id()))
        .await
        .unwrap_or_default();

    let proxy = Proxy::new("org.freedesktop.DBus", "/", DBUS_TIMEOUT, get_conn().await);
    let (names,): (Vec<String>,) = proxy
        .method_call("org.freedesktop.DBus", "ListNames", ())
        .await?;

    let konsole_instances: Vec<String> = names
        .iter()
        .filter(|line| line.contains("org.kde.konsole") || line.contains("org.kde.yakuake"))
        .filter_map(|line| line.split_whitespace().next().map(|s| s.to_owned()))
        .collect();

    let mut instance_infos: Option<(String, String)> = None;
    for instance in konsole_instances {
        let proxy = Proxy::new(&instance, "/Sessions", DBUS_TIMEOUT, get_conn().await);
        let introspect = proxy.introspect().await?;
        let konsole_sessions = extract_node_values(introspect);

        for session in konsole_sessions {
            let proxy = Proxy::new(&instance, &session, DBUS_TIMEOUT, get_conn().await);
            let (session_process_id,): (i32,) = proxy
                .method_call("org.kde.konsole.Session", "processId", ())
                .await?;

            if child == session_process_id.to_string() {
                instance_infos = Some((session, instance));
                break;
            }
        }
        if instance_infos.is_some() {
            break;
        }
    }
    let instance_infos = match instance_infos {
        None => return Ok(None),
        Some(instance_infos) => instance_infos,
    };
    let proxy = Proxy::new(
        &instance_infos.1,
        &instance_infos.0,
        DBUS_TIMEOUT,
        get_conn().await,
    );
    let (profile_name,): (String,) = proxy
        .method_call("org.kde.konsole.Session", "profile", ())
        .await?;

    if profile_name.is_empty() {
        return Ok(None);
    }

    if profile_name == "Built-in" || profile_name == "Intégré" {
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

    let terminal_name: String = get_terminal().await?.unwrap().to_lowercase();

    match terminal_name.as_str() {
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
        "gnustep_terminal" => {
            term_font = get_gnustep_font(&home_dir).await?;
        }
        "hyper" | "hyperterm" => {
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
        &_ => {}
    }

    #[cfg(all(unix, not(target_os = "macos")))]
    {
        let local_dir = dirs::data_local_dir().unwrap();
        match terminal_name.as_str() {
            "konsole" | "yakuake" => {
                term_font = get_konsole_font(&local_dir).await?;
            }
            "deepin-terminal" => {
                term_font = get_deepin_font(&config_dir).await?;
            }
            &_ => {}
        }
    }

    Ok(term_font.map(|font| font.replace('\n', "")))
}
