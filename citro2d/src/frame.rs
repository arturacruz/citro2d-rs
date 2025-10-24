use citro2d_sys::{C2D_Flush, C2D_SceneSize, C2D_SceneTarget, C2D_TargetClear, C3D_FrameBegin, C3D_FrameEnd, C3D_FRAME_SYNCDRAW};

use crate::{base::Citro2D, color::Color, render::Target};

// TODO: Put ALL drawing functions in the frame struct to enforce starting the frame in order to
// draw.

pub struct Frame<'a> {
    citro: &'a mut Citro2D
}

impl Citro2D {
    #[doc(alias = "C3D_FrameBegin")]
    /// If drawing is true, a frame is already being drawn.
    /// This way, only a single instance of Frame can be created.
    pub fn begin_frame<'a>(&'a mut self) -> Option<Frame<'a>> {
        if self.drawing {
            return None;
        }

        self.drawing = true;
        match unsafe { C3D_FrameBegin(C3D_FRAME_SYNCDRAW) } {
            true => Some(Frame { citro: self }),
            false => None
        }
    }
}

impl<'a> Drop for Frame<'a> {
    #[doc(alias = "C3D_FrameEnd")]
    /// Sets drawing to false so that a new Frame can be created.
    fn drop(&mut self) {
        self.citro.drawing = false;
        unsafe { C3D_FrameEnd(0); };
    }
}

impl<'a> Frame<'a> {
    #[doc(alias = "C2D_Flush")]
    pub fn flush(&self) {
        unsafe { C2D_Flush(); };
    }

    #[doc(alias = "C2D_SceneSize")]
    pub fn set_scene_size(&self, width: u32, height: u32, tilt: bool) {
        unsafe { C2D_SceneSize(width, height, tilt); };
    }

    pub fn clear_target(&self, target: &mut Target, color: u32) {
        unsafe { C2D_TargetClear(&mut *target.inner, color); };
    }

    #[doc(alias = "C2D_SceneTarget")]
    pub fn set_scene_target(&self, target: &mut Target) {
        unsafe { C2D_SceneTarget(&mut *target.inner);}
    }
}
