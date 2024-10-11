use crate::error::FetchInfosError;
use crate::utils::{command_exist, return_str_from_command};
use std::process::Command;
#[cfg(target_family = "unix")]
use {crate::utils::count_lines_in_output, std::path::Path, tokio::task, tokio::task::JoinHandle};

pub async fn get_packages_infos() -> Result<Option<String>, FetchInfosError> {
    let mut packages_string: Vec<String> = Vec::new();

    #[cfg(target_family = "unix")]
    {
        let package_managers = [
            ("pacman", "pacman", vec!["-Qq", "--color", "never"]),
            ("kiss", "kiss", vec!["l"]),
            ("cpt", "cpt-list", Vec::new()),
            ("dpkg", "dpkg-query", vec!["-f", "'.\n'", "-W"]),
            ("xbps-query", "xbps-query", vec!["-l"]),
            ("apk", "apk", vec!["info"]),
            ("opkg", "opkg", vec!["list-installed"]),
            ("pacman-g2", "pacman-g2", vec!["-Q"]),
            ("lvu", "lvu", vec!["installed"]),
            ("tce", "tce-status", vec!["-i"]),
            ("pkg", "pkg_info", Vec::new()),
            ("pkg", "pkg", vec!["info"]),
            ("pkgin", "pkgin", vec!["list"]),
            (
                "tazpkg",
                "",
                vec!["pkgs_h=6", "tazpkg", "list", "&&", "((packages-=6))"],
            ),
            ("sorcery", "gaze", vec!["installed"]),
            ("alps", "alps", vec!["showinstalled"]),
            ("butch", "butch", vec!["list"]),
            ("swupd", "swupd", vec!["bundle-list", "--quiet"]),
            ("pisi", "pisi", vec!["li"]),
            ("pacstall", "pacstall", vec!["-L"]),
            ("flatpak", "flatpak", vec!["list"]),
            ("spm", "spm", vec!["list", "-i"]),
            ("snap", "snap", vec!["list"]),
            ("snap", "mine", vec!["-q"]),
        ];

        let mut handles: Vec<JoinHandle<Result<Option<String>, FetchInfosError>>> = Vec::new();
        for (name, command, args) in package_managers {
            if !command_exist(command) {
                continue;
            }

            let handle: JoinHandle<Result<Option<String>, FetchInfosError>> =
                task::spawn(async move {
                    let packages_count: usize = count_lines_in_output(return_str_from_command(
                        Command::new(command).args(args),
                    )?);

                    if packages_count != 0 {
                        return Ok(Some(format!("{packages_count} ({name})")));
                    }
                    Ok(None)
                });
            handles.push(handle);
        }

        let handle_rpm: JoinHandle<Result<Option<String>, FetchInfosError>> =
            task::spawn(async move {
                if command_exist("dnf")
                    && command_exist("sqlite3")
                    && Path::new("/var/cache/dnf/packages.db").exists()
                {
                    let packages_count = count_lines_in_output(return_str_from_command(
                        Command::new("sqlite3")
                            .arg("/var/cache/dnf/packages.db")
                            .arg(r#""SELECT count(pkg) FROM installed""#),
                    )?);
                    if packages_count != 0 {
                        return Ok(Some(format!("{packages_count} (dnf)")));
                    }
                } else if command_exist("rpm") {
                    let packages_count = count_lines_in_output(return_str_from_command(
                        Command::new("rpm").arg("-qa"),
                    )?);
                    if packages_count != 0 {
                        return Ok(Some(format!("{packages_count} (dnf)")));
                    }
                }

                Ok(None)
            });
        handles.push(handle_rpm);

        for handle in handles {
            match handle.await {
                Ok(Ok(Some(formatted))) => packages_string.push(formatted),
                Err(error) => {
                    println!("Error while fetching packages number: {error}");
                }
                _ => {}
            }
        }

        if packages_string.is_empty() {
            return Ok(None);
        }

        Ok(Some(packages_string.join(", ")))
    }

    #[cfg(target_os = "windows")]
    {
        if command_exist("choco") {
            let choco_output: String =
                return_str_from_command(Command::new("choco").arg("list").arg("--localonly"))?;
            let choco_output_split: Vec<&str> = choco_output
                .split(" packages installed")
                .collect::<Vec<&str>>()[0]
                .lines()
                .collect::<Vec<&str>>();
            packages_string.push(format!(
                "{} (chocolatey)",
                choco_output_split[choco_output_split.len() - 1],
            ));
        }

        Ok(Some(packages_string.join(", ")))
    }

    #[cfg(not(any(target_os = "windows", target_family = "unix")))]
    {
        // TODO - add other OS
        Ok(None)
    }
}
