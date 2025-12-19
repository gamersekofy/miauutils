#[derive(Debug)]
pub struct Uname {
    pub sys_name: String,
    pub node_name: String,
    pub release: String,
    pub version: String,
    pub machine: String,
    pub os: String,
}

#[cfg(unix)]
mod unix;

#[cfg(unix)]
pub use self::unix::get_platform_info;

#[cfg(windows)]
mod windows;

#[cfg(windows)]
pub use self::windows::get_platform_info;
