use std::fs;
use std::path::Path;
use std::str::FromStr;

pub fn get_ppid(pid: &str) -> Option<String> {
    let status_path = Path::new("/proc").join(pid).join("status");

    if let Ok(status_content) = fs::read_to_string(status_path) {
        for line in status_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "PPid:" {
                return Some(parts[1].to_owned());
            }
        }
    }

    None
}

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
    let mut args: Vec<String> = Vec::new();
    for pid in pids {
        args.push("-p".to_owned());
        args.push(format!("{}", pid));
    }
    args.push("-o".to_owned());
    args.push("comm=".to_owned());

    let output = std::process::Command::new("ps")
        .args(args)
        .output()
        .map_err(|e| e.to_string())?;

    let output: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .trim()
        .split('\n')
        .map(|s| s.into())
        .collect::<Vec<String>>();

    Ok(output)
}

pub fn clean_pid_names(pid_names: Vec<String>) -> Vec<String> {
    let filtered_names: [&str; 5] = ["bash", "fish", "sh", "ksh", "afetch"];
    pid_names
        .iter()
        .filter(|name| !filtered_names.contains(&name.as_str()))
        .map(|name| name.to_owned())
        .collect()
}
