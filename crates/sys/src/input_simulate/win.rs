use std::ffi::c_void;

use crate::error::Error;

use super::key::Key;

const KEY_EVENT: u16 = 0x0001;
const GENERIC_READ: u32 = 0x80000000;
const GENERIC_WRITE: u32 = 0x40000000;
const FILE_SHARE_READ: u32 = 0x00000001;
const FILE_SHARE_WRITE: u32 = 0x00000002;
const OPEN_EXISTING: u32 = 3;
const INVALID_HANDLE_VALUE: *mut c_void = -1isize as *mut c_void;

#[repr(C)]
#[derive(Clone, Copy)]
struct InputRecord {
    event_type: u16,
    key_event: KeyEventRecord,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct KeyEventRecord {
    key_down: i32,
    repeat_count: u16,
    virtual_key_code: u16,
    virtual_scan_code: u16,
    unicode_char: u16,
    control_key_state: u32,
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn CreateFileW(
        lp_file_name: *const u16,
        dw_desired_access: u32,
        dw_share_mode: u32,
        lp_security_attributes: *mut c_void,
        dw_creation_disposition: u32,
        dw_flags_and_attributes: u32,
        h_template_file: *mut c_void,
    ) -> *mut c_void;
    fn CloseHandle(h_object: *mut c_void) -> i32;
    fn WriteConsoleInputW(
        h_console_input: *mut c_void,
        lp_buffer: *const InputRecord,
        n_length: u32,
        lp_number_of_events_written: *mut u32,
    ) -> i32;
}

pub fn inject_keys(keys: &[&str]) -> Result<(), Error> {
    let parsed: Vec<Key> = keys
        .iter()
        .map(|s| Key::from_str(s))
        .collect::<Result<_, _>>()?;

    let records: Vec<InputRecord> = parsed
        .iter()
        .map(|k| InputRecord {
            event_type: KEY_EVENT,
            key_event: KeyEventRecord {
                key_down: 1,
                repeat_count: 1,
                virtual_key_code: k.vk_code,
                virtual_scan_code: 0,
                unicode_char: k.unicode_char,
                control_key_state: 0,
            },
        })
        .collect();

    unsafe {
        let conin: Vec<u16> = "CONIN$\0".encode_utf16().collect();
        let handle = CreateFileW(
            conin.as_ptr(),
            GENERIC_READ | GENERIC_WRITE,
            FILE_SHARE_READ | FILE_SHARE_WRITE,
            std::ptr::null_mut(),
            OPEN_EXISTING,
            0,
            std::ptr::null_mut(),
        );

        if handle == INVALID_HANDLE_VALUE {
            let io_err = std::io::Error::last_os_error();
            return Err(Error::Internal(format!(
                "CreateFileW(CONIN$) failed: {io_err}"
            )));
        }

        let mut written = 0u32;
        let result =
            WriteConsoleInputW(handle, records.as_ptr(), records.len() as u32, &mut written);

        CloseHandle(handle);

        if result == 0 {
            let io_err = std::io::Error::last_os_error();
            return Err(Error::Internal(format!(
                "WriteConsoleInputW failed: {io_err}"
            )));
        }
    }

    Ok(())
}
