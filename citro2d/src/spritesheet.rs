use std::error::Error;
use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_SpriteSheet, C2D_SpriteSheetCount, C2D_SpriteSheetFree, C2D_SpriteSheetGetImage, C2D_SpriteSheetLoad};

use crate::Citro2DError;
use crate::{image::Image, sprite::Sprite};

#[derive(Debug)]
pub enum SpritesheetError {
    Load, NullPtr
}

pub struct Spritesheet {
    pub(super) inner: C2D_SpriteSheet
}

impl Spritesheet {
    #[doc(alias = "C2D_SpriteSheetLoad")]
    /// Expects a ROMFS to be initialized
    pub fn new(filename: &str) -> Result<Self, impl Error> {
        let path = CString::from_str(filename)?;
        let inner = unsafe { C2D_SpriteSheetLoad(path.as_ptr()) };

        if inner.is_null() {
            return Err(Citro2DError::FileNotFound);
        }

        Ok(Spritesheet { inner })
    }

    #[doc(alias = "C2D_SpriteSheetGetImage")]
    pub fn get_image(&self, index: usize) -> Option<Image> {
        if index > self.size() {
            return None;
        }

        let img = unsafe { C2D_SpriteSheetGetImage(self.inner, index) };
        if img.tex.is_null() || img.subtex.is_null() {
            return None;
        }

        Some(Image::from(img))
    }

    pub fn get_sprite(&self, index: usize) -> Option<Sprite> {
        let img = self.get_image(index)?;
        Some(img.into())
    }

    #[doc(alias = "C2D_SpriteSheetCount")]
    pub fn size(&self) -> usize {
        unsafe { C2D_SpriteSheetCount(self.inner) }
    }
}

impl Drop for Spritesheet {
    fn drop(&mut self) {
        unsafe { C2D_SpriteSheetFree(self.inner); };
    }
}
