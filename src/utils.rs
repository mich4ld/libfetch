use std::{process::Command, time::SystemTime};

#[cfg(target_os = "freebsd")]
use std::time::{SystemTime, UNIX_EPOCH};

pub fn exec(command: &mut Command) -> Option<String> {
    let result = command.output().ok()?;
    
    if !result.status.success() {
        return None;
    }
    
    let result_str = String::from_utf8(result.stdout).ok()?;
    Some(result_str)
}

#[cfg(target_os = "freebsd")]
pub fn get_now() -> Option<u64> {
    let start = SystemTime::now();
    let duration = start.duration_since(UNIX_EPOCH).ok()?;

    Some(duration.as_secs())
}