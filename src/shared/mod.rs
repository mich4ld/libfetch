use std::{ffi::CStr, fs};

pub mod procfs;

#[cfg(not(target_os = "android"))]
pub fn libc_hostname() -> Option<String> {
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

pub fn libc_kernel() -> Option<String> {
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

pub fn libc_user() -> Option<String> {
    let login = unsafe { libc::getlogin() };
    let login_str = unsafe { CStr::from_ptr(login) }.to_str().ok()?;

    Some(String::from(login_str))
}

pub fn os_release() -> Option<String> {
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