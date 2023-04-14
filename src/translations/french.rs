use std::collections::HashMap;

pub fn french() -> HashMap<&'static str, &'static str> {
    HashMap::from_iter(vec![
        ("days", "jours"),
        ("hours", "heures"),
        ("minutes", "minutes"),
        ("seconds", "secondes"),
        ("day", "jour"),
        ("hour", "heure"),
        ("minute", "minute"),
        ("second", "seconde"),
        ("label-os", "OS : "),
        ("label-host", "Hôte : "),
        ("label-kernel", "Noyau : "),
        ("label-uptime", "Uptime : "),
        ("label-packages", "Paquets : "),
        ("label-resolution", "Résolution : "),
        ("label-shell", "Shell : "),
        ("label-terminal", "Terminal : "),
        ("label-terminal-font", "Police : "),
        ("label-memory", "Mémoire : "),
        ("label-cpu", "CPU : "),
        ("label-network", "Réseau : "),
        ("label-disk", "Disque "),
        ("label-disk-1", " : "),
        ("label-disks", "Disques : "),
        ("label-public-ip", "IP publique : "),
        ("label-desktop", "Bureau : "),
        ("label-battery", "Batterie : "),
    ])
}
