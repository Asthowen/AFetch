use colored::*;

pub struct OutputEntriesGenerator {
    entries: Vec<String>, disabled_entries: Vec<String>
}

impl OutputEntriesGenerator {
    pub fn init(disabled_entries: Vec<String>) -> OutputEntriesGenerator {
        OutputEntriesGenerator { entries: Vec::new(), disabled_entries }
    }

    pub fn add_entry(&mut self, entry_name: &str, entry_value: String) {
        if !self.disabled_entries.contains(&entry_name.to_owned()) && !entry_value.is_empty() {
            self.entries.push(format!("{}: {}", entry_name.cyan().bold(), entry_value));
        }
    }

    pub fn add_custom_entry(&mut self, entry_value: String) {
        self.entries.push(entry_value);
    }

    pub fn get_entries(self) -> Vec<String> {
        self.entries
    }
}