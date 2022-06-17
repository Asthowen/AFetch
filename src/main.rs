use afetch::output::builder::OutputBuilder;
use clap::{Arg, Command};


fn main() {
    let output_builder: OutputBuilder = OutputBuilder::init();

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

    match app.get_matches().subcommand() {
        Some(("logo", sub_matches)) => {
            output_builder
                .fake_logo(sub_matches.value_of("logo").unwrap())
                .show_logo(true)
                .generate_output();
        },
        _ => {
            output_builder
                .show_logo(true)
                .generate_output();
        },
    }
}