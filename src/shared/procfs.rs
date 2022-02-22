use std::fs;

pub fn memory() -> Option<Memory> {
    let meminfo = fs::read_to_string("/proc/meminfo").ok()?;

    let mut free: usize = 0;
    let mut total: usize = 0;
    let mut buffers: usize = 0;
    let mut cached: usize = 0;

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
                total = entries[1].parse().ok()?;
            }
            "MemFree:" => {
                free = entries[1].parse().ok()?;
            }
            "Buffers:" => {
                buffers = entries[1].parse().ok()?;
            }
            "Cached:" => {
                cached = entries[1].parse().ok()?;
            }
            _ => {
                continue;
            }
        }
    }

    let used = total - free - buffers - cached;
    let memory = Memory::new(used, total);
    Some(memory)
}

#[derive(Debug)]
pub struct Memory {
    pub total: usize,
    pub used: usize,
}

impl Memory {
    pub fn new(used: usize, total: usize) -> Self {
        Memory { used, total  }
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            total: 0,
            used: 0,
        }  
    }
}
