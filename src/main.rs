use std::collections::HashMap;
use std::fmt::Write;

use owo_colors::{DynColors, OwoColorize, XtermColors};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use supports_unicode::supports_unicode;

use afetch::config::deserialize::ColorWrapper;
use afetch::config::{Config, Entry, LogoStyle, SeparatorSizing};
use afetch::error::{ErrorType, FetchInfoError};
use afetch::logos::system_logo;
use afetch::system::{
    InfoGroup, InfoKind, InfoResult, battery_info, cpu_info, disk_info, disks_info, hostname_info,
    kernel_info, loadavg_info, memory_info, motherboard_info, networks_info, product_info,
    public_ip_info, uptime_info,
};
use afetch::translations::get_language;
use afetch::util::count_str_length;

#[cfg(feature = "image")]
use afetch::util::pictures;

fn main() -> Result<(), FetchInfoError> {
    let mut config_buffer: Vec<u8> = Vec::new();
    let config: Config = Config::load(&mut config_buffer);
    let language_func = get_language(config.language);

    let results: HashMap<InfoKind, Result<InfoResult, FetchInfoError>> = config
        .info
        .par_iter()
        .map(|(kind, fields)| {
            (
                *kind,
                match kind {
                    InfoKind::Battery => battery_info(language_func, fields, &config),
                    InfoKind::Cpu => cpu_info(language_func, fields, &config),
                    InfoKind::Disk => disk_info(language_func, fields, &config),
                    InfoKind::Disks => disks_info(language_func, fields, &config),
                    InfoKind::Host => hostname_info(language_func, fields, &config),
                    InfoKind::Kernel => kernel_info(language_func, fields, &config),
                    InfoKind::Loadavg => loadavg_info(language_func, fields, &config),
                    InfoKind::Memory => memory_info(language_func, fields, &config),
                    InfoKind::Motherboard => motherboard_info(language_func, fields, &config),
                    InfoKind::Networks => networks_info(language_func, fields, &config),
                    InfoKind::Product => product_info(language_func, fields, &config),
                    InfoKind::PublicIp => public_ip_info(language_func, fields, &config),
                    InfoKind::Uptime => uptime_info(language_func, fields, &config),
                },
            )
        })
        .collect();

    let logo_buffer;
    let mut logo = if supports_unicode() {
        match config.logo {
            LogoStyle::Braille { logo } => Some(system_logo(logo.map(str::to_owned)))
                .map(|(max_length, ansi, logo)| (max_length, ansi, logo.lines())),
            LogoStyle::File { location: path } => {
                logo_buffer = Some(std::fs::read_to_string(path)?);
                logo_buffer.as_deref().map(|logo| {
                    let max_length = count_str_length(logo) + 6;
                    (max_length, 0, logo.lines())
                })
            }
            _ => None,
        }
    } else {
        None
    };

    let header_color: DynColors = match config.colors.header {
        Some(color) => color.into(),
        None => match &logo {
            Some(color) => DynColors::Xterm(color.1.into()),
            None => DynColors::Xterm(XtermColors::Cyan),
        },
    };
    let header_separator_color: DynColors = config
        .colors
        .header_separator
        .map(ColorWrapper::into)
        .unwrap_or(match &logo {
            Some(color) => DynColors::Xterm(color.1.into()),
            None => DynColors::Xterm(XtermColors::Cyan),
        });
    let info_color: DynColors = config
        .colors
        .info
        .map(ColorWrapper::into)
        .unwrap_or(match &logo {
            Some(color) => DynColors::Xterm(color.1.into()),
            None => DynColors::Xterm(XtermColors::Cyan),
        });
    let separator_color: DynColors =
        config
            .colors
            .separator
            .map(ColorWrapper::into)
            .unwrap_or(match &logo {
                Some(color) => DynColors::Xterm(color.1.into()),
                None => DynColors::Xterm(XtermColors::Cyan),
            });

    let mut output: String = String::default();
    let mut last_info_len = 0;

    for entry in &config.entries {
        let mut write_entry = |entry: String| {
            #[cfg(feature = "image")]
            if matches!(config.logo, LogoStyle::Image { .. }) {
                writeln!(
                    output,
                    "{}{}",
                    " ".repeat(afetch::util::constants::LOGO_LENGTH),
                    entry
                )
                .ok();
            }

            if let Some((width, _, lines)) = logo.as_mut() {
                if let Some(line) = lines.next() {
                    writeln!(output, "   {}{}   {}", line, "".default_color(), entry).ok();
                } else {
                    writeln!(output, "{}{}", " ".repeat(*width), entry).ok();
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
                        formatted_header.color(header_color).bold(),
                        separator.color(header_separator_color),
                        formatted_info.color(info_color)
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

                write_entry(formatted_separator.color(separator_color).to_string());
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
                    let second_colors: String = (0..8).fold(String::default(), |mut acc, i| {
                        write!(&mut acc, "\x1b[9{i}m{content}\x1b[0m").unwrap();
                        acc
                    });
                    write_entry(second_colors);
                }
            }
        }
    }

    if let Some((_, _, lines)) = logo {
        for line in lines {
            writeln!(output, "   {}{}", line, "".default_color()).ok();
        }
    }

    #[cfg(feature = "image")]
    if let LogoStyle::Image { location } = config.logo {
        print!("\n{}\x1b[{}A", output, config.entries.len());
        pictures::display(location);

        return Ok(());
    }

    println!("\n{output}");

    Ok(())
}
