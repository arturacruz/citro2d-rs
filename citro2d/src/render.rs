use std::ptr::NonNull;

use citro2d_sys::{C2D_CreateScreenTarget, C3D_RenderTarget};

pub struct Target {
    ptr: NonNull<C3D_RenderTarget>
}

pub enum TargetError {
    Init2D,
}

pub enum Screen { 
    TopLeft, TopRight, Bottom
}


impl Target {
    pub fn new(screen: Screen) -> Result<Target, TargetError> {
        let (screen, side) = match screen {
            Screen::TopLeft => (0, 0),
            Screen::TopRight => (0, 1),
            Screen::Bottom => (1, 0)  
        };
    
        let ptr = match NonNull::new(unsafe { C2D_CreateScreenTarget(screen, side) }) {
            Some(p) => p,
            None => return Err(TargetError::Init2D)
        };

        Ok(Target { ptr })
    }
}
