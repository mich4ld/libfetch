use std::{fs, env::var};
use crate::shared::{self, procfs::Memory};

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
        var("SHELL").ok()
    }

    pub fn memory(&self) -> Option<Memory> {
        shared::procfs::memory()
    }

    pub fn kernel(&self) -> Option<String> {
        let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").ok()?;
        
        Some(kernel.trim().to_string())
    }

    pub fn uptime(&self) -> Option<usize> {
        let uptime_file = fs::read_to_string("/proc/uptime").ok()?;
        let uptime = uptime_file.split_whitespace().next()?;

        let uptime = uptime.parse::<f64>().ok()? as usize;
        Some(uptime)
    }

    pub fn user(&self) -> Option<String> {
        var("USER").ok()
    }

    pub fn hostname(&self) -> Option<String> {
        let len = 64;
        let mut hostname = std::vec![0; len];

        let err = unsafe {
            libc::gethostname(hostname.as_mut_ptr() as *mut i8, hostname.len().into())
        };

        if err != 0 {
            return None;
        }

        let actual_len = hostname.iter().position(|&byte| { byte == 0 }).unwrap_or(hostname.len());
        let hostname = &hostname[..actual_len];

        String::from_utf8(hostname.to_vec()).ok()
    }

    pub fn desktop(&self) -> Option<String> {
        var("XDG_CURRENT_DESKTOP").ok()
    }
}
