use std::{fs, env::var};
use crate::{shared::{self, procfs::Memory}, platform::Platform};

pub struct Linux;

impl Platform for Linux {
    fn new() -> Self {
        Linux
    }

    fn name(&self) -> Option<String> {
        shared::os_release()
    }

    fn shell(&self) -> Option<String> {
        var("SHELL").ok()
    }

    fn memory(&self) -> Option<Memory> {
        shared::procfs::memory()
    }

    fn kernel(&self) -> Option<String> {
        let kernel = fs::read_to_string("/proc/sys/kernel/osrelease").ok()?;
        
        Some(kernel.trim().to_string())
    }

    fn uptime(&self) -> Option<usize> {
        let uptime_file = fs::read_to_string("/proc/uptime").ok()?;
        let uptime = uptime_file.split_whitespace().next()?;

        let uptime = uptime.parse::<f64>().ok()? as usize;
        Some(uptime)
    }

    fn user(&self) -> Option<String> {
        var("USER").ok()
    }

    fn hostname(&self) -> Option<String> {
        shared::libc_hostname()
    }

    fn desktop(&self) -> Option<String> {
        var("XDG_CURRENT_DESKTOP").ok()
    }
}
