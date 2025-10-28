use std::ptr;
use std::sync::Mutex;
use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_Font_s, C2D_Text, C2D_TextBuf, C2D_TextBufClear, C2D_TextBufDelete, C2D_TextBufNew, C2D_TextBufResize, C2D_TextFontParse};

use crate::font::Font;
use crate::Citro2DError as CErr;

pub struct Text {
    pub(crate) inner: Box<C2D_Text>,

    flags: u32
}

impl Text {
    #[doc(alias = "docs")]
    pub fn new(text: &str, font: Option<Font>) -> Result<Self, CErr> {
        let cstr = CString::from_str(text)?;
        let buf = TextBuffer::new(1024);

        let mut inner = Box::<C2D_Text>::new_uninit();

        let fontptr: *mut C2D_Font_s = match font {
            None => ptr::null_mut(),
            Some(i) => i.inner
        };
        let result = unsafe { C2D_TextFontParse(inner.as_mut_ptr(), &mut *fontptr, buf.inner, cstr.as_ptr()) };

        let result = match unsafe { result.as_ref() } {
            Some(n) => *n as char,
            None => return Err(CErr::TextParseError)
        };

        if result != '\0' && result != '\n' {
            return Err(CErr::TextBufferOverflow);
        }

        Ok(Text { 
            inner: unsafe { inner.assume_init() },
            flags: 0
        })
    }
}

pub(crate) struct TextBuffer {
    pub(crate) inner: C2D_TextBuf
}

impl TextBuffer {
    #[doc(alias = "C2D_TextBufNew")]
    pub fn new(size: usize) -> Self {
        let inner = unsafe { C2D_TextBufNew(size) };
        TextBuffer { inner }
    }

    #[doc(alias = "C2D_TextBufResize")]
    pub fn resize(&mut self, size: usize) {
        self.inner = unsafe { C2D_TextBufResize(&mut *self.inner, size) };
    }

    #[doc(alias = "C2D_TextBufClear")]
    pub fn clear(&mut self) {
        unsafe { C2D_TextBufClear(&mut *self.inner); };
    }
}

impl Drop for TextBuffer {
    #[doc(alias = "C2D_TextBufDelete")]
    fn drop(&mut self) {
        unsafe {
            C2D_TextBufDelete(self.inner);
        }
    }
}
