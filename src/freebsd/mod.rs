use std::env::var;
use crate::{platform::Platform, shared::{self, procfs::Memory}};
extern crate sysctl;
use sysctl::Sysctl;

pub struct FreeBSD;

impl Platform for FreeBSD {
    fn new() -> Self {
        FreeBSD
    }
    
    fn name(&self) -> Option<String> {
        shared::os_release().or(Some("FreeBSD".to_string()))
    }

    fn user(&self) -> Option<String> {
        shared::libc_user()
    }

    fn kernel(&self) -> Option<String> {
        shared::libc_kernel()
    }

    fn hostname(&self) -> Option<String> {
        shared::libc_hostname()
    }

    fn desktop(&self) -> Option<String> {
        var("XDG_CURRENT_DESKTOP").ok()
    }

    fn shell(&self) -> Option<String> {
        var("SHELL").ok()
    }

    fn uptime(&self) -> Option<usize> {
        let result = sysctl::Ctl::new("kern.boottime").ok()?;
        println!("FREEBSD uptime: {}", result.value_string().unwrap_or("nothing".to_string()));

        None
    }

    fn memory(&self) -> Option<Memory> {
        None
    }
}