use afetch::error::{ErrorType, FetchInfosError};
use afetch::logos::get_logo;
use afetch::system::battery::get_battery;
use afetch::system::cpu::get_cpu;
use afetch::system::host::get_hostname;
use afetch::system::kernel::get_kernel;
use afetch::system::loadavg::get_loadavg;
use afetch::system::memory::get_memory;
use afetch::system::uptime::get_uptime;
use afetch::system::{InfoFunction, InfoGroup, InfoKind, InfosResult};
use afetch::translations::get_language;
use afetch::util::colored::{ColorWrapper, ColorizeExt};
use afetch::util::count_str_length;
#[cfg(feature = "image")]
use afetch::util::print_picture;
use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::fmt::Write;
use supports_unicode::supports_unicode;

use afetch::config::{Config, Entry, LogoStyle, SeparatorSizing, load_config};

fn main() -> Result<(), FetchInfosError> {
    let config: Config = load_config();
    let language_func = get_language(config.language);

    let results: Vec<Result<InfosResult, FetchInfosError>> = config
        .entries
        .par_iter()
        .filter_map(|element| match element {
            Entry::Info { kind, .. } => match kind {
                InfoKind::Battery => Some(get_battery as InfoFunction),
                InfoKind::Cpu => Some(get_cpu as InfoFunction),
                InfoKind::Host => Some(get_hostname as InfoFunction),
                InfoKind::Kernel => Some(get_kernel as InfoFunction),
                InfoKind::Uptime => Some(get_uptime as InfoFunction),
                InfoKind::Memory => Some(get_memory as InfoFunction),
                InfoKind::Loadavg => Some(get_loadavg as InfoFunction),
            },
            _ => None,
        })
        .map(|f| f(language_func))
        .collect();

    let logo = if supports_unicode() {
        match config.logo {
            LogoStyle::Braille { logo } => Some(get_logo(logo.map(str::to_owned))),
            LogoStyle::File { location: path } => {
                let file_content: &str = Box::leak(std::fs::read_to_string(path)?.into_boxed_str());
                let max_length = count_str_length(file_content) + 6;
                Some((max_length, 0, file_content))
            }
            _ => None,
        }
        .map(|(max_length, ansi, logo)| (max_length, ansi, logo.lines().collect::<Vec<&str>>()))
    } else {
        None
    };

    let header_color = match config.colors.header {
        Some(color) => color,
        None => match logo.as_ref() {
            Some(color) => ColorWrapper::Ansi(color.1),
            None => ColorWrapper::Ansi(6),
        },
    };
    let header_separator_color = config.colors.header_separator.unwrap_or(match &logo {
        Some(color) => ColorWrapper::Ansi(color.1),
        None => ColorWrapper::Ansi(6),
    });
    let info_color = config.colors.info.unwrap_or(match &logo {
        Some(color) => ColorWrapper::Ansi(color.1),
        None => ColorWrapper::Ansi(6),
    });
    let separator_color = config.colors.separator.unwrap_or(match &logo {
        Some(color) => ColorWrapper::Ansi(color.1),
        None => ColorWrapper::Ansi(6),
    });

    let mut output: String = String::default();
    let mut last_info_len = 0;
    let mut i2 = 0;
    for (i, entry) in config.entries.iter().enumerate() {
        let mut write_entry = |entry: String| {
            #[cfg(feature = "image")]
            if matches!(config.logo, LogoStyle::Image { .. }) {
                writeln!(output, "{}{}", " ".repeat(47), entry).ok();
            }

            if let Some((logo_width, _, lines)) = &logo {
                if lines.len() > i {
                    writeln!(output, "   {}{}   {}", lines[i], "".white(), entry).ok();
                } else {
                    writeln!(output, "{}{}", " ".repeat(*logo_width), entry).ok();
                }
            }

            #[cfg(not(feature = "image"))]
            if logo.is_none() {
                writeln!(output, "{entry}").ok();
            }
        };

        match entry {
            Entry::Info {
                kind,
                header,
                format: value,
                separator,
                ..
            } => {
                let result = match &results[i2] {
                    Ok(result) => result,
                    Err(error) => {
                        match &error.0 {
                            ErrorType::Missing => {
                                eprintln!("Mising information for {}", kind.default_header())
                            }
                            ErrorType::Error(error) => eprintln!("An error occurred: {error}"),
                        }
                        continue;
                    }
                };

                let mut format_and_write = |infos: &InfoGroup| {
                    let mut formatted_info = (*value).to_owned();
                    for value in &infos.values {
                        formatted_info =
                            formatted_info.replace(&format!("{{{}}}", value.field), &value.value);
                    }

                    last_info_len = count_str_length(header)
                        + count_str_length(separator)
                        + count_str_length(&formatted_info);

                    formatted_info = format!(
                        "{}{}{}",
                        header.custom_color_wrapper(header_color).bold(),
                        separator.custom_color_wrapper(header_separator_color),
                        formatted_info.custom_color_wrapper(info_color)
                    );

                    write_entry(formatted_info);
                };

                match result {
                    InfosResult::Single(single) => format_and_write(single),
                    InfosResult::Several(elements) => elements.iter().for_each(format_and_write),
                }

                i2 += 1;
            }
            Entry::Separator { content, sizing } => {
                let formatted_separator = match sizing {
                    SeparatorSizing::Fixed(_) => content,
                    SeparatorSizing::Dynamic => {
                        &content.chars().cycle().take(last_info_len).collect()
                    }
                };

                write_entry(
                    formatted_separator
                        .custom_color_wrapper(separator_color)
                        .to_string(),
                );
            }
        }
    }

    if let Some((_, _, lines)) = &logo {
        if config.entries.len() < lines.len() {
            for logo_line in &lines[config.entries.len()..] {
                writeln!(output, "   {}{}", logo_line, "".white()).ok();
            }
        }
    }

    #[cfg(feature = "image")]
    if let LogoStyle::Image { location } = config.logo {
        print!("\n{}\x1b[{}A", output, config.entries.len());
        print_picture(location);
        return Ok(());
    }

    println!("\n{output}");

    Ok(())
}
