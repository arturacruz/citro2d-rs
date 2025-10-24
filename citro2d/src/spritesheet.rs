use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_SpriteSheet, C2D_SpriteSheetCount, C2D_SpriteSheetFree, C2D_SpriteSheetGetImage, C2D_SpriteSheetLoad};

use crate::{image::Image, sprite::Sprite};

pub enum SpritesheetError {
    Load
}

pub struct Spritesheet {
    inner: C2D_SpriteSheet
}

impl Spritesheet {
    pub fn new(filename: &str) -> Result<Self, SpritesheetError> {
        let path = match CString::from_str(filename) {
            Ok(p) => p.as_ptr(),
            Err(_) => return Err(SpritesheetError::Load),
        };
        let inner = unsafe { C2D_SpriteSheetLoad(path) };

        if inner.is_null() {
            return Err(SpritesheetError::Load);
        }

        Ok(Spritesheet { inner })
    }

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

    pub fn size(&self) -> usize {
        unsafe { C2D_SpriteSheetCount(self.inner) }
    }
}

impl Drop for Spritesheet {
    fn drop(&mut self) {
        unsafe { C2D_SpriteSheetFree(self.inner); };
    }
}
