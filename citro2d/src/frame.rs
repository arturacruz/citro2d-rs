use std::sync::Mutex;

use citro2d_sys::{C2D_Flush, C2D_TargetClear, C3D_FrameBegin, C3D_FrameEnd, C3D_FRAME_SYNCDRAW};

use crate::{base::Citro2D, render::Target};

pub mod scene;

/// Mutable singleton to enforce only one instance of Frame at a time.
pub static ACTIVE_FRAME: Mutex<bool> = Mutex::new(false);

#[derive(Debug)]
pub struct Frame;

impl Citro2D {
    #[doc(alias = "C3D_FrameBegin")]
    /// If drawing is true, a frame is already being drawn.
    /// This way, only a single instance of Frame can be created.
    pub fn begin_frame(&self) -> Option<Frame> {
        let mut active = ACTIVE_FRAME
            .lock()
            .expect("[Citro2D] Failed to get current state of Frame.");

        if *active {
            return None;
        }

        match unsafe { C3D_FrameBegin(C3D_FRAME_SYNCDRAW) } {
            true => {
                *active = true;
                Some(Frame {})
            }
            false => None,
        }
    }
}

impl Drop for Frame {
    #[doc(alias = "C3D_FrameEnd")]
    /// Sets drawing to false so that a new Frame can be created.
    fn drop(&mut self) {
        let mut active = ACTIVE_FRAME
            .lock()
            .expect("[Citro2D] Failed to get current state of Frame.");
        *active = false;
        unsafe { C3D_FrameEnd(0); };
    }
}

impl Frame {
    #[doc(alias = "C2D_Flush")]
    pub fn flush(&self) {
        unsafe { C2D_Flush(); };
    }

    #[doc(alias = "C2D_TargetClear")]
    pub fn fill_target(&self, target: &mut Target, color: u32) {
        unsafe { C2D_TargetClear(&mut *target.inner, color); };
    }
}
