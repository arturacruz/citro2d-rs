use citro2d_sys::{C2D_CreateScreenTarget, C3D_RenderTarget};

use crate::{base::Citro2D};

#[doc(alias = "C3D_RenderTarget")]
pub struct Target {
    pub inner: Box<C3D_RenderTarget>
}

pub enum TargetError {
    Init2D,
}

pub enum Screen { 
    TopLeft, TopRight, Bottom
}

impl Citro2D {
    #[doc(alias = "C2D_CreateScreenTarget")]
    pub fn create_target(&self, screen: Screen) -> Result<Target, TargetError> {
        let (screen, side) = match screen {
            Screen::TopLeft => (0, 0),
            Screen::TopRight => (0, 1),
            Screen::Bottom => (1, 0)  
        };
    
        let ptr = unsafe { C2D_CreateScreenTarget(screen, side) };

        if ptr.is_null() {
            return Err(TargetError::Init2D)
        }
        let inner = unsafe { Box::from_raw(ptr) };
        Ok(Target { inner })
    }
}
