use afetch::config::{ColorType, Config, LogoStyle, load_config};
use afetch::error::FetchInfosError;
use afetch::logos::get_logo;
use afetch::system::cpu::get_cpu;
use afetch::system::host::get_hostname;
use afetch::system::kernel::get_kernel;
use afetch::system::uptime::get_uptime;
use afetch::system::{InfoFunction, InfoGroup, InfosResult};
use afetch::translations::get_language;
use afetch::util::colored::{ColorWrapper, ColorizeExt};
use afetch::util::count_str_length;
#[cfg(feature = "image")]
use afetch::util::print_picture;
use colored::{Colorize, CustomColor};
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
            "cpu" => Some(get_cpu as InfoFunction),
            "host" => Some(get_hostname as InfoFunction),
            "kernel" => Some(get_kernel as InfoFunction),
            "uptime" => Some(get_uptime as InfoFunction),
            _ => None,
        })
        .collect::<Vec<_>>();

    let results: Vec<InfosResult> = funcs
        .into_par_iter()
        .filter_map(|f| f(languages_func).ok())
        .collect();

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
        ColorType::Rgb { r, g, b } => ColorWrapper::CustomColor(CustomColor::new(r, g, b)),
        ColorType::Ansi(color) => ColorWrapper::Ansi(color),
    };
    let infos_color = match config.colors.infos {
        Some(ColorType::Rgb { r, g, b }) => {
            Some(ColorWrapper::CustomColor(CustomColor::new(r, g, b)))
        }
        Some(ColorType::Ansi(color)) => Some(ColorWrapper::Ansi(color)),
        _ => None,
    };

    let mut output: String = String::default();
    let mut last_char_count = 0;
    let mut i2 = 0;
    for (i, result) in config.entries.iter().enumerate() {
        let mut format_and_write = |infos: Option<&InfoGroup>| {
            let mut default = if let Some(infos) = &infos {
                let mut default = result.value.clone();
                for value in &infos.values {
                    default = default.replace(&format!("{{{}}}", value.field), &value.value);
                }
                default
            } else {
                result.value.repeat(last_char_count)
            };

            if let Some(header) = result.header.as_ref().filter(|s| !s.trim().is_empty()) {
                last_char_count = count_str_length(header)
                    + count_str_length(languages_func("separator"))
                    + count_str_length(&default);

                let header_color = match infos_color {
                    Some(color) => color,
                    None => match logo.as_ref() {
                        Some(color) => ColorWrapper::Ansi(color.1),
                        None => ColorWrapper::Ansi(6),
                    },
                };

                default = format!(
                    "{}{}{}",
                    header.custom_color_wrapper(header_color).bold(),
                    languages_func("separator"),
                    default.custom_color_wrapper(headers_color)
                );
            } else {
                last_char_count = count_str_length(&default);
                default = format!("{}", default.custom_color_wrapper(headers_color));
            }

            #[cfg(feature = "image")]
            if config.logo.style == LogoStyle::Picture {
                writeln!(output, "{}{}", " ".repeat(47), default).ok();
            }

            if let Some((max_length, _, lines)) = &logo {
                if lines.len() > i {
                    writeln!(output, "   {}{}   {}", lines[i], "".white(), default).ok();
                } else {
                    writeln!(output, "{}{}", " ".repeat(*max_length), default).ok();
                }
            }

            #[cfg(not(feature = "image"))]
            if logo.is_none() {
                writeln!(output, "{default}").ok();
            }
        };

        if result.entry == "separator" {
            format_and_write(None);
            continue;
        }

        match &results[i2] {
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
                writeln!(output, "   {}{}   ", logo_line, "".white()).ok();
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
