use windows::{
    Win32::System::{
        SystemInformation::{
            GetNativeSystemInfo, OSVERSIONINFOEXW, OSVERSIONINFOW,
            PROCESSOR_ARCHITECTURE_AMD64, PROCESSOR_ARCHITECTURE_ARM, PROCESSOR_ARCHITECTURE_ARM64,
            PROCESSOR_ARCHITECTURE_INTEL, SYSTEM_INFO,
        },
        WindowsProgramming::GetComputerNameW,
    },
    core::PWSTR,
};

use crate::Uname;

#[link(name = "ntdll")]
unsafe extern "system" {
    fn RtlGetVersion(lpVersionInformation: *mut OSVERSIONINFOW) -> i32;
}

pub fn get_platform_info() -> Result<Uname, String> {
    unsafe {
        let mut buffer = [0u16; 256];
        let mut size = buffer.len() as u32;

        if GetComputerNameW(Some(PWSTR(buffer.as_mut_ptr())), &mut size).is_err() {
            return Err("Failed to get computer name".to_string());
        }

        let node_name = String::from_utf16_lossy(&buffer[..size as usize]);

        let mut sys_info = SYSTEM_INFO::default();
        GetNativeSystemInfo(&mut sys_info);

        let machine = match sys_info.Anonymous.Anonymous.wProcessorArchitecture {
            PROCESSOR_ARCHITECTURE_AMD64 => "x86_64",
            PROCESSOR_ARCHITECTURE_ARM => "arm",
            PROCESSOR_ARCHITECTURE_ARM64 => "aarch64",
            PROCESSOR_ARCHITECTURE_INTEL => "x86",
            _ => "unknown",
        }
        .to_string();

        let mut os_version = OSVERSIONINFOEXW::default();
        os_version.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOEXW>() as u32;

        let (release, version) =
            if RtlGetVersion(&mut os_version as *mut _ as *mut OSVERSIONINFOW) == 0 {
                let build = os_version.dwBuildNumber;
                let release = format!(
                    "{}.{}.{}",
                    os_version.dwMajorVersion, os_version.dwMinorVersion, build
                );

                let csd_len = os_version
                    .szCSDVersion
                    .iter()
                    .position(|&c| c == 0)
                    .unwrap_or(os_version.szCSDVersion.len());

                let service_pack = String::from_utf16_lossy(&os_version.szCSDVersion[..csd_len]);

                let version = if service_pack.is_empty() {
                    format!("Build {}", build)
                } else {
                    service_pack
                };

                (release, version)
            } else {
                ("unknown".to_string(), "unknown".to_string())
            };

        Ok(Uname {
            sys_name: "Windows_NT".to_string(),
            node_name,
            release,
            version,
            machine,
            os: "Windows".to_string(),
        })
    }
}
