use afetch::output::builder::OutputBuilder;
use yaml_rust::{YamlLoader, Yaml};
use clap::{Arg, Command};
use std::process::exit;
use std::fs;


fn main() {
    let afetch_config_path: std::path::PathBuf = dirs::config_dir().unwrap_or_else(|| {
        println!("Your system is not supported, please open an issue at: https://github.com/Asthowen/AFetch/ so I can add support for your system.");
        exit(9);
    }).join("afetch").join("config.yaml");
    fs::create_dir_all(afetch_config_path.parent().unwrap()).unwrap();

    let yaml_to_parse: String = if afetch_config_path.exists() {
        fs::read_to_string(afetch_config_path).unwrap()
    } else {
        let to_write: String = "language: auto # fr / en / auto\nlogo: default # default / disable / ...\ndisable_entries:\n  - publicip\n  - disk".to_owned();
        fs::write(afetch_config_path, to_write.clone()).unwrap();
        to_write
    };
    let read: Vec<Yaml> = YamlLoader::load_from_str(&*yaml_to_parse).unwrap_or_else(|e| {
        println!("Your YAML is malformed ({})", e);
        exit(9);
    });
    let wanted_yaml: Yaml = read[0].clone();
    let output_builder: OutputBuilder = OutputBuilder::init(wanted_yaml);

    let app: Command = Command::new("AFetch")
        .about("A CLI app to retrieve system information.")
        .version("0.0.1")
        .help_template("{bin} ({version}) - Created by {author}\n\n{usage-heading}\n{usage}\n\n{all-args}\n")
        .subcommand_required(false)
        .arg_required_else_help(false)
        .author("Asthowen")
        .subcommand(
            Command::new("logo")
                .short_flag('l')
                .long_flag("logo")
                .about("Choose a fake logo to display.")
                .arg(
                    Arg::new("logo")
                        .help("The distribution name.")
                        .required_unless_present("logo")
                        .takes_value(true)
                        .multiple_values(false),
                ),
        );
    let disables_entries: Yaml = output_builder.config["disable_entries"].clone();
    let disables_entries_array: Vec<String> =  if disables_entries.is_badvalue() && !disables_entries.is_null() && !disables_entries.is_array() {
        Vec::new()
    } else {
        disables_entries.into_iter().map(|x| x.into_string().unwrap()).collect::<Vec<String>>()
    };
    let logo: String = output_builder.config["logo"].clone().into_string().unwrap_or_else(|| "default".to_owned());
    let show_logo: bool = logo.to_lowercase() != "disable";

    match app.get_matches().subcommand() {
        Some(("logo", sub_matches)) => {
            output_builder
                .disable_entries(disables_entries_array)
                .fake_logo(sub_matches.value_of("logo").unwrap())
                .show_logo(show_logo)
                .generate_output();
        },
        _ => {
            output_builder
                .disable_entries(disables_entries_array)
                .show_logo(show_logo)
                .fake_logo(&*logo)
                .generate_output();
        },
    }
}