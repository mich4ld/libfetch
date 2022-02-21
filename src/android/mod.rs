use std::{env::var, ffi::CStr, process::Command};
use crate::{utils::exec, shared::{self, procfs::Memory}, platform::Platform};

pub struct Android;

impl Platform for Android {
    fn new() -> Self {
        Android
    }

    fn name(&self) -> Option<String> {
        let mut getprop = Command::new("getprop");
        getprop.arg("ro.build.version.release");
        
        let release = exec(&mut getprop)?;

        Some(format!("Android {}", release.trim()))
    }

    fn kernel(&self) -> Option<String> {
        let mut utsname = unsafe { libc::utsname::from(std::mem::zeroed()) };
        let err = unsafe { 
            libc::uname(&mut utsname)  
        };

        if err == -1 {
            return None;
        }
        
        let release = unsafe { CStr::from_ptr(utsname.release.as_ptr()) }.to_str().ok()?;
        Some(release.to_string())
    }

    fn memory(&self) -> Option<Memory> {
        shared::procfs::memory()
    }

    fn shell(&self) -> Option<String> {
        var("SHELL").ok()
    }

    fn uptime(&self) -> Option<usize> {
        let mut sysinfo = unsafe { libc::sysinfo::from(std::mem::zeroed()) };
        let err = unsafe { libc::sysinfo(&mut sysinfo) };

        if err != 0 {
            return None;
        }

        let uptime: usize = sysinfo.uptime.try_into().ok()?;

        Some(uptime)
    }

    fn user(&self) -> Option<String> {
        None
    }

    fn hostname(&self) -> Option<String> {
        let len = 64;
        let mut hostname = std::vec![0; len];

        let err = unsafe {
            libc::gethostname(hostname.as_mut_ptr() as *mut u8, hostname.len().into())
        };

        if err != 0 {
            return None;
        }

        let actual_len = hostname.iter().position(|&byte| { byte == 0 }).unwrap_or(hostname.len());
        let hostname = &hostname[..actual_len];

        String::from_utf8(hostname.to_vec()).ok()   
    }

    fn desktop(&self) -> Option<String> {
        None
    }
}
