use std::os::unix::io::RawFd;
use std::sync::OnceLock;

use crate::error::Error;

static MASTER_FD: OnceLock<RawFd> = OnceLock::new();

fn ensure_pty() -> Result<RawFd, Error> {
    if let Some(&fd) = MASTER_FD.get() {
        return Ok(fd);
    }

    let mut master: libc::c_int = 0;
    let mut slave: libc::c_int = 0;

    let ret = unsafe {
        libc::openpty(
            &mut master,
            &mut slave,
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        )
    };

    if ret != 0 {
        let io_err = std::io::Error::last_os_error();
        return Err(Error::Internal(format!("openpty failed: {io_err}")));
    }

    let ret = unsafe { libc::dup2(slave, 0) };
    if ret < 0 {
        unsafe {
            libc::close(master);
            libc::close(slave);
        }
        let io_err = std::io::Error::last_os_error();
        return Err(Error::Internal(format!(
            "dup2 slave to stdin failed: {io_err}"
        )));
    }

    unsafe { libc::close(slave) };

    unsafe {
        let mut termios: libc::termios = std::mem::zeroed();
        libc::tcgetattr(0, &mut termios);
        libc::cfmakeraw(&mut termios);
        libc::tcsetattr(0, libc::TCSANOW, &termios);
    }

    let _ = MASTER_FD.set(master);
    Ok(master)
}

pub fn init() -> Result<(), Error> {
    ensure_pty()?;
    Ok(())
}

pub fn inject_keys(keys: &[&str]) -> Result<(), Error> {
    let bytes = keys_to_bytes(keys)?;
    let master = ensure_pty()?;

    let ret = unsafe { libc::write(master, bytes.as_ptr() as *const libc::c_void, bytes.len()) };
    if ret < 0 {
        let io_err = std::io::Error::last_os_error();
        return Err(Error::Internal(format!(
            "write to PTY master failed: {io_err}"
        )));
    }

    Ok(())
}

fn keys_to_bytes(keys: &[&str]) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();

    for key in keys {
        match key.to_lowercase().as_str() {
            "enter" | "return" => bytes.push(b'\r'),
            "escape" | "esc" => bytes.push(0x1b),
            "tab" => bytes.push(b'\t'),
            "backspace" => bytes.push(0x7f),
            "delete" | "del" => bytes.extend_from_slice(b"\x1b[3~"),
            "up" => bytes.extend_from_slice(b"\x1b[A"),
            "down" => bytes.extend_from_slice(b"\x1b[B"),
            "right" => bytes.extend_from_slice(b"\x1b[C"),
            "left" => bytes.extend_from_slice(b"\x1b[D"),
            "home" => bytes.extend_from_slice(b"\x1b[H"),
            "end" => bytes.extend_from_slice(b"\x1b[F"),
            "pageup" => bytes.extend_from_slice(b"\x1b[5~"),
            "pagedown" => bytes.extend_from_slice(b"\x1b[6~"),
            "insert" => bytes.extend_from_slice(b"\x1b[2~"),
            "space" => bytes.push(b' '),
            s if s.chars().count() == 1 => {
                let c = s.chars().next().unwrap();
                bytes.push(c as u8);
            }
            other => return Err(Error::UnknownKey(other.to_string())),
        }
    }

    Ok(bytes)
}
