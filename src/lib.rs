extern crate alloc;

pub mod epd2in13d;
pub mod ffi;

pub mod dev {
    use crate::ffi;

    pub fn delay_ms(ms: f64) {
        unsafe {
            ffi::DEV_Delay_ms(ms);
        }
    }

    pub fn module_init() {
        unsafe {
            ffi::DEV_Module_Init();
        }
    }
}
