use crate::config::Shell;
use crate::error::FetchInfosError;
use crate::system::infos::shell::get_shell;
use crate::utils::{env_exist, return_str_from_command};
use std::env::var;
use std::process::Command;

pub async fn get_terminal() -> Result<Option<String>, FetchInfosError> {
    if let Ok(term_program) = var("TERM_PROGRAM") {
        return Ok(Some(match term_program.trim() {
            "iTerm.app" => "iTerm2".to_owned(),
            "Terminal.app" => "Apple Terminal".to_owned(),
            "Hyper" => "HyperTerm".to_owned(),
            "vscode" => "VSCode".to_owned(),
            value => value.to_owned(),
        }));
    }
    if let Ok(term) = var("TERM") {
        if term == "tw52" || term == "tw100" {
            return Ok(Some("TosWin2".to_owned()));
        }
    }
    if env_exist("SSH_CONNECTION") {
        if let Ok(ssh_tty) = var("SSH_TTY") {
            return Ok(Some(ssh_tty));
        }
        return Ok(None);
    }
    if env_exist("WT_SESSION") {
        return Ok(Some("Windows Terminal".to_owned()));
    }
    let pids_names: Vec<String> = match crate::system::pid::get_parent_pid_names() {
        Ok(pids) => pids,
        Err(error) => {
            println!("{error}");
            return Ok(None);
        }
    };
    let shell_opt = get_shell(Shell { version: false }).await?;
    let shell_name = match shell_opt {
        Some(shell) => shell.0,
        None => return Ok(None),
    };

    let mut term: String = String::default();
    for name in pids_names {
        match name.as_str() {
            name if shell_name == name => {}
            "sh" | "screen" | "su" | "dolphin" | "nautilus" => {}
            "login" | "Login" | "init" | "(init)" => {
                term = return_str_from_command(&mut Command::new("tty"))?;
            }
            "ruby" | "1" | "tmux" | "systemd" | "sshd" | "python" | "USER" | "PID" | "kdeinit"
            | "launchd" | "ksmserver" => break,
            _ if name.starts_with("plasma") || name.starts_with("kwin_") => break,
            "gnome-terminal-" => "gnome-terminal".clone_into(&mut term),
            "urxvtd" => "urxvt".clone_into(&mut term),
            _ if name.contains("nvim") => "Neovim Terminal".clone_into(&mut term),
            _ if name.contains("NeoVimServer") => "VimR Terminal".clone_into(&mut term),
            _ => {
                term = if term.starts_with('.') && term.ends_with("-wrapped") {
                    term.trim_start_matches('.')
                        .trim_end_matches("-wrapped")
                        .to_owned()
                } else {
                    name.clone()
                };
            }
        }
    }

    if term.is_empty() {
        return Ok(None);
    }

    Ok(Some(format!("{}{}", &term[..1].to_uppercase(), &term[1..])))
}
