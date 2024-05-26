use crate::utils::get_file_content;
use std::path::Path;
use sysinfo::{Pid, System};

pub async fn get_ppid(pid: &str) -> Option<String> {
    let status_path = Path::new("/proc").join(pid).join("status");

    if let Ok(status_content) = get_file_content(status_path).await {
        for line in status_content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 && parts[0] == "PPid:" {
                return Some(parts[1].to_owned());
            }
        }
    }

    None
}

pub fn get_parent_pid_names() -> Result<Vec<String>, String> {
    let process_pid: Pid = Pid::from_u32(std::process::id());

    let mut system = System::new();
    system.refresh_process(process_pid);

    let process = system
        .process(process_pid)
        .ok_or_else(|| format!("Process with PID {} not found", std::process::id()))?;

    let mut parent_names = Vec::new();
    let mut current_pid = process.parent();

    while let Some(parent) = current_pid {
        if parent_names.len() == 5 {
            break;
        }

        system.refresh_process(parent);

        let parent_process = system
            .process(parent)
            .ok_or_else(|| format!("Parent process with PID {} not found", parent.as_u32()))?;

        parent_names.push(parent_process.name().to_owned());
        current_pid = parent_process.parent();
    }

    Ok(parent_names)
}
