use magickwand_bindgen;
use magickwand_bindgen::MagickBooleanType_MagickFalse as MagickFalse;
// use magickwand_bindgen::MagickBooleanType_MagickTrue as MagickTrue;

use std::ffi::CString;

use crate::error;

/// Pixel treats with a concept of *color*.
///
/// See [Pixel Wand Methods](https://imagemagick.org/www/api/pixel-wand.php)
/// documentation for more details.
pub struct Pixel {
    pub(crate) ptr: *mut magickwand_bindgen::PixelWand,
}

impl Pixel {
    pub fn new() -> Self {
        let ptr = unsafe { magickwand_bindgen::NewPixelWand() };
        Pixel { ptr }
    }

    pub fn clear_pixel_wand(&mut self) {
        unsafe { magickwand_bindgen::ClearPixelWand(self.ptr) };
    }

    pub fn pixel_set_color(&mut self, color: &str) -> Result<(), error::ExceptionType> {
        let c_color = CString::new(color).expect("CString::new color");
        let status = unsafe { magickwand_bindgen::PixelSetColor(self.ptr, c_color.as_ptr()) };

        if status == MagickFalse {
            Err(self.pixel_get_exception_type())
        } else {
            Ok(())
        }
    }

    /// the level of transparency: alpha 1.0 is fully opaque and 0.0 is fully transparent.
    pub fn pixel_set_alpha(&mut self, alpha: f64) {
        unsafe { magickwand_bindgen::PixelSetAlpha(self.ptr, alpha) };
    }

    fn pixel_get_exception_type(&self) -> error::ExceptionType {
        let exception_type = unsafe { magickwand_bindgen::PixelGetExceptionType(self.ptr) };
        error::get_exception_type(exception_type)
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Pixel {
    fn drop(&mut self) {
        unsafe {
            self.ptr = magickwand_bindgen::DestroyPixelWand(self.ptr);
        }
    }
}
