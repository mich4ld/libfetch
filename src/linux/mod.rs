use std::{fs, env::var};

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

    pub fn shell(&self) -> Option<String> {
        let shell = var("SHELL").ok()?;

        Some(shell)
    }

    pub fn memory(&self) -> Option<Memory> {
        let meminfo = fs::read_to_string("/proc/meminfo").ok()?;
        let mut memory = Memory::default();

        for line in meminfo.lines() {
            if line.is_empty() {
                continue;
            }

            let entries = line.split_whitespace().collect::<Vec<&str>>();
            if entries.len() < 2 {
                continue;
            }
            
            match entries[0] {
                "MemTotal:" => {
                    memory.total = entries[1].parse().ok()?;
                }
                "MemFree:" => {
                    memory.free = entries[1].parse().ok()?;
                }
                "MemAvailable:" => {
                    memory.available = entries[1].parse().ok()?;
                }
                "Buffers:" => {
                    memory.buffers = entries[1].parse().ok()?;
                }
                "Cached:" => {
                    memory.cached = entries[1].parse().ok()?;
                }
                _ => {
                    continue;
                }
            }
        }

        Some(memory)
    }
}

#[derive(Debug)]
pub struct Memory {
    pub total: usize,
    pub available: usize,
    pub free: usize,
    pub buffers: usize,
    pub cached: usize,
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            total: 0,
            available: 0,
            free: 0,
            buffers: 0,
            cached: 0,
        }  
    }
}

impl Memory {
    pub fn used(&self) -> usize {
        self.total - self.free - self.buffers - self.cached
    }
}
