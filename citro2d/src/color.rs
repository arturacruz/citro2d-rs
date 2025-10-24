use citro2d_sys::{C2D_Color32, C2D_Color32f};

pub struct Color<T> {
    pub r: T,
    pub g: T,
    pub b: T,
    pub a: T
}

impl Into<u32> for Color<u8> {
    fn into(self) -> u32 {
        unsafe { C2D_Color32(self.r, self.g, self.b, self.a) }
    }
}

impl Into<u32> for Color<f32> {
    fn into(self) -> u32 {
        unsafe { C2D_Color32f(self.r, self.g, self.b, self.a) }
    }
}

impl Color<u8> {
    pub const RED: Self = Self::new(255, 0, 0, 255);
    pub const BLUE: Self = Self::new(0, 0, 255, 255);
    pub const GREEN: Self = Self::new(0, 255, 0, 255);
    pub const BLACK: Self = Self::new(0, 0, 0, 255);
    pub const WHITE: Self = Self::new(255, 255, 255, 255);
    pub const TRANSPARENT: Self = Self::new(0, 0, 0, 0);

    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color { r, g, b, a }
    }
}

impl Color<f32> {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color { r, g, b, a }
    }
}
