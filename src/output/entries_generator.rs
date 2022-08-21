use colored::*;

pub struct OutputEntriesGenerator {
    entries: Vec<String>
}

impl OutputEntriesGenerator {
    pub const fn init() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry_name: &str, entry_value: String) {
        if !entry_value.is_empty() {
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