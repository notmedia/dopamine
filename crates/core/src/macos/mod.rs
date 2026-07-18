mod cf;

use std::ffi::c_int;

use cf::{CFStringRef, CfString, CfStringError};

use crate::{Config, Error};

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: u32,
        assertion_name: CFStringRef,
        assertion_id: *mut u32,
    ) -> c_int;

    fn IOPMAssertionRelease(assertion_id: u32) -> c_int;
}

const K_IOPM_ASSERTION_LEVEL_ON: u32 = 255;

#[derive(Clone, Copy)]
enum AssertionType {
    Idle,
    Display,
}

impl AssertionType {
    fn as_str(self) -> &'static str {
        match self {
            AssertionType::Idle => "PreventUserIdleSystemSleep",
            AssertionType::Display => "PreventUserIdleDisplaySleep",
        }
    }
}

impl From<CfStringError> for Error {
    fn from(err: CfStringError) -> Self {
        match err {
            CfStringError::NulByte(e) => Error::InvalidName(e),
            CfStringError::CreationFailed => Error::AssertionFailure(err.to_string()),
        }
    }
}

pub(crate) struct Assertion {
    id: u32,
}

pub(crate) type Token = Vec<Assertion>;

impl Drop for Assertion {
    fn drop(&mut self) {
        unsafe { IOPMAssertionRelease(self.id) };
    }
}

pub(crate) fn acquire(name: &str, config: &Config) -> Result<Token, Error> {
    let mut assertions: Token = vec![];

    if config.idle {
        assertions.push(acquire_one(name, AssertionType::Idle)?);
    }

    if config.display {
        assertions.push(acquire_one(name, AssertionType::Display)?);
    }

    Ok(assertions)
}

fn acquire_one(name: &str, assertion_type: AssertionType) -> Result<Assertion, Error> {
    let cf_name = CfString::new(name)?;
    let cf_type = CfString::new(assertion_type.as_str())?;

    let mut assertion_id: u32 = 0;

    let result = unsafe {
        IOPMAssertionCreateWithName(
            cf_type.as_ptr(),
            K_IOPM_ASSERTION_LEVEL_ON,
            cf_name.as_ptr(),
            &raw mut assertion_id,
        )
    };

    if result != 0 {
        return Err(Error::AssertionFailure(format!(
            "assertion create failed with error code {result:#x}"
        )));
    }

    Ok(Assertion { id: assertion_id })
}
