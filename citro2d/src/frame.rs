use citro2d_sys::{C3D_FrameBegin, C3D_FrameEnd, C3D_FRAME_SYNCDRAW};

use crate::base::Citro2D;

// TODO: Put ALL drawing functions in the frame struct to enforce starting the frame in order to
// draw.

pub struct Frame<'a> {
    citro: &'a Citro2D
}

impl Citro2D {
    #[doc(alias = "C3D_")]
    pub fn begin_frame<'a>(&'a self) -> Option<Frame<'a>> {
        match unsafe { C3D_FrameBegin(C3D_FRAME_SYNCDRAW) } {
            true => Some(Frame { citro: &self }),
            false => None
        }
    }
}

impl<'a> Drop for Frame<'a> {
    fn drop(&mut self) {
        unsafe { C3D_FrameEnd(0); };
    }
}

