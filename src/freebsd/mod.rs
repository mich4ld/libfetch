use std::env::var;
use crate::{platform::Platform, utils, shared::{self, procfs::Memory}};
extern crate sysctl;
use sysctl::{Sysctl,CtlValue};

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
        let pagesize = sysctl::Ctl::new("hw.pagesize").ok()?
            .value().ok()?;
        let pagesize = get_int_value(pagesize)?;


        let mem_inactive = sysctl::Ctl::new("vm.stats.vm.v_inactive_count").ok()?
            .value().ok()?;
        let mem_inactive = get_int_value(mem_inactive)?;


        let mem_unused = sysctl::Ctl::new("vm.stats.vm.v_free_count").ok()?
            .value().ok()?;
        let mem_unused = get_int_value(mem_unused)?;
 

        let mem_cache = sysctl::Ctl::new("vm.stats.vm.v_cache_count").ok()?
            .value().ok()?;
        let mem_cache = get_int_value(mem_cache)?;


        let mem_total = sysctl::Ctl::new("hw.physmem").ok()?
            .value().ok()?;
        let mem_total = get_int_value(mem_total)? / 1024;

        let mem_free = (mem_inactive + mem_unused + mem_cache) * pagesize / 1024;

        let memory = Memory::new(mem_total - mem_free, mem_total);

        Some(memory)
    }
}

fn get_int_value(enum_value: CtlValue) -> Option<usize> {
    match enum_value {
        CtlValue::Int(i) => {
            let as_usize: usize = i.try_into().ok()?;
            return Some(as_usize);
        }
        CtlValue::U32(i) => {
            let as_usize: usize = i.try_into().ok()?;
            return Some(as_usize);
        }
        CtlValue::Uint(i) => {
            let as_usize: usize = i.try_into().ok()?;
            return Some(as_usize);
        }
        CtlValue::Ulong(i) => {
            let as_usize: usize = i.try_into().ok()?;
            return Some(as_usize);
        }
        _ => {
            return None;
        }
    };
}
