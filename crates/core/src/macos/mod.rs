mod cf;

use cf::{CFStringRef, CfString};

use crate::Error;

#[link(name = "IOKit", kind = "framework")]
unsafe extern "C" {
    fn IOPMAssertionCreateWithName(
        assertion_type: CFStringRef,
        assertion_level: u32,
        assertion_name: CFStringRef,
        assertion_id: *mut u32,
    ) -> i32;

    fn IOPMAssertionRelease(assertion_id: u32) -> i32;
}

const K_IOPM_ASSERTION_LEVEL_ON: u32 = 255;
const ASSERTION_TYPE: &str = "PreventUserIdleSystemSleep";

pub(crate) fn acquire(name: &str) -> Result<u32, Error> {
    let cf_name = CfString::new(name)?;
    let cf_type = CfString::new(ASSERTION_TYPE)?;

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

    Ok(assertion_id)
}

pub(crate) fn release(id: u32) -> Result<(), Error> {
    let result = unsafe { IOPMAssertionRelease(id) };

    if result != 0 {
        return Err(Error::AssertionFailure(format!(
            "assertion release failed with error code {result:#x}"
        )));
    }

    Ok(())
}
