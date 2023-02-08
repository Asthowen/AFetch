use std::str::FromStr;

pub fn get_parent_pids(pid: u32) -> Result<Vec<u32>, String> {
    let output = std::process::Command::new("ps")
        .arg("-o")
        .arg(format!("ppid={}", pid))
        .output()
        .map_err(|e| e.to_string())?;
    let output = String::from_utf8_lossy(&output.stdout);

    let parent_pids = output
        .lines()
        .filter_map(|line| u32::from_str(line.trim()).ok())
        .collect();
    Ok(parent_pids)
}

pub fn get_pid_names(pids: Vec<u32>) -> Result<Vec<String>, String> {
    let pid_names = pids
        .iter()
        .map(|&pid| {
            let output = std::process::Command::new("ps")
                .arg("-p")
                .arg(format!("{}", pid))
                .arg("-o")
                .arg("comm=")
                .output()
                .map_err(|e| e.to_string())?;
            let output = String::from_utf8_lossy(&output.stdout);
            Ok(output.trim().to_owned())
        })
        .collect::<Result<Vec<String>, String>>()?;
    Ok(pid_names)
}

pub fn clean_pid_names(pid_names: Vec<String>) -> Vec<String> {
    let filtered_names = vec!["bash", "fish", "sh", "ksh", "afetch"];
    pid_names
        .iter()
        .filter(|name| !filtered_names.contains(&name.as_str()))
        .map(|name| name.to_owned())
        .collect()
}
