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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alive_for_own_process() {
        assert!(is_alive(std::process::id().cast_signed()));
    }

    #[test]
    fn alive_for_unsignalable_process() {
        assert!(is_alive(1));
    }

    #[test]
    fn dead_after_child_is_reaped() {
        let mut child = std::process::Command::new("true").spawn().unwrap();
        let pid = child.id().cast_signed();
        child.wait().unwrap();
        assert!(!is_alive(pid));
    }
}
