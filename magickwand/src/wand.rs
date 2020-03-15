use magickwand_bindgen;
use std::sync::Once;

static GENESIS: Once = Once::new();

fn magick_wand_genesis() {
    GENESIS.call_once(|| unsafe {
        magickwand_bindgen::MagickWandGenesis();
    });
}

pub struct Wand {
    ptr: *mut magickwand_bindgen::MagickWand,
}

impl Wand {
    pub fn new() -> Wand {
        magick_wand_genesis();
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
