use afetch::config::{Config, LogoStyle, load_config};
use afetch::error::{ErrorType, FetchInfosError};
use afetch::logos::get_logo;
use afetch::system::battery::get_battery;
use afetch::system::cpu::get_cpu;
use afetch::system::host::get_hostname;
use afetch::system::kernel::get_kernel;
use afetch::system::memory::get_memory;
use afetch::system::uptime::get_uptime;
use afetch::system::{InfoFunction, InfoGroup, InfosResult};
use afetch::translations::get_language;
use afetch::util::colored::{ColorWrapper, ColorizeExt};
use afetch::util::count_str_length;
#[cfg(feature = "image")]
use afetch::util::print_picture;
use colored::Colorize;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelIterator;
use std::fmt::Write;

fn main() -> Result<(), FetchInfosError> {
    let config: Config = load_config()?;
    let languages_func = get_language("auto");

    let funcs = config
        .entries
        .iter()
        .filter_map(|element| match element.entry.as_str() {
            "battery" => Some(get_battery as InfoFunction),
            "cpu" => Some(get_cpu as InfoFunction),
            "host" => Some(get_hostname as InfoFunction),
            "kernel" => Some(get_kernel as InfoFunction),
            "uptime" => Some(get_uptime as InfoFunction),
            "memory" => Some(get_memory as InfoFunction),
            _ => None,
        })
        .collect::<Vec<_>>();

    let results: Vec<Result<InfosResult, FetchInfosError>> =
        funcs.into_par_iter().map(|f| f(languages_func)).collect();

    let logo = if config.logo.status
        && ([LogoStyle::Braille, LogoStyle::File].contains(&config.logo.style)
            || !cfg!(feature = "image"))
        && supports_unicode::on(supports_unicode::Stream::Stdout)
    {
        let logo = match (config.logo.file_path, config.logo.style) {
            (Some(file_path), LogoStyle::File) => {
                let file_content: &str =
                    Box::leak(std::fs::read_to_string(file_path)?.into_boxed_str());
                let max_length = count_str_length(file_content) + 6;
                Some((max_length, 0, file_content))
            }
            _ => get_logo(None)?,
        };
        logo.map(|(max_length, ansi, logo)| (max_length, ansi, logo.lines().collect::<Vec<&str>>()))
    } else {
        None
    };

    let headers_color = match config.colors.headers {
        Some(color) => color,
        None => match logo.as_ref() {
            Some(color) => ColorWrapper::Ansi(color.1),
            None => ColorWrapper::Ansi(6),
        },
    };
    let separators_color = config.colors.separator.unwrap_or(ColorWrapper::Rgb {
        r: 255,
        g: 255,
        b: 255,
    });
    let infos_color = config.colors.infos.unwrap_or(ColorWrapper::Rgb {
        r: 255,
        g: 255,
        b: 255,
    });

    let mut output: String = String::default();
    let mut last_char_count = 0;
    let mut i2 = 0;
    for (i, entry) in config.entries.iter().enumerate() {
        let result = match &results[i2] {
            Ok(result) => result,
            Err(error) => {
                match &error.0 {
                    ErrorType::Missing => eprintln!("Mising information for {}", entry.entry),
                    ErrorType::Error(error) => eprintln!("An error occurred: {error}"),
                }
                i2 += 1;
                continue;
            }
        };

        let mut format_and_write = |infos: Option<&InfoGroup>| {
            let mut default = if let Some(infos) = &infos {
                let mut default = entry.value.clone();
                for value in &infos.values {
                    default = default.replace(&format!("{{{}}}", value.field), &value.value);
                }
                default
            } else {
                entry.value.repeat(last_char_count)
            };

            if entry.entry != "separator" {
                let header = entry
                    .header
                    .as_deref()
                    .unwrap_or_else(|| languages_func(&entry.entry));

                let separator = entry
                    .separator
                    .as_deref()
                    .unwrap_or_else(|| languages_func("_colon_"));

                last_char_count = count_str_length(header)
                    + count_str_length(separator)
                    + count_str_length(&default);

                default = format!(
                    "{}{}{}",
                    header.custom_color_wrapper(headers_color).bold(),
                    separator.custom_color_wrapper(separators_color),
                    default.custom_color_wrapper(infos_color)
                );
            }

            #[cfg(feature = "image")]
            if config.logo.style == LogoStyle::Picture {
                writeln!(output, "{}{}", " ".repeat(47), default).ok();
            }

            if let Some((max_length, _, lines)) = &logo {
                if lines.len() > i {
                    writeln!(
                        output,
                        "   {}{}   {}",
                        lines[i],
                        "".white(),
                        default.custom_color_wrapper(infos_color)
                    )
                    .ok();
                } else {
                    writeln!(
                        output,
                        "{}{}",
                        " ".repeat(*max_length),
                        default.custom_color_wrapper(infos_color)
                    )
                    .ok();
                }
            }

            #[cfg(not(feature = "image"))]
            if logo.is_none() {
                writeln!(output, "{}", default.custom_color_wrapper(infos_color)).ok();
            }
        };

        if entry.entry == "separator" {
            format_and_write(None);
            continue;
        }

        match result {
            InfosResult::Single(single) => format_and_write(Some(single)),
            InfosResult::Several(elements) => elements
                .iter()
                .for_each(|element| format_and_write(Some(element))),
        }

        i2 += 1;
    }

    if let Some((_, _, lines)) = &logo {
        if config.entries.len() < lines.len() {
            for logo_line in &lines[config.entries.len()..] {
                writeln!(output, "   {}{}", logo_line, "".white()).ok();
            }
        }
    }

    #[cfg(feature = "image")]
    if config.logo.style == LogoStyle::Picture {
        print!("\n{}\x1b[{}A", output, config.entries.len());
        print_picture(&config.logo.picture_path.unwrap())?;
        return Ok(());
    }

    println!("\n{output}");

    Ok(())
}
