pub mod colored;

use crate::error::FetchInfoError;
#[cfg(feature = "image")]
use image::{GenericImageView, ImageReader};
use std::fmt::Write;
use std::process::Command;
#[cfg(feature = "image")]
use std::{fs::File, io::BufReader};
use unicode_segmentation::UnicodeSegmentation;
#[cfg(feature = "image")]
use viuer::Config as ViuerConfig;

const fn div_mod(dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}

pub fn env_exist(env_var: &str) -> bool {
    std::env::var(env_var).is_ok()
}

pub fn command_exist(program: &str) -> bool {
    which::which(program).is_ok()
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

#[cfg(feature = "image")]
pub fn print_picture(path: &str) {
    let file = match File::open(path) {
        Ok(f) => f,
        Err(error) => FetchInfoError::error_exit(format!(
            "An error occurred while reading the image: {error}"
        )),
    };
    let reader: ImageReader<BufReader<File>> =
        match ImageReader::new(BufReader::new(file)).with_guessed_format() {
            Ok(r) => r,
            Err(error) => FetchInfoError::error_exit(format!(
                "An error occurred while guessing the image format: {error}"
            )),
        };
    let image = match reader.decode() {
        Ok(i) => i,
        Err(error) => FetchInfoError::error_exit(format!(
            "An error occurred while decoding the image: {error}"
        )),
    };

    let dimensions: (u32, u32) = image.dimensions();
    let (width_ratio, height_ratio): (f64, f64) = if dimensions.0 < 44 {
        (1.0, 1.0)
    } else {
        (dimensions.0 as f64 / 44.0, dimensions.1 as f64 / 44.0)
    };
    let ratio: f64 = width_ratio.max(height_ratio);
    let new_width: u32 = (dimensions.0 as f64 / ratio) as u32;

    let config: ViuerConfig = ViuerConfig {
        x: ((47 - new_width) / 2) as u16,
        width: Some(new_width),
        absolute_offset: false,
        ..ViuerConfig::default()
    };
    if let Err(error) = viuer::print(&image, &config) {
        FetchInfoError::error_exit(format!(
            "An error occurred while printing the image: {error}",
        ))
    }
    println!();
}
