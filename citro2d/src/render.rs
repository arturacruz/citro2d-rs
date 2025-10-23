use std::ptr::NonNull;

use citro2d_sys::{C2D_CreateScreenTarget, C3D_RenderTarget};

pub struct RenderTarget {
    ptr: NonNull<C3D_RenderTarget>

}

pub enum Screen {
    TopLeft, TopRight, Bottom
}


impl RenderTarget {
    pub fn new(screen: Screen) {
        let (screen, side) = match screen {
            Screen::TopLeft => (0, 0),
            Screen::TopRight => (0, 1),
            Screen::Bottom => (1, 0)
        };
    
        let a = unsafe { C2D_CreateScreenTarget(screen, side) };
    }
}
