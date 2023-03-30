use std::collections::HashMap;

pub fn english() -> HashMap<&'static str, &'static str> {
    HashMap::from_iter(vec![
        ("days", "days"),
        ("hours", "hours"),
        ("minutes", "minutes"),
        ("seconds", "seconds"),
        ("day", "day"),
        ("hour", "hour"),
        ("minute", "minute"),
        ("second", "second"),
        ("label-os", "OS: "),
        ("label-host", "Host: "),
        ("label-kernel", "Kernel: "),
        ("label-uptime", "Uptime: "),
        ("label-packages", "Packages: "),
        ("label-resolution", "Resolution: "),
        ("label-shell", "Shell: "),
        ("label-terminal", "Terminal: "),
        ("label-memory", "Memory: "),
        ("label-cpu", "CPU: "),
        ("label-network", "Network: "),
        ("label-disk", "Disk "),
        ("label-disk-1", ": "),
        ("label-disks", "Disks: "),
        ("label-public-ip", "Public IP: "),
        ("label-desktop", "Desktop: "),
        ("label-battery", "Battery: "),
    ])
}
