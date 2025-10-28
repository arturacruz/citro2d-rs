use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_Font, C2D_FontFree, C2D_FontLoad};

use crate::Citro2DError as CErr;

pub struct Font {
    pub(super) inner: C2D_Font
}

impl Font {
    #[doc(alias = "C2D_FontLoad")]
    pub fn new(filename: &str) -> Result<Self, CErr> {
        let path = CString::from_str(filename)?;
        let inner = unsafe { C2D_FontLoad(path.as_ptr()) };

        if inner.is_null() {
            return Err(CErr::FileNotFound);
        }

        Ok(Self { inner })
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        unsafe {
            C2D_FontFree(self.inner);
        }
    }
}
