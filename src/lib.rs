extern crate alloc;

use core::fmt::{Debug, Display, Formatter};
use std::error::Error;

pub mod epd2in13d;
pub mod ffi;

pub struct TrivialError(&'static str);

impl From<&'static str> for TrivialError {
    fn from(value: &'static str) -> Self {
        Self(value)
    }
}

impl Debug for TrivialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Debug::fmt(self.0, f)
    }
}

impl Display for TrivialError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        Display::fmt(self.0, f)
    }
}

impl Error for TrivialError {}

pub mod dev {
    use crate::{ffi, TrivialError};

    pub fn delay_ms(ms: f64) {
        unsafe {
            ffi::DEV_Delay_ms(ms);
        }
    }

    pub fn module_init() -> Result<(), TrivialError> {
        unsafe {
            if ffi::DEV_Module_Init() != 0 {
                return Err("DEV_Module_Init failed".into());
            }
        }
        Ok(())
    }
}
