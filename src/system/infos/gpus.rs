use crate::error::FetchInfosError;
use crate::utils::return_str_from_command;
use std::process::Command;

pub fn get_gpus() -> Result<Option<Vec<String>>, FetchInfosError> {
    #[cfg(target_os = "macos")]
    return Ok(None);

    #[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
    {
        let gpu_cmd: String = return_str_from_command(Command::new("lspci").args(["-mm"]))?;
        let mut gpus: Vec<String> = Vec::new();
        for line in gpu_cmd
            .lines()
            .filter(|line| line.contains("Display") || line.contains("3D") || line.contains("VGA"))
        {
            let parts: Vec<&str> = line
                .split(['"', '(', ')'])
                .filter(|&s| !s.trim().is_empty())
                .map(|s| s.trim())
                .collect();
            let gpu: String = format!(
                "{}{}",
                parts[2].trim(),
                if parts[3].is_empty() {
                    String::default()
                } else {
                    format!(" {}", parts[3])
                },
            );

            if !gpus.contains(&gpu) {
                gpus.push(gpu);
            }
        }

        if gpus.first().map_or(false, |gpu| gpu.contains("Intel"))
            && gpus.get(1).map_or(false, |gpu| gpu.contains("Intel"))
        {
            gpus.remove(0);
        }

        let mut gpus_clean: Vec<String> = Vec::new();
        for gpu in gpus {
            gpus_clean.push(match &*gpu {
                gpu if gpu.contains("Advanced") => {
                    let mut gpu: String = gpu.to_owned();
                    if let Some(start_index) = gpu.find(']') {
                        let temp = &gpu[start_index + 1..];
                        if let Some(end_index) = temp.find('[') {
                            gpu = format!("{}{}", &gpu[0..start_index + 1], &temp[end_index..]);
                        }
                    }
                    gpu = gpu
                        .replace("[AMD/ATI]", "AMD ATI ")
                        .replace("[AMD]", "AMD ")
                        .replace("OEM ", "")
                        .replace("Advanced Micro Devices, Inc.", "");

                    if let Some(start_index) = gpu.find('[') {
                        if let Some(end_index) = gpu.find(']') {
                            gpu = format!(
                                "{}{}",
                                &gpu[..start_index],
                                &gpu[start_index + 1..end_index]
                            );
                        }
                    }
                    gpu.trim().to_owned()
                }
                gpu if gpu.contains("NVIDIA") => format!(
                    "NVIDIA {}",
                    gpu.split('[').collect::<Vec<&str>>()[1].replace(']', "")
                ),
                gpu if gpu.contains("Intel") => {
                    let gpu: String = gpu
                        .replace("(R)", "")
                        .replace("Corporation ", "")
                        .split(" (")
                        .next()
                        .unwrap()
                        .to_owned()
                        .trim()
                        .to_owned();
                    if gpu.is_empty() {
                        "Intel Integrated Graphics".to_owned()
                    } else {
                        gpu
                    }
                }
                gpu if gpu.contains("MCST") => gpu.replace("MCST MGA2", "").to_owned(),
                gpu if gpu.contains("VirtualBox") => "VirtualBox Graphics Adapter".to_owned(),
                gpu => gpu.trim().to_owned(),
            });
        }
        Ok(Some(gpus_clean))
    }

    #[cfg(target_os = "windows")]
    {
        let mut gpus: Vec<String> = Vec::new();

        let wmic_output = return_str_from_command(Command::new("wmic").args([
            "path",
            "Win32_VideoController",
            "get",
            "caption",
        ]))?;

        for line in wmic_output.lines() {
            let line: String = line.replace('\n', "").trim().to_owned();
            if line.is_empty() || line == "Caption" {
                continue;
            }
            gpus.push(line);
        }

        Ok(Some(gpus))
    }
}
