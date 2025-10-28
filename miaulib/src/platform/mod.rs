use std::{ffi::CStr, mem};

#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,
    pub node_name: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub os: String,
}

fn ctsr_to_string(raw_ctsr: &[i8]) -> String {
    unsafe { CStr::from_ptr(raw_ctsr.as_ptr()) }
        .to_string_lossy()
        .into_owned()
}

pub fn get_platform_info() -> Result<Uname, String> {
    let mut uts: mem::MaybeUninit<libc::utsname> = mem::MaybeUninit::uninit();

    let result = unsafe { libc::uname(uts.as_mut_ptr()) };

    if result < 0 {
        return Err("Failed to get system information from uname() syscall".to_string());
    }

    let uts = unsafe { uts.assume_init() };

    Ok(Uname {
        sys_name: ctsr_to_string(&uts.sysname),
        node_name: ctsr_to_string(&uts.nodename),
        release: ctsr_to_string(&uts.release),
        version: ctsr_to_string(&uts.version),
        machine: ctsr_to_string(&uts.machine),
        os: "GNU/Linux".to_string(),
    })
}
