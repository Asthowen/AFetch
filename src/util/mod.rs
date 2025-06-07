pub mod colored;

use crate::error::FetchInfosError;
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

pub fn str_from_command(command: &mut Command) -> Result<String, FetchInfosError> {
    Ok(String::from_utf8_lossy(&command.output()?.stdout).to_string())
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
    let mut time_formatted = String::new();

    let mut append_time_part = |value, singular, plural| match value {
        1 => write!(time_formatted, "{value} {singular}, ").unwrap(),
        _ if value > 0 => write!(time_formatted, "{value} {plural}, ").unwrap(),
        _ => {}
    };

    append_time_part(days, languages_func("day"), languages_func("days"));
    append_time_part(hours, languages_func("hour"), languages_func("hours"));
    append_time_part(minutes, languages_func("minute"), languages_func("minutes"));

    if seconds > 0 && minutes == 0 && hours == 0 {
        append_time_part(seconds, languages_func("second"), languages_func("seconds"));
    }

    if time_formatted.is_empty() {
        None
    } else {
        time_formatted.pop();
        time_formatted.pop();
        Some(time_formatted)
    }
}

#[cfg(feature = "image")]
pub fn print_picture(path: &str) -> Result<(), FetchInfosError> {
    let file = File::open(path).map_err(|error| {
        FetchInfosError::error_exit(format!(
            "An error occurred while reading the image: {error}"
        ))
    })?;

    let reader = ImageReader::new(BufReader::new(file))
        .with_guessed_format()
        .map_err(|error| {
            FetchInfosError::error_exit(format!(
                "An error occurred while guessing the image format: {error}"
            ))
        })?;

    let image = reader.decode().map_err(|error| {
        FetchInfosError::error_exit(format!(
            "An error occurred while decoding the image: {error}"
        ))
    })?;

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
    viuer::print(&image, &config).map_err(|error| {
        FetchInfosError::error_exit(format!(
            "An error occurred while printing the image: {error}",
        ))
    })?;
    println!();

    Ok(())
}
