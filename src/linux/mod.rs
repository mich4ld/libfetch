use std::fs;

pub struct Linux {}

impl Linux {
    pub fn name(&self) -> Option<String> {
        let mut os_name = String::new();
        let release_file = fs::read_to_string("/etc/os-release").ok()?;

        for line in release_file.lines() {
            if line.is_empty() {
                continue;
            }

            let entries = line.splitn(2, "=")
                .map(|entry| { entry.trim() })
                .collect::<Vec<&str>>();
            
            // Expecting exacly 2 entries
            if entries.len() != 2 {
                continue;
            }

            let key = entries[0];
            if key == "PRETTY_NAME" || key == "NAME" {
                os_name = entries[1].replace('\"', "");
            }
        }

        if os_name.is_empty() {
            return None;
        }

        Some(os_name)
    }

    pub fn memory(&self) {
    }
}
