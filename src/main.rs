use afetch::config::{Config, Entry, LogoStyle, SeparatorSizing, load_config};
use afetch::error::{ErrorType, FetchInfoError};
use afetch::logos::get_logo;
use afetch::system::battery::get_battery;
use afetch::system::cpu::get_cpu;
use afetch::system::disk::get_disk;
use afetch::system::disks::get_disks;
use afetch::system::host::get_hostname;
use afetch::system::kernel::get_kernel;
use afetch::system::loadavg::get_loadavg;
use afetch::system::memory::get_memory;
use afetch::system::uptime::get_uptime;
use afetch::system::{InfoGroup, InfoKind, InfoResult};
use afetch::translations::get_language;
use afetch::util::colored::{ColorWrapper, ColorizeExt};
use afetch::util::count_str_length;
#[cfg(feature = "image")]
use afetch::util::print_picture;
use colored::Colorize;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fmt::Write;
use supports_unicode::supports_unicode;

fn main() -> Result<(), FetchInfoError> {
    let config: Config = load_config();
    let language_func = get_language(config.language);

    let results: HashMap<InfoKind, Result<InfoResult, FetchInfoError>> = config
        .info
        .par_iter()
        .map(|(kind, fields)| {
            (
                *kind,
                match kind {
                    InfoKind::Battery => get_battery(language_func, fields, &config),
                    InfoKind::Cpu => get_cpu(language_func, fields, &config),
                    InfoKind::Disk => get_disk(language_func, fields, &config),
                    InfoKind::Disks => get_disks(language_func, fields, &config),
                    InfoKind::Host => get_hostname(language_func, fields, &config),
                    InfoKind::Kernel => get_kernel(language_func, fields, &config),
                    InfoKind::Uptime => get_uptime(language_func, fields, &config),
                    InfoKind::Memory => get_memory(language_func, fields, &config),
                    InfoKind::Loadavg => get_loadavg(language_func, fields, &config),
                },
            )
        })
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
    let mut i = 0;

    for entry in &config.entries {
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

            i += 1;
        };

        match entry {
            Entry::Info {
                kind,
                header,
                format: value,
                separator,
                ..
            } => {
                let result = match &results[kind] {
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

                let mut format_and_write = |info: &InfoGroup| {
                    let mut formatted_header = (*header).to_owned();
                    let mut formatted_info = (*value).to_owned();
                    for value in &info.values {
                        let placeholder = format!("{{{}}}", value.field);
                        formatted_header = formatted_header.replace(&placeholder, &value.value);
                        formatted_info = formatted_info.replace(&placeholder, &value.value);
                    }

                    last_info_len = count_str_length(&formatted_header)
                        + count_str_length(separator)
                        + count_str_length(&formatted_info);

                    formatted_info = format!(
                        "{}{}{}",
                        formatted_header.custom_color_wrapper(header_color).bold(),
                        separator.custom_color_wrapper(header_separator_color),
                        formatted_info.custom_color_wrapper(info_color)
                    );

                    write_entry(formatted_info);
                };

                match result {
                    InfoResult::Single(single) => format_and_write(single),
                    InfoResult::Several(elements) => elements.iter().for_each(format_and_write),
                }
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
            Entry::ColorBlocks { content, display } => {
                if display.show_normal() {
                    let first_colors: String = (0..8).fold(String::default(), |mut acc, i| {
                        write!(&mut acc, "\x1b[3{i}m{content}\x1b[0m").ok();
                        acc
                    });
                    write_entry(first_colors);
                }
                if display.show_bright() {
                    let second_colors: String = (0..8).fold(String::new(), |mut acc, i| {
                        write!(&mut acc, "\x1b[9{i}m{content}\x1b[0m").unwrap();
                        acc
                    });
                    write_entry(second_colors);
                }
            }
        }
    }

    if let Some((_, _, lines)) = &logo {
        if i < lines.len() {
            for logo_line in &lines[i..] {
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
