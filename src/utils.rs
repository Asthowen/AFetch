use crate::error::FetchInfosError;
#[cfg(all(unix, not(target_os = "macos")))]
use dbus::nonblock::SyncConnection;
#[cfg(all(unix, not(target_os = "macos")))]
use dbus_tokio::connection;
use std::collections::HashMap;
use std::fmt::Write;
use std::path::Path;
use std::process::Command;
#[cfg(all(unix, not(target_os = "macos")))]
use {std::sync::Arc, std::time::Duration, tokio::sync::OnceCell};

#[cfg(all(unix, not(target_os = "macos")))]
pub const DBUS_TIMEOUT: Duration = Duration::from_secs(5);
#[cfg(all(unix, not(target_os = "macos")))]
pub static CONN: OnceCell<Arc<SyncConnection>> = OnceCell::const_new();

pub const fn div_mod(dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}

pub fn return_str_from_command(command: &mut Command) -> Result<String, FetchInfosError> {
    Ok(String::from_utf8_lossy(&command.output()?.stdout).to_string())
}

pub async fn get_file_content_without_lines(
    file_path: impl AsRef<Path>,
) -> Result<String, FetchInfosError> {
    Ok(tokio::fs::read_to_string(file_path)
        .await?
        .replace('\n', ""))
}
pub async fn get_file_content(file_path: impl AsRef<Path>) -> Result<String, FetchInfosError> {
    Ok(tokio::fs::read_to_string(file_path).await?)
}

pub fn command_exist(program: &str) -> bool {
    which::which(program).is_ok()
}

pub fn format_time(
    time_to_format: u64,
    language: &HashMap<&'static str, &'static str>,
) -> Option<String> {
    let (minutes, seconds): (u64, u64) = div_mod(time_to_format, 60);
    let (hours, minutes): (u64, u64) = div_mod(minutes, 60);
    let (days, hours): (u64, u64) = div_mod(hours, 24);
    let mut time_formatted = String::new();

    let mut append_time_part = |value, singular, plural| match value {
        1 => write!(time_formatted, "{value} {singular}, ").unwrap(),
        _ if value > 0 => write!(time_formatted, "{value} {plural}, ").unwrap(),
        _ => {}
    };

    append_time_part(days, language["day"], language["days"]);
    append_time_part(hours, language["hour"], language["hours"]);
    append_time_part(minutes, language["minute"], language["minutes"]);

    if seconds > 0 && minutes == 0 && hours == 0 {
        append_time_part(seconds, language["second"], language["seconds"]);
    }

    if time_formatted.is_empty() {
        None
    } else {
        time_formatted.pop();
        time_formatted.pop();
        Some(time_formatted)
    }
}

// Based on the human_bytes library of Forkbomb9: https://gitlab.com/forkbomb9/human_bytes-rs
pub fn convert_to_readable_unity<T: Into<f64>>(size: T) -> String {
    const SUFFIX: [&str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let size_converted: f64 = size.into();
    if size_converted <= 0.0_f64 {
        return "0 B".to_owned();
    }
    let base: f64 = size_converted.log10() / 1024_f64.log10();
    let mut result: String = format!("{:.1}", 1024_f64.powf(base - base.floor()))
        .trim_end_matches(".0")
        .to_owned();
    result.push_str(SUFFIX[base.floor() as usize]);
    result
}

pub fn env_exist(env_var: &str) -> bool {
    std::env::var(env_var).is_ok()
}

pub fn count_lines_in_output(output: String) -> usize {
    output.lines().count()
}

#[cfg(all(unix, not(target_os = "macos")))]
pub async fn get_conn() -> Arc<SyncConnection> {
    Arc::clone(
        CONN.get_or_init(|| async {
            let (resource, conn) = connection::new_session_sync().unwrap();
            let _handle = tokio::spawn(async {
                let error = resource.await;
                panic!("Lost connection to D-Bus: {error}");
            });
            conn
        })
        .await,
    )
}
