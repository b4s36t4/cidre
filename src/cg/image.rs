use crate::{cf, cg, define_cf_type};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum AlphaInfo {
    /// For example, RGB
    None,
    /// For example, premultiplied RGBA
    PremultipliedLast,
    /// For example, premultiplied ARGB
    PremultipliedFirst,
    /// For example, non-premultiplied RGBA
    Last,
    /// For example, non-premultiplied ARGB
    First,
    /// For example, RBGX
    NoneSkipLast,
    /// For example, XRGB
    NoneSkipFirst,
    Only,
}

define_cf_type!(Image(cf::Type));

impl Image {
    /// Return true if `image' is an image mask, false otherwise.`
    #[inline]
    pub fn is_mask(&self) -> bool {
        unsafe { CGImageIsMask(self) }
    }

    #[inline]
    pub fn width(&self) -> usize {
        unsafe { CGImageGetWidth(self) }
    }

    #[inline]
    pub fn height(&self) -> usize {
        unsafe { CGImageGetHeight(self) }
    }

    #[inline]
    pub fn alpha_info(&self) -> AlphaInfo {
        unsafe { CGImageGetAlphaInfo(self) }
    }

    #[inline]
    pub fn ut_type(&self) -> Option<&cf::String> {
        unsafe { CGImageGetUTType(self) }
    }

    #[inline]
    pub fn color_space(&self) -> Option<&cg::ColorSpace> {
        unsafe { CGImageGetColorSpace(self) }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    fn CGImageIsMask(image: &Image) -> bool;
    fn CGImageGetWidth(image: &Image) -> usize;
    fn CGImageGetHeight(image: &Image) -> usize;
    fn CGImageGetAlphaInfo(image: &Image) -> AlphaInfo;
    fn CGImageGetUTType(image: &Image) -> Option<&cf::String>;
    fn CGImageGetColorSpace(image: &Image) -> Option<&cg::ColorSpace>;
}
