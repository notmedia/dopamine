// Core Foundation
use std::ffi::{CString, NulError, c_char, c_void};

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

#[derive(Debug)]
pub(super) enum CfStringError {
    NulByte(NulError),
    CreationFailed,
}

impl std::error::Error for CfStringError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            CfStringError::NulByte(err) => Some(err),
            CfStringError::CreationFailed => None,
        }
    }
}

impl std::fmt::Display for CfStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CfStringError::NulByte(_) => write!(f, "failed to convert to a C string"),
            CfStringError::CreationFailed => write!(f, "CFStringCreateWithCString returned null"),
        }
    }
}

impl From<NulError> for CfStringError {
    fn from(err: NulError) -> Self {
        CfStringError::NulByte(err)
    }
}

pub(super) struct CfString(CFStringRef);

impl CfString {
    pub(super) fn new(s: &str) -> Result<Self, CfStringError> {
        let cstr = CString::new(s)?;

        let cf_str = unsafe {
            CFStringCreateWithCString(std::ptr::null(), cstr.as_ptr(), K_CFSTRING_ENCODING_UTF8)
        };

        if cf_str.is_null() {
            return Err(CfStringError::CreationFailed);
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
