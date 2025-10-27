use std::sync::Mutex;

use citro2d_sys::{C2D_BotLeft, C2D_BotRight, C2D_Fini, C2D_Init, C2D_Prepare, C2D_TintLuma, C2D_TintMult, C2D_TintSolid, C2D_TopLeft, C2D_TopRight, C3D_Fini, C3D_Init, C2D_DEFAULT_MAX_OBJECTS, C3D_DEFAULT_CMDBUF_SIZE};

static ACTIVE_CITRO: Mutex<bool> = Mutex::new(false);

#[doc(alias = "C2D_TintMode")]
#[repr(u8)]
pub enum TintMode {
    #[doc(alias = "C2D_TintSolid")]
    Solid = C2D_TintSolid,
    #[doc(alias = "C2D_TintMult")]
    Mult = C2D_TintMult,
    #[doc(alias = "C2D_TintLuma")]
    Luma = C2D_TintLuma
}

#[doc(alias = "C2D_Corner")]
#[repr(u8)]
pub enum Corner {
    #[doc(alias = "C2D_TopLeft")]
    TopLeft = C2D_TopLeft,
    #[doc(alias = "C2D_TopRight")]
    TopRight = C2D_TopRight,
    #[doc(alias = "C2D_BotLeft")]
    BotLeft = C2D_BotLeft,
    #[doc(alias = "C2D_BotLeft")]
    BotRight = C2D_BotRight
}

#[derive(Debug)]
pub enum Citro2DError {
    Init3D, Init2D, AlreadyInitialized
}

/// Main Citro2D controller.
#[derive(Debug)]
pub struct Citro2D;

impl Citro2D {
    #[doc(alias = "C2D_Init")]
    pub fn new() -> Result<Self, Citro2DError> {
        Self::init(C3D_DEFAULT_CMDBUF_SIZE, C2D_DEFAULT_MAX_OBJECTS)
    }

    #[doc(alias = "C2D_Init")]
    pub fn from(buffer_size: u32, max_objs: u16) -> Result<Self, Citro2DError> {
        Self::init(buffer_size, max_objs)
    }

    #[doc(alias = "C2D_Init")]
    fn init(buffer_size: u32, max_objs: u16) -> Result<Self, Citro2DError> {
        let mut active = ACTIVE_CITRO
            .lock()
            .expect("[Citro2D] Failed to get current state of Citro.");
        
        if *active {
            return Err(Citro2DError::AlreadyInitialized);
        }
        if !unsafe { C3D_Init(buffer_size as usize) } { 
            return Err(Citro2DError::Init3D);
        }
        if !unsafe { C2D_Init(max_objs as usize) } {
            return Err(Citro2DError::Init2D);
        }
        unsafe {
            C2D_Prepare();
        }

        *active = true;
        Ok(Self {})
    }
}

impl Drop for Citro2D {
    #[doc(alias = "C2D_Fini")]
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
        *ACTIVE_CITRO
            .lock()
            .expect("[Citro2D] Failed to get current state of Citro.") = false;
    }
}

