use crate::config::Shell;
use crate::error::FetchInfosError;
use crate::utils::return_str_from_command;
use std::env::var;
use std::process::Command;

pub async fn get_shell(config: Shell) -> Result<Option<(String, Option<String>)>, FetchInfosError> {
    let shell_path: String = var("SHELL")?;
    let shell_name: String = match shell_path.split('/').last() {
        Some(shell_name) => shell_name.to_owned(),
        None => return Ok(None),
    };

    if shell_name.is_empty() {
        return Ok(None);
    }

    if !config.version {
        return Ok(Some((shell_name.replace('\n', ""), None)));
    }

    if let Ok(shell_version) = var("SHELL_VERSION") {
        return Ok(Some((shell_name.replace('\n', ""), Some(shell_version))));
    }

    let shell_version: String = if shell_name == "fish" {
        return_str_from_command(Command::new(shell_path).arg("--version"))?
            .split("fish, version ")
            .collect::<Vec<&str>>()[1]
            .replace('\n', "")
    } else if shell_name == "bash" {
        match var("BASH_VERSION") {
            Ok(bash_version) => bash_version,
            Err(_) => return_str_from_command(
                Command::new(shell_path).arg("-c").arg("echo $BASH_VERSION"),
            )?,
        }
    } else if shell_name == "sh" {
        return_str_from_command(Command::new("sh").arg("--version"))?
            .split("GNU bash, version ")
            .collect::<Vec<&str>>()[1]
            .split(' ')
            .collect::<Vec<&str>>()[0]
            .to_owned()
    } else if shell_name == "ksh" {
        return_str_from_command(Command::new("ksh").arg("--version"))?
            .split("(AT&T Research) ")
            .collect::<Vec<&str>>()[1]
            .trim()
            .to_owned()
    } else {
        String::default()
    };

    if shell_version.is_empty() {
        Ok(Some((shell_name.replace('\n', ""), None)))
    } else {
        Ok(Some((shell_name.replace('\n', ""), Some(shell_version))))
    }
}
