use std::{error::Error, ffi::NulError, fmt::Display};

pub mod font;
pub mod text;
pub mod color;
pub mod frame;
pub mod image;
pub mod base;
pub mod render;
pub mod sprite;
pub mod spritesheet;

#[derive(Debug)]
pub enum Citro2DError {
    FileNotFound,
    InvalidString,
    TextBufferOverflow,
    TextParseError
}

impl From<NulError> for Citro2DError {
    fn from(_: NulError) -> Self {
        Citro2DError::InvalidString
    }
}

impl Display for Citro2DError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "erro")
    }
}

impl Error for Citro2DError {}
