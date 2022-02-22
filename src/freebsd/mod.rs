use std::env::var;
use crate::{platform::Platform, utils, shared::{self, procfs::Memory}};
extern crate sysctl;
use sysctl::Sysctl;

pub struct FreeBSD;

#[derive(Debug)]
#[repr(C)]
struct Boottime {
    sec: libc::c_int,
}

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
        let val_enum = result.value().ok()?;

        if let sysctl::CtlValue::Struct(val) = val_enum {
            let val_ptr: *const u8 = val.as_ptr();
            let struct_ptr: *const Boottime = val_ptr as *const Boottime;
            let struct_ref: &Boottime = unsafe { &*struct_ptr };

            let now = utils::get_now()?;
            let uptime = now - struct_ref.sec as u64;
            let uptime: usize = uptime.try_into().ok()?;

            return Some(uptime);
        }

        None
    }

    fn memory(&self) -> Option<Memory> {
        None
    }
}