use magickwand_bindgen;
use magickwand_bindgen::MagickBooleanType_MagickFalse as MagickFalse;
use magickwand_bindgen::MagickBooleanType_MagickTrue as MagickTrue;

use std::ffi::CString;
use std::sync::Once;

use crate::enums;
use crate::error;
use crate::pixel;

/// File is an abstraction of FILE in C language.
pub struct File {
    ptr: *mut magickwand_bindgen::FILE,
}

impl File {
    pub fn new(filename: &str, mode: &str) -> Self {
        let c_filename = CString::new(filename).expect("Cstring::new filename");
        let c_mode = CString::new(mode).expect("Cstring::new mode");
        let ptr = unsafe { magickwand_bindgen::fopen(c_filename.as_ptr(), c_mode.as_ptr()) };
        File { ptr }
    }
}

impl Drop for File {
    fn drop(&mut self) {
        unsafe {
            magickwand_bindgen::fclose(self.ptr);
        }
    }
}

static GENESIS: Once = Once::new();

fn magick_wand_genesis() {
    GENESIS.call_once(|| unsafe {
        magickwand_bindgen::MagickWandGenesis();
    });
}

// TODO: automatically called after the all of Wand resources are dropped
pub fn magick_wand_terminus() {
    unsafe {
        magickwand_bindgen::MagickWandTerminus();
    }
}

/// Wand treats with a concept of *image* and provides some associated methods.
///
/// See [Magick Wand Methods](https://imagemagick.org/api/magick-wand.php)
/// and [Magick Wand Image Methods](https://imagemagick.org/api/magick-image.php)
/// documentation for more details.
pub struct Wand {
    ptr: *mut magickwand_bindgen::MagickWand,
}

impl Wand {
    pub fn new() -> Self {
        magick_wand_genesis();
        let ptr = unsafe { magickwand_bindgen::NewMagickWand() };
        Wand { ptr }
    }

    pub fn clone_magick_wand(&self) -> Self {
        let ptr = unsafe { magickwand_bindgen::CloneMagickWand(self.ptr) };
        Wand { ptr }
    }

    pub fn clear_magick_wand(&mut self) {
        unsafe { magickwand_bindgen::ClearMagickWand(self.ptr) };
    }

    pub fn magick_reset_iterator(&mut self) {
        unsafe {
            magickwand_bindgen::MagickResetIterator(self.ptr);
        }
    }

    pub fn magick_next_image(&mut self) -> Option<()> {
        let has_next = unsafe { magickwand_bindgen::MagickNextImage(self.ptr) };

        if has_next == MagickTrue {
            Some(())
        } else {
            None
        }
    }

    pub fn magick_read_image_file(&mut self, file: &mut File) -> Result<(), error::ExceptionType> {
        let status = unsafe { magickwand_bindgen::MagickReadImageFile(self.ptr, file.ptr) };

        self.magick_result(status)
    }

    pub fn magick_write_image_file(&mut self, file: &mut File) -> Result<(), error::ExceptionType> {
        let status = unsafe { magickwand_bindgen::MagickWriteImageFile(self.ptr, file.ptr) };

        self.magick_result(status)
    }

    pub fn magick_write_images_file(
        &mut self,
        file: &mut File,
    ) -> Result<(), error::ExceptionType> {
        let status = unsafe { magickwand_bindgen::MagickWriteImagesFile(self.ptr, file.ptr) };

        self.magick_result(status)
    }

    pub fn magick_resize_image(
        &mut self,
        columns: magickwand_bindgen::size_t,
        rows: magickwand_bindgen::size_t,
        filter: enums::FilterType,
    ) -> Result<(), error::ExceptionType> {
        let status = unsafe {
            magickwand_bindgen::MagickResizeImage(self.ptr, columns, rows, filter.into())
        };

        self.magick_result(status)
    }

    pub fn magick_set_image_gravity(
        &self,
        gravity_type: enums::GravityType,
    ) -> Result<(), error::ExceptionType> {
        let status =
            unsafe { magickwand_bindgen::MagickSetImageGravity(self.ptr, gravity_type.into()) };

        self.magick_result(status)
    }

    pub fn magick_set_image_background_color(
        &mut self,
        pixel: &pixel::Pixel,
    ) -> Result<(), error::ExceptionType> {
        let status =
            unsafe { magickwand_bindgen::MagickSetImageBackgroundColor(self.ptr, pixel.ptr) };

        self.magick_result(status)
    }

    pub fn magick_reset_image_page(&mut self, page: &str) -> Result<(), error::ExceptionType> {
        let c_page = CString::new(page).expect("CString::new(page) should be created");
        let status = unsafe { magickwand_bindgen::MagickResetImagePage(self.ptr, c_page.as_ptr()) };

        self.magick_result(status)
    }

    /// Simulates an image shadow.
    ///
    /// - `alpha`: percentage transparency.
    /// - `sigma`: the standard deviation of the Gaussian, in pixels.
    /// - `x`: the shadow x-offset.
    /// - `y`: the shadow y-offset.
    pub fn magick_shadow_image(
        &mut self,
        alpha: f64,
        sigma: f64,
        x: magickwand_bindgen::ssize_t,
        y: magickwand_bindgen::ssize_t,
    ) -> Result<(), error::ExceptionType> {
        let status = unsafe { magickwand_bindgen::MagickShadowImage(self.ptr, alpha, sigma, x, y) };

        self.magick_result(status)
    }

    pub fn magick_composite_image(
        &mut self,
        source: &Wand,
        operator: enums::CompositeOperator,
        clip_to_self: bool,
        x: magickwand_bindgen::ssize_t,
        y: magickwand_bindgen::ssize_t,
    ) -> Result<(), error::ExceptionType> {
        let clip_to_self = if clip_to_self {
            MagickTrue
        } else {
            MagickFalse
        };

        let status = unsafe {
            magickwand_bindgen::MagickCompositeImage(
                self.ptr,
                source.ptr,
                operator.into(),
                clip_to_self,
                x,
                y,
            )
        };

        self.magick_result(status)
    }

    pub fn magick_composite_image_gravity(
        &mut self,
        source: &Wand,
        operator: enums::CompositeOperator,
        gravity_type: enums::GravityType,
    ) -> Result<(), error::ExceptionType> {
        let status = unsafe {
            magickwand_bindgen::MagickCompositeImageGravity(
                self.ptr,
                source.ptr,
                operator.into(),
                gravity_type.into(),
            )
        };

        self.magick_result(status)
    }

    fn magick_result(
        &self,
        status: magickwand_bindgen::MagickBooleanType,
    ) -> Result<(), error::ExceptionType> {
        if status == MagickFalse {
            Err(self.magick_get_exception_type())
        } else {
            Ok(())
        }
    }

    fn magick_get_exception_type(&self) -> error::ExceptionType {
        let exception_type = unsafe { magickwand_bindgen::MagickGetExceptionType(self.ptr) };
        error::get_exception_type(exception_type)
    }
}

impl Default for Wand {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Wand {
    fn drop(&mut self) {
        unsafe {
            magickwand_bindgen::ClearMagickWand(self.ptr);
            self.ptr = magickwand_bindgen::DestroyMagickWand(self.ptr);
        }
    }
}
