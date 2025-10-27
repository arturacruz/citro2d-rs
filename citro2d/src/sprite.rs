use citro2d_sys::{C2D_Sprite, C2D_SpriteFromImage, C2D_SpriteMove, C2D_SpriteRotate, C2D_SpriteRotateDegrees, C2D_SpriteScale, C2D_SpriteSetCenter, C2D_SpriteSetCenterRaw, C2D_SpriteSetDepth, C2D_SpriteSetPos, C2D_SpriteSetRotation, C2D_SpriteSetRotationDegrees, C2D_SpriteSetScale};

use crate::image::Image;

#[doc(alias = "C2D_Sprite")]
pub struct Sprite {
    pub(super) inner: Box<C2D_Sprite>
}

impl From<Image> for Sprite {
    fn from(image: Image) -> Self {
        let mut inner = Box::<C2D_Sprite>::new_uninit();
        unsafe { C2D_SpriteFromImage(inner.as_mut_ptr(), image.inner()); };

        Sprite { inner: unsafe { inner.assume_init() } }
    }
}

impl Sprite {
    #[doc(alias = "C2D_SpriteScale")]
    pub fn scale(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteScale(&mut *self.inner, x, y); };
    }

    #[doc(alias = "C2D_SpriteSetScale")]
    pub fn set_scale(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteSetScale(&mut *self.inner, x, y); };
    }

    #[doc(alias = "C3D_SpriteRotate")]
    pub fn rotate(&mut self, radians: f32) {
        unsafe { C2D_SpriteRotate(&mut *self.inner, radians); };
    }

    #[doc(alias = "C2D_SpriteRotateDegrees")]
    pub fn rotate_degrees(&mut self, degrees: f32) {
        unsafe { C2D_SpriteRotateDegrees(&mut *self.inner, degrees); };
    }

    #[doc(alias = "C2D_SpriteSetRotation")]
    pub fn set_rotation(&mut self, radians: f32) {
        unsafe { C2D_SpriteSetRotation(&mut *self.inner, radians); };
    }

    #[doc(alias = "C2D_SpriteSetRotationDegrees")]
    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        unsafe { C2D_SpriteSetRotationDegrees(&mut *self.inner, degrees); };
    }

    #[doc(alias = "C2D_SpriteSetCenter")]
    pub fn set_center(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteSetCenter(&mut *self.inner, x, y); };
    }

    #[doc(alias = "C2D_SpriteSetCenterRaw")]
    pub fn set_center_raw(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteSetCenterRaw(&mut *self.inner, x, y); };
    }

    #[doc(alias = "C2D_SpriteSetPos")]
    pub fn set_pos(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteSetPos(&mut *self.inner, x, y); };
    }

    #[doc(alias = "C2D_SpriteSetDepth")]
    pub fn set_depth(&mut self, depth: f32) {
        unsafe { C2D_SpriteSetDepth(&mut *self.inner, depth); };
    }

    #[doc(alias = "C2D_SpriteMove")]
    pub fn mov(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteMove(&mut *self.inner, x, y); };
    }
}
