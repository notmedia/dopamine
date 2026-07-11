// Core Foundation
use std::ffi::{CString, c_char, c_void};

use crate::Error;

const K_CFSTRING_ENCODING_UTF8: u32 = 0x0800_0100;

#[repr(C)]
pub(super) struct CFString {
    _opaque: [u8; 0],
}

pub(super) type CFStringRef = *const CFString;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(
        alloc: *const c_void,
        c_str: *const c_char,
        encoding: u32,
    ) -> CFStringRef;

    fn CFRelease(cf: *const c_void);
}

pub(super) struct CfString(CFStringRef);

impl CfString {
    pub(super) fn new(s: &str) -> Result<Self, Error> {
        let cstr = CString::new(s)
            .map_err(|_| Error::AssertionFailure("assertion name contains a NUL byte".into()))?;

        let cf_str = unsafe {
            CFStringCreateWithCString(std::ptr::null(), cstr.as_ptr(), K_CFSTRING_ENCODING_UTF8)
        };

        if cf_str.is_null() {
            return Err(Error::AssertionFailure(
                "CFStringCreateWithCString returned null".into(),
            ));
        }

        Ok(Self(cf_str))
    }

    pub(super) fn as_ptr(&self) -> CFStringRef {
        self.0
    }
}

impl Drop for CfString {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_ptr().cast());
        }
    }
}
