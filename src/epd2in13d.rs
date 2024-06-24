use crate::ffi;

pub struct Epd2in13d;

impl Epd2in13d {
    pub fn init(&self) {
        unsafe {
            ffi::EPD_2IN13D_Init();
        }
    }

    pub fn clear(&self) {
        unsafe {
            ffi::EPD_2IN13D_Clear();
        }
    }

    pub fn display(&self, image: &[u8]) {
        unsafe {
            // SAFETY: the callee should guarantee `image` won't
            // be mutated even though its C signature has no `const`
            ffi::EPD_2IN13D_Display(image.as_ptr() as *mut u8);
        }
    }

    pub fn display_part(&self, image: &[u8]) {
        unsafe {
            // SAFETY: the callee should guarantee `image` won't
            // be mutated even though its C signature has no `const`
            ffi::EPD_2IN13D_DisplayPart(image.as_ptr() as *mut u8);
        }
    }

    pub fn sleep(&self) {
        unsafe {
            ffi::EPD_2IN13D_Sleep();
        }
    }
}
