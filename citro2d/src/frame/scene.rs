
use citro2d_sys::{C2D_DrawSprite, C2D_DrawText, C2D_SceneBegin, C2D_SceneSize, C2D_SceneTarget, C2D_TargetClear};

use crate::{frame::Frame, render::Target, sprite::Sprite, text::Text};

/// Mutable singleton to enforce only one instance of Scene at a time.
pub struct Scene<'a> {
    target: &'a mut Target
}

impl Frame {
    #[doc(alias = "C2D_SceneBegin")]
    pub fn start_scene<'a>(&self, target: &'a mut Target) -> Scene<'a> {
        unsafe { C2D_SceneBegin(&mut *target.inner); };

        Scene { target }
    }
}

impl<'a> Scene<'a> {
    #[doc(alias = "C2D_SceneSize")]
    pub fn set_scene_size(&self, width: u32, height: u32, tilt: bool) {
        unsafe { C2D_SceneSize(width, height, tilt); };
    }

    #[doc(alias = "C2D_SceneTarget")]
    pub fn set_target(&'a mut self, target: &'a mut Target) {
        unsafe { C2D_SceneTarget(&mut *target.inner);};
        self.target = target;
    }

    #[doc(alias = "C2D_DrawSprite")]
    pub fn draw_sprite(&self, sprite: &Sprite) {
        unsafe { C2D_DrawSprite(&*sprite.inner); };
    }

    #[doc(alias = "C2D_DrawText")]
    pub fn draw_text(&self, text: &Text) {
        unsafe {
            C2D_DrawText(
                &*text.inner,
                text.flags,
                text.pos.0,
                text.pos.1,
                text.pos.2,
                text.scale.0,
                text.scale.1
            );
        }
    }

    #[doc(alias = "C2D_TargetClear")]
    pub fn fill(&'a mut self, color: u32) {
        unsafe { C2D_TargetClear(&mut *self.target.inner, color); };
    }

    /// Ends and drops the scene, marking active_scene in the Frame as false.
    /// Essentially allows new scene instances to be created.
    pub fn end(self) {
        drop(self);
    }
}
