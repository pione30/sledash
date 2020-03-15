use magickwand_bindgen;

pub struct Wand {
    ptr: *mut magickwand_bindgen::MagickWand,
}

impl Wand {
    pub fn new() -> Wand {
        let ptr = unsafe { magickwand_bindgen::NewMagickWand() };
        Wand { ptr }
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
            magickwand_bindgen::DestroyMagickWand(self.ptr);
        }
    }
}
