pub fn get_parent_pid(pid: u32) -> Vec<u32> {
    let mut pids: Vec<u32> = Vec::new();
    let ret = std::process::Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output();

    if ret.is_err() {
        return pids;
    }

    let output: String = String::from_utf8_lossy(&ret.unwrap().stdout).to_string();
    for pid in output.split("\n") {
        match pid.trim().parse::<u32>() {
            Ok(p) => pids.push(p),
            Err(_) => break,
        }
    }
    pids
}

pub fn get_pid_names(pids: Vec<u32>) -> Vec<String> {
    let mut names: Vec<String> = Vec::new();
    for pid in pids {
        let ret = std::process::Command::new("ps")
            .arg("-p")
            .arg(format!("{}", pid))
            .arg("-o")
            .arg("comm=")
            .output();
        names.push(
            String::from_utf8_lossy(&ret.unwrap().stdout)
                .to_string()
                .replace("\n", ""),
        );
    }
    names
}

pub fn pid_names_clean(pids_name: Vec<String>) -> Vec<String> {
    let mut pids_name_clean: Vec<String> = Vec::new();

    for pid_name in pids_name {
        if vec!["bash", "fish", "sh", "ksh", "afetch"].contains(&pid_name.as_str()) {
            continue;
        }
        pids_name_clean.push(pid_name);
    }

    pids_name_clean
}
