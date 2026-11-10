use std::{ptr};
use std::{ffi::CString, str::FromStr};

use citro2d_sys::{C2D_AlignCenter, C2D_AlignJustified, C2D_AlignLeft, C2D_AlignRight, C2D_AtBaseline, C2D_Text, C2D_TextBuf, C2D_TextBufClear, C2D_TextBufDelete, C2D_TextBufNew, C2D_TextBufResize, C2D_TextFontParse, C2D_TextGetDimensions, C2D_TextOptimize, C2D_WithColor, C2D_WordWrap};

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
    pub(crate) scale: (f32, f32),
    pub(crate) pos: (f32, f32, f32),
    pub(crate) flags: u32,
    buffer: TextBuffer
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
        let buffer = TextBuffer::new(
            text.chars().count().max(1)
        );

        let mut txt = Text { 
            inner: unsafe { inner.assume_init() },
            scale: (1.0, 1.0),
            pos: (0.0, 0.0, 0.0),
            flags: 0,
            buffer
        };

        txt.set_text(text, font)?;

        Ok(txt)
    }

    pub fn set_text(&mut self, text: &str, font: Option<&Font>) -> Result<(), CErr> {
        self.buffer.clear();
        let cstr = CString::from_str(text)?;

        let fontptr = font.map_or(ptr::null_mut(), |f| f.inner); 
        let result = unsafe { 
            C2D_TextFontParse(
                &mut *self.inner, 
                fontptr,
                self.buffer.inner,
                cstr.as_ptr()
            ) 
        };

        handle_parse_result(result)
    }

    pub fn dimensions(&self) -> (f32, f32) {
        let (mut width, mut height) = (0.0, 0.0);

        unsafe { C2D_TextGetDimensions(&*self.inner, self.scale.0, self.scale.1, &mut width, &mut height); };

        (width, height)
    }

    pub fn set_position(&mut self, x: f32, y: f32, z: f32) {
        self.pos = (x, y, z);
    }

    pub fn set_scale(&mut self, x: f32, y: f32) {
        self.scale = (x, y);
    }

    pub fn set_flags(&mut self, flags: Vec<TextFlags>) {
        self.flags = 0;
        for flag in flags {
            self.flags |= flag as u32;
        }
    }

    pub fn optimize(&self) {
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
