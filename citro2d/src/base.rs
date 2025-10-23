use citro2d_sys::{C2D_Fini, C2D_Prepare, C3D_Fini, C3D_Init, C2D_Init, C2D_DEFAULT_MAX_OBJECTS, C3D_DEFAULT_CMDBUF_SIZE};

pub enum Citro2DError {
    Init3D, Init2D, AlreadyInitialized
}

pub struct Citro2D;

impl Citro2D {
    pub fn new() -> Result<Self, Citro2DError> {
        Self::init(C3D_DEFAULT_CMDBUF_SIZE, C2D_DEFAULT_MAX_OBJECTS)
    }

    pub fn from(buffer_size: u32, max_objs: u16) -> Result<Self, Citro2DError> {
        Self::init(buffer_size, max_objs)
    }

    fn init(buffer_size: u32, max_objs: u16) -> Result<Self, Citro2DError> {
        if !unsafe { C3D_Init(buffer_size as usize) } { 
            return Err(Citro2DError::Init3D);
        }
        if !unsafe { C2D_Init(max_objs as usize) } {
            return Err(Citro2DError::Init2D);
        }
        unsafe {
            C2D_Prepare();
        }

        Ok(Self)
    }
}

impl Drop for Citro2D {
    fn drop(&mut self) {
        unsafe {
            C2D_Fini();
            C3D_Fini();
        }
    }
}
