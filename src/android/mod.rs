use std::{env::var, ffi::CStr, process::Command};

use crate::utils::exec;

pub struct Android {}

impl Android {
    pub fn name(&self) -> Option<String> {
        let mut getprop = Command::new("getprop");
        getprop.arg("ro.build.version.release");
        
        let release = exec(&mut getprop)?;

        Some(format!("Android {}", release.trim()))
    }

    pub fn kernel(&self) -> Option<String> {
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

    pub fn memory(&self) {
     // it's will be shared with Linux 
    }

    pub fn shell(&self) -> Option<String> {
        var("SHELL").ok()
    }
}
