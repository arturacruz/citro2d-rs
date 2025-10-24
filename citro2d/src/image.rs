use citro2d_sys::C2D_Image;

#[doc(alias = "C2D_Image")]
pub struct Image {
   img: C2D_Image
}

impl Image {
    pub fn from(img: C2D_Image) -> Self {
        Image { img }
    }

    pub unsafe fn inner(&self) -> C2D_Image {
        self.img
    }
}
