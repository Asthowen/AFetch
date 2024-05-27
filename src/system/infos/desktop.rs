use crate::config::DesktopEnvironment;
use crate::error::FetchInfosError;
use crate::utils::{
    env_exist, get_file_content_without_lines, return_str_from_command, DBUS_TIMEOUT,
};
use dbus::nonblock::{Proxy, SyncConnection};
use std::env::var;
use std::process::Command;
use std::sync::Arc;

pub async fn get_de(
    config: DesktopEnvironment,
    conn: Arc<SyncConnection>,
) -> Result<Option<(String, Option<String>)>, FetchInfosError> {
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
        if env_exist("DESKTOP_SESSION") && var("DESKTOP_SESSION").unwrap_or_default() == "regolith"
        {
            "Regolith".clone_into(&mut de_name);
        } else if let Ok(var) = var("XDG_CURRENT_DESKTOP") {
            de_name = var
                .replace("X-", "")
                .replace("Gnome", "Budgie")
                .replace("Budgie:GNOME", "Budgie");
        } else if let Ok(var) = var("DESKTOP_SESSION") {
            de_name = var;
        } else if env_exist("GNOME_DESKTOP_SESSION_ID") {
            "Gnome".clone_into(&mut de_name);
        } else if env_exist("MATE_DESKTOP_SESSION_ID") {
            "Mate".clone_into(&mut de_name);
        } else if env_exist("TDE_FULL_SESSION") {
            "Trinity".clone_into(&mut de_name);
        }

        match de_name.as_str() {
            "KDE_SESSION_VERSION" => "KDE".clone_into(&mut de_name),
            "xfce4" => "Xfce4".clone_into(&mut de_name),
            "xfce5" => "Xfce5".clone_into(&mut de_name),
            "xfce" => "Xfce".clone_into(&mut de_name),
            "mate" => "Mate".clone_into(&mut de_name),
            "GNOME" => "Gnome".clone_into(&mut de_name),
            "MUFFIN" => "Cinnamon".clone_into(&mut de_name),
            &_ => {}
        }
        if !config.version {
            return Ok(Some((de_name, None)));
        }

        let mut version: String = match de_name.as_str() {
            "Plasma" | "KDE" => {
                let mut version: String = String::default();

                let proxy = Proxy::new("org.kde.KWin", "/KWin", DBUS_TIMEOUT, conn);
                let (support_info,): (String,) = proxy
                    .method_call("org.kde.KWin", "supportInformation", ())
                    .await?;

                for line in support_info.lines() {
                    if line.contains("KWin version: ") {
                        line.replace("KWin version:", "")
                            .trim()
                            .clone_into(&mut version);
                        break;
                    }
                }

                if version.is_empty() {
                    version =
                        return_str_from_command(Command::new("plasmashell").arg("--version"))?
                            .replace("plasmashell", "");
                }

                version
            }
            "Mate" => return_str_from_command(Command::new("mate-session").arg("--version"))?,
            "Gnome" => return_str_from_command(Command::new("gnome-shell").arg("--version"))?,
            "Xfce" => return_str_from_command(Command::new("xfce4-session").arg("--version"))?,
            "Deepin" => get_file_content_without_lines("/etc/os-version")
                .await?
                .lines()
                .find(|line| line.starts_with("MajorVersion="))
                .map(|line| line.split('=').nth(1).unwrap_or(""))
                .unwrap_or("")
                .to_owned(),
            "Cinnamon" => return_str_from_command(Command::new("cinnamon").arg("--version"))?,
            "Budgie" => return_str_from_command(Command::new("budgie-desktop").arg("--version"))?,
            "LXQt" => return_str_from_command(Command::new("lxqt-session").arg("--version"))?,
            "Lumina" => return_str_from_command(Command::new("lumina-desktop").arg("--version"))?,
            "Trinity" => return_str_from_command(Command::new("tde-config").arg("--version"))?,
            "Unity" => return_str_from_command(Command::new("unity").arg("--version"))?,
            &_ => String::default(),
        };
        version = version
            .replace(['\n', '-'], "")
            .replace([')', '('], "")
            .replace(r#"\""#, "")
            .replace(' ', "");
        version = version
            .matches(|c: char| !c.is_ascii_digit() || c != '.')
            .collect();

        Ok(Some((de_name, Some(version))))

        // todo hide VM if VM == WM
    }
}