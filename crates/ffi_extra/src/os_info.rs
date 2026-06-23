pub fn get_ppid() -> u32 {
    #[cfg(unix)]
    {
        unsafe { libc::getppid() as u32 }
    }
    #[cfg(windows)]
    {
        unsafe { windows_sys::Win32::System::Threading::GetCurrentProcessId() }
    }
}

pub fn get_hostname() -> String {
    #[cfg(unix)]
    {
        let mut buf = [0u8; 256];
        let ret = unsafe { libc::gethostname(buf.as_mut_ptr() as *mut libc::c_char, buf.len()) };
        if ret == 0 {
            if let Ok(s) = std::ffi::CStr::from_ptr(buf.as_ptr() as *const libc::c_char).to_str() {
                return s.to_string();
            }
        }
        String::new()
    }
    #[cfg(windows)]
    {
        std::env::var("COMPUTERNAME").unwrap_or_default()
    }
}
