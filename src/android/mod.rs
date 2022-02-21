use std::{env::var, ffi::CStr, process::Command};
use crate::{utils::exec, shared::{self, procfs::Memory}, platform::Platform};

pub struct Android {}

impl Platform for Android {
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
        None
    }

    fn user(&self) -> Option<String> {
        None
    }

    fn hostname(&self) -> Option<String> {
        None
    }

    fn desktop(&self) -> Option<String> {
        None
    }
}
