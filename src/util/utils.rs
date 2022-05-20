pub fn div_mod(dividend: u64, divisor: u64) -> (u64, u64) {
    (dividend / divisor, dividend % divisor)
}

pub fn return_str_from_command(command: &mut std::process::Command) -> String {
    String::from_utf8(command.output().unwrap().stdout).unwrap()
}

pub fn get_file_in_one_line(file_path: &str) -> String {
    std::fs::read_to_string(file_path).unwrap().replace("\n", "")
}

pub fn is_command_exist(program: &str) -> bool {
    if let Ok(_) = which::which(program) {
        true
    } else {
        false
    }
}

pub fn format_time(time_to_format: u64) -> String {
    let (minutes, seconds): (u64, u64) = div_mod(time_to_format, 60);
    let (hours, minutes): (u64, u64) = div_mod(minutes, 60);
    let (days, hours): (u64, u64) = div_mod(hours, 24);
    let mut uptime_formatted: Vec<String> = Vec::new();

    if days > 0 {
        uptime_formatted.push(format!("{} days", days.to_string()));
    }
    if hours > 0 {
        uptime_formatted.push(format!("{} hours", hours.to_string()));
    }
    if minutes > 0 {
        uptime_formatted.push(format!("{} mins", minutes.to_string()));
    }
    if seconds > 0 && seconds == 0 {
        uptime_formatted.push(format!("{} seconds", seconds.to_string()));
    }
    uptime_formatted.join(", ")
}

// Based on the human_bytes library of Forkbomb9: https://gitlab.com/forkbomb9/human_bytes-rs
pub fn convert_to_readable_unity<T: Into<f64>>(size: T) -> String {
    const SUFFIX: [&'static str; 9] = ["B", "kB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];
    let size: f64 = size.into();
    if size <= 0.0 {
        return "0 B".to_string();
    }
    let base: f64 = size.log10() / 1024_f64.log10();
    let mut result: String = format!("{:.1}", 1024_f64.powf(base - base.floor()))
        .trim_end_matches(".0")
        .to_owned();
    result.push_str(SUFFIX[base.floor() as usize]);
    result
}

pub fn check_if_env_exist(env_var: &str) -> bool {
    match std::env::var(env_var) {
        Ok(_) => true,
        Err(_) => false
    }
}

pub fn get_env(env_var: &str) -> String {
    return if check_if_env_exist(env_var) {
        std::env::var(env_var).unwrap()
    } else {
        "".to_string()
    }
}