pub mod constants;
#[cfg(feature = "image")]
pub mod pictures;

mod filtered_values;

pub(crate) use self::filtered_values::{ToOptionString, filtered_values};

use std::fmt::Write;
use std::process::Command;

use unicode_segmentation::UnicodeSegmentation;

use crate::error::FetchInfoError;

pub const PROJECT_VERSION: &str = env!("CARGO_PKG_VERSION");

const fn div_mod(dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}

pub fn env_var_exists(env_var: &str) -> bool {
    std::env::var(env_var).is_ok()
}

pub fn executable_exists(binary_name: &str) -> bool {
    which::which(binary_name).is_ok()
}

pub fn str_from_command(command: &mut Command) -> Result<String, FetchInfoError> {
    Ok(String::from_utf8_lossy(&command.output()?.stdout).into_owned())
}

pub fn count_str_length(value: &str) -> usize {
    let value_escaped: Vec<u8> = strip_ansi_escapes::strip(value);
    let logo_escape = String::from_utf8_lossy(&value_escaped);
    logo_escape
        .lines()
        .fold(0, |acc, line| acc.max(line.graphemes(true).count()))
}

pub fn format_time(time_to_format: u64, languages_func: fn(&str) -> &str) -> Option<String> {
    let (minutes, seconds): (u64, u64) = div_mod(time_to_format, 60);
    let (hours, minutes): (u64, u64) = div_mod(minutes, 60);
    let (days, hours): (u64, u64) = div_mod(hours, 24);
    let mut time_formatted = String::default();

    let mut append_time_part = |value, singular, plural| match value {
        1 => write!(time_formatted, "{value} {}, ", languages_func(singular)).unwrap(),
        _ if value > 0 => write!(time_formatted, "{value} {}, ", languages_func(plural)).unwrap(),
        _ => {}
    };

    append_time_part(days, "day", "days");
    append_time_part(hours, "hour", "hours");
    append_time_part(minutes, "minute", "minutes");

    if seconds > 0 && minutes == 0 && hours == 0 {
        append_time_part(seconds, "second", "seconds");
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
    format!(
        "{:.1} {}",
        1024_f64.powf(base.fract()),
        SUFFIX[base.floor() as usize]
    )
    .replace(".0", "")
}
