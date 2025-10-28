use std::{ptr};
use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_AlignCenter, C2D_AlignJustified, C2D_AlignLeft, C2D_AlignRight, C2D_AtBaseline, C2D_Font_s, C2D_Text, C2D_TextBuf, C2D_TextBufClear, C2D_TextBufDelete, C2D_TextBufNew, C2D_TextBufResize, C2D_TextFontParse, C2D_TextGetDimensions, C2D_TextOptimize, C2D_WithColor, C2D_WordWrap};

use crate::font::Font;
use crate::Citro2DError as CErr;

#[derive(Debug)]
#[repr(u8)]
pub enum TextFlags {
    #[doc(alias = "C2D_AtBaseline")]
    Baseline = C2D_AtBaseline,
    #[doc(alias = "C2D_WithColor")]
    WithColor = C2D_WithColor,
    #[doc(alias = "C2D_AlignLeft")]
    AlignLeft = C2D_AlignLeft,
    #[doc(alias = "C2D_AlignRight")]
    AlignRight = C2D_AlignRight,
    #[doc(alias = "C2D_AlignCenter")]
    AlignCenter = C2D_AlignCenter,
    #[doc(alias = "C2D_AlignJustified")]
    AlignJustified = C2D_AlignJustified,
    #[doc(alias = "C2D_WordWrap")]
    Wrap = C2D_WordWrap
}

pub struct Text {
    pub(crate) inner: Box<C2D_Text>,
    scale: (f32, f32),
    flags: u32
}

fn handle_parse_result(c: *const ::libc::c_char) -> Result<(), CErr> {
    let result = match unsafe { c.as_ref() } {
        Some(n) => *n as char,
        None => return Err(CErr::TextParseError)
    }; 

    if result != '\0' && result != '\n' {
        return Err(CErr::TextBufferOverflow);
    }

    Ok(())
}

impl Text {
    #[doc(alias = "docs")]
    pub fn new(text: &str, font: Option<&Font>) -> Result<Self, CErr> {

        let inner = Box::<C2D_Text>::new_zeroed();

        let mut txt = Text { 
            inner: unsafe { inner.assume_init() },
            scale: (1.0, 1.0),
            flags: 0
        };

        txt.add_text(text, font)?;

        Ok(txt)
    }

    pub fn add_text(&mut self, text: &str, font: Option<&Font>) -> Result<(), CErr> {
        let cstr = CString::from_str(text)?;
        let buf = TextBuffer::new(1024);

        let fontptr: *mut C2D_Font_s = match font {
            None => ptr::null_mut(),
            Some(i) => i.inner
        };
        let result = unsafe { C2D_TextFontParse(&mut *self.inner, &mut *fontptr, buf.inner, cstr.as_ptr()) };
        let _ = handle_parse_result(result)?;
        Ok(())
    }

    pub fn dimensions(&self) -> (f32, f32) {
        let (mut width, mut height) = (0.0, 0.0);

        unsafe { C2D_TextGetDimensions(&*self.inner, self.scale.0, self.scale.1, &mut width, &mut height); };

        (width, height)
    }

    pub(crate) fn optimize(&self) {
        unsafe { C2D_TextOptimize(&*self.inner); };
    }
}

pub(crate) struct TextBuffer {
    pub(crate) inner: C2D_TextBuf
}

impl TextBuffer {
    #[doc(alias = "C2D_TextBufNew")]
    pub(crate) fn new(size: usize) -> Self {
        let inner = unsafe { C2D_TextBufNew(size) };
        TextBuffer { inner }
    }

    #[allow(dead_code)]
    #[doc(alias = "C2D_TextBufResize")]
    pub fn resize(&mut self, size: usize) {
        self.inner = unsafe { C2D_TextBufResize(&mut *self.inner, size) };
    }

    #[allow(dead_code)]
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
