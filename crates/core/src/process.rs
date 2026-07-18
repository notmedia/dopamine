use std::ffi::c_int;

unsafe extern "C" {
    fn kill(pid: c_int, sig: c_int) -> c_int;
}

#[must_use]
pub fn is_alive(pid: i32) -> bool {
    const EPERM: c_int = 1;

    let result = unsafe { kill(pid, 0) };

    if result == 0 {
        return true;
    }

    std::io::Error::last_os_error().raw_os_error() == Some(EPERM)
}
