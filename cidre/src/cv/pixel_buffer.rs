use crate::{arc, cf, cv, define_options, os};

#[cfg(feature = "io")]
use crate::io;

pub type PixelBuf = cv::ImageBuf;

impl PixelBuf {
    #[inline]
    pub fn type_id() -> cf::TypeId {
        unsafe { CVPixelBufferGetTypeID() }
    }

    /// Width in pixels.
    #[doc(alias = "CVPixelBufferGetWidth")]
    #[inline]
    pub fn width(&self) -> usize {
        unsafe { CVPixelBufferGetWidth(self) }
    }

    /// Height in pixels.
    #[doc(alias = "CVPixelBufferGetHeight")]
    #[inline]
    pub fn height(&self) -> usize {
        unsafe { CVPixelBufferGetHeight(self) }
    }

    /// Returns the PixelFormat of the PixelBuf.
    #[doc(alias = "CVPixelBufferGetPixelFormatType")]
    #[inline]
    pub fn pixel_format(&self) -> PixelFormat {
        unsafe { CVPixelBufferGetPixelFormatType(self) }
    }

    #[doc(alias = "CVPixelBufferGetPlaneCount")]
    #[inline]
    pub fn plane_count(&self) -> usize {
        unsafe { CVPixelBufferGetPlaneCount(self) }
    }

    #[doc(alias = "CVPixelBufferGetWidthOfPlane")]
    #[inline]
    pub fn plane_width(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetWidthOfPlane(self, plane_index) }
    }

    #[doc(alias = "CVPixelBufferGetHeightOfPlane")]
    #[inline]
    pub fn plane_height(&self, plane_index: usize) -> usize {
        unsafe { CVPixelBufferGetHeightOfPlane(self, plane_index) }
    }

    /// ```
    /// use cidre::{cv, cg};
    ///
    /// let pixel_buffer = cv::PixelBuf::new(200, 100, cv::PixelFormat::_32_BGRA, None).unwrap();
    ///
    /// assert_eq!(200, pixel_buffer.width());
    /// assert_eq!(100, pixel_buffer.height());
    /// assert_eq!(cv::PixelFormat::_32_BGRA, pixel_buffer.pixel_format());
    /// assert_eq!(0, pixel_buffer.plane_count());
    /// assert_eq!(cv::PixelBuf::type_id(), pixel_buffer.get_type_id());
    ///
    /// ```
    pub fn new(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormat,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<arc::R<PixelBuf>, cv::Return> {
        let mut pixel_buffer_out = None;

        let r = Self::create_in(
            width,
            height,
            pixel_format_type,
            pixel_buffer_attributes,
            &mut pixel_buffer_out,
            None,
        );

        unsafe { r.to_result_unchecked(pixel_buffer_out) }
    }

    pub fn create_in(
        width: usize,
        height: usize,
        pixel_format_type: cv::PixelFormat,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<arc::R<PixelBuf>>,
        allocator: Option<&cf::Allocator>,
    ) -> cv::Return {
        unsafe {
            CVPixelBufferCreate(
                allocator,
                width,
                height,
                pixel_format_type,
                pixel_buffer_attributes,
                pixel_buffer_out,
            )
        }
    }

    #[doc(alias = "CVPixelBufferLockBaseAddress")]
    #[inline]
    pub unsafe fn lock_base_address(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferLockBaseAddress(self, flags)
    }

    #[doc(alias = "CVPixelBufferUnlockBaseAddress")]
    #[inline]
    pub unsafe fn unlock_lock_base_address(&self, flags: LockFlags) -> cv::Return {
        CVPixelBufferUnlockBaseAddress(self, flags)
    }

    #[inline]
    pub fn base_address_lock(&self, flags: LockFlags) -> Result<BaseAddressLockGuard, cv::Return> {
        unsafe {
            let res = self.lock_base_address(flags);
            if res.is_ok() {
                Ok(BaseAddressLockGuard(self, flags))
            } else {
                Err(res)
            }
        }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn get_io_surface(&self) -> Option<&io::Surface> {
        unsafe { CVPixelBufferGetIOSurface(self) }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn get_io_surface_mut(&mut self) -> Option<&mut io::Surface> {
        unsafe { std::mem::transmute(CVPixelBufferGetIOSurface(self)) }
    }

    #[cfg(feature = "io")]
    #[inline]
    pub fn with_io_surface(
        surface: &io::Surface,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
    ) -> Result<arc::R<Self>, cv::Return> {
        Self::with_io_surface_in(surface, pixel_buffer_attributes, None)
    }

    /// Call to create a single cv::PixelBuffer for a passed-in IOSurface.
    #[cfg(feature = "io")]
    #[inline]
    pub fn with_io_surface_in(
        surface: &io::Surface,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        allocator: Option<&cf::Allocator>,
    ) -> Result<arc::R<Self>, cv::Return> {
        let mut buffer = None;
        unsafe {
            let res = CVPixelBufferCreateWithIOSurface(
                allocator,
                surface,
                pixel_buffer_attributes,
                &mut buffer,
            );
            if res.is_ok() {
                Ok(buffer.unwrap_unchecked())
            } else {
                Err(res)
            }
        }
    }
}

pub struct BaseAddressLockGuard<'a>(&'a PixelBuf, LockFlags);

impl<'a> Drop for BaseAddressLockGuard<'a> {
    #[inline]
    fn drop(&mut self) {
        let res = unsafe { self.0.unlock_lock_base_address(self.1) };
        debug_assert!(res.is_ok());
    }
}

define_options!(LockFlags(cv::OptionFlags));

impl LockFlags {
    pub const DEFAULT: Self = Self(0);
    pub const READ_ONLY: Self = Self(1);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PixelFormat(pub os::Type);

// https://developer.apple.com/documentation/technotes/tn3121-selecting-a-pixel-format-for-an-avcapturevideodataoutput

impl PixelFormat {
    /// 1 bit indexed
    #[doc(alias = "kCVPixelFormatType_1Monochrome")]
    pub const _1_MONOCHROME: Self = Self(0x00000001);

    /// 2 bit indexed
    #[doc(alias = "kCVPixelFormatType_2Indexed")]
    pub const _2_INDEXED: Self = Self(0x00000002);

    /// 4 bit indexed
    #[doc(alias = "kCVPixelFormatType_4Indexed")]
    pub const _4_INDEXED: Self = Self(0x00000004);

    /// 8 bit indexed
    #[doc(alias = "kCVPixelFormatType_8Indexed")]
    pub const _8_INDEXED: Self = Self(0x00000008);

    /// 1 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_1IndexedGray_WhiteIsZero")]
    pub const _1_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000021);

    /// 2 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_2IndexedGray_WhiteIsZero")]
    pub const _2_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000022);

    /// 4 bit indexed gray, white is zero
    #[doc(alias = "kCVPixelFormatType_4IndexedGray_WhiteIsZero")]
    pub const _4_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000024);

    /// 8 bit indexed gray, white is zero
    #[doc(alias = "kCMPixelFormat_8IndexedGray_WhiteIsZero")]
    #[doc(alias = "kCVPixelFormatType_8IndexedGray_WhiteIsZero")]
    pub const _8_INDEXED_GREY_WHITE_IS_ZERO: Self = Self(0x000000028);

    #[doc(alias = "kCMPixelFormat_32BGRA")]
    #[doc(alias = "kCVPixelFormatType_32BGRA")]
    pub const _32_BGRA: Self = Self(os::Type::from_be_bytes(*b"BGRA"));

    #[doc(alias = "kCVPixelFormatType_Lossless_32BGRA")]
    pub const LOSSLESS_32_BGRA: Self = Self(os::Type::from_be_bytes(*b"&GRA"));

    #[doc(alias = "kCVPixelFormatType_Lossy_32BGRA")]
    pub const LOSSY_32_BGRA: Self = Self(os::Type::from_be_bytes(*b"-GRA"));

    pub const _420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self = Self(os::Type::from_be_bytes(*b"420v"));
    pub const _420V: Self = Self::_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE;
    // TODO: how we can optimize that agly long consts?
    pub const _420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self = Self(os::Type::from_be_bytes(*b"420f"));
    pub const _420F: Self = Self::_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;
    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8v0"));

    pub const LOSSY_420V: Self = Self::LOSSY_420_YP_CB_CR_8_BI_PLANAR_VIDEO_RANGE;

    pub const LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-8f0"));

    pub const LOSSY_420F: Self = Self::LOSSY_420_YP_CB_CR_8_BI_PLANAR_FULL_RANGE;

    pub const _420_YP_CB_CR_10_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"x420"));
    pub const LOSSY_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv0"));

    pub const LOSSY_PACKED_10_420V: Self = Self::LOSSY_420_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE;
    pub const LOSSY_422_YP_CB_CR_10_PACKED_BI_PLANAR_VIDEO_RANGE: Self =
        Self(os::Type::from_be_bytes(*b"-xv2"));

    pub const ARGB_2101010_LE_PACKED: Self = Self(os::Type::from_be_bytes(*b"l10r")); /* little-endian ARGB2101010 full-range ARGB */

    #[doc(alias = "kCVPixelFormatType_OneComponent8")]
    pub const ONE_COMPONENT_8: Self = Self(os::Type::from_be_bytes(*b"L008"));

    #[doc(alias = "kCVPixelFormatType_OneComponent16Half")]
    pub const ONE_COMPONENT_16_HALF: Self = Self(os::Type::from_be_bytes(*b"L00h"));

    #[doc(alias = "kCVPixelFormatType_OneComponent32Float")]
    pub const ONE_COMPONENT_32_FLOAT: Self = Self(os::Type::from_be_bytes(*b"L00f"));

    pub fn from_cf_number(number: &cf::Number) -> Self {
        Self(number.to_i32().unwrap_or(0) as u32)
    }

    pub fn to_description(&self) -> Option<arc::R<cf::Dictionary>> {
        cv::pixel_format_description_create(*self)
    }

    #[inline]
    pub fn to_cf_number(&self) -> arc::R<cf::Number> {
        cf::Number::from_i32(self.0 as _)
    }

    #[doc(alias = "CVIsCompressedPixelFormatAvailable")]
    #[inline]
    pub fn is_compressed_format_avaliable(&self) -> bool {
        unsafe { CVIsCompressedPixelFormatAvailable(*self) }
    }
}

extern "C" {
    fn CVPixelBufferGetTypeID() -> cf::TypeId;
    fn CVPixelBufferCreate(
        allocator: Option<&cf::Allocator>,
        width: usize,
        height: usize,
        pixel_format_type: PixelFormat,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: &mut Option<arc::R<PixelBuf>>,
    ) -> cv::Return;

    fn CVPixelBufferGetWidth(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetHeight(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetPixelFormatType(pixel_buffer: &PixelBuf) -> PixelFormat;
    fn CVPixelBufferGetPlaneCount(pixel_buffer: &PixelBuf) -> usize;
    fn CVPixelBufferGetWidthOfPlane(pixel_buffer: &PixelBuf, plane_index: usize) -> usize;
    fn CVPixelBufferGetHeightOfPlane(pixel_buffer: &PixelBuf, plane_index: usize) -> usize;

    fn CVPixelBufferLockBaseAddress(pixel_buffer: &PixelBuf, lock_flags: LockFlags) -> cv::Return;
    fn CVPixelBufferUnlockBaseAddress(pixel_buffer: &PixelBuf, lock_flags: LockFlags)
        -> cv::Return;

    #[cfg(feature = "io")]
    fn CVPixelBufferGetIOSurface(pixel_buffer: &PixelBuf) -> Option<&io::Surface>;

    #[cfg(feature = "io")]
    fn CVPixelBufferCreateWithIOSurface(
        allocator: Option<&cf::Allocator>,
        surface: &io::Surface,
        pixel_buffer_attributes: Option<&cf::Dictionary>,
        pixel_buffer_out: *mut Option<arc::R<cv::PixelBuf>>,
    ) -> cv::Return;

    fn CVIsCompressedPixelFormatAvailable(pixel_format: PixelFormat) -> bool;
}

pub mod keys {
    use crate::cf;

    /// A single cf::Number or a cf::Array of cf::Numbers (os::Types)
    #[doc(alias = "kCVPixelBufferPixelFormatTypeKey")]
    #[inline]
    pub fn pixel_format_type() -> &'static cf::String {
        unsafe { kCVPixelBufferPixelFormatTypeKey }
    }

    #[doc(alias = "kCVPixelBufferWidthKey")]
    #[inline]
    pub fn width() -> &'static cf::String {
        unsafe { kCVPixelBufferWidthKey }
    }

    #[doc(alias = "kCVPixelBufferHeightKey")]
    #[inline]
    pub fn height() -> &'static cf::String {
        unsafe { kCVPixelBufferHeightKey }
    }

    #[doc(alias = "kCVPixelBufferIOSurfacePropertiesKey")]
    #[inline]
    pub fn io_surface_properties() -> &'static cf::String {
        unsafe { kCVPixelBufferIOSurfacePropertiesKey }
    }

    #[doc(alias = "kCVPixelBufferMetalCompatibilityKey")]
    #[inline]
    pub fn metal_compatability() -> &'static cf::String {
        unsafe { kCVPixelBufferMetalCompatibilityKey }
    }

    #[doc(alias = "kCVPixelBufferPlaneAlignmentKey")]
    #[inline]
    pub fn plane_aligment() -> &'static cf::String {
        unsafe { kCVPixelBufferPlaneAlignmentKey }
    }

    #[doc(alias = "kCVPixelBufferExtendedPixelsBottomKey")]
    #[inline]
    pub fn extended_pixels_bottom() -> &'static cf::String {
        unsafe { kCVPixelBufferExtendedPixelsBottomKey }
    }

    #[doc(alias = "kCVPixelBufferCGImageCompatibilityKey")]
    #[inline]
    pub fn cg_image_comaptibility() -> &'static cf::String {
        unsafe { kCVPixelBufferCGImageCompatibilityKey }
    }

    extern "C" {
        static kCVPixelBufferPixelFormatTypeKey: &'static cf::String;
        static kCVPixelBufferWidthKey: &'static cf::String;
        static kCVPixelBufferHeightKey: &'static cf::String;
        static kCVPixelBufferIOSurfacePropertiesKey: &'static cf::String;
        static kCVPixelBufferMetalCompatibilityKey: &'static cf::String;
        static kCVPixelBufferPlaneAlignmentKey: &'static cf::String;
        static kCVPixelBufferExtendedPixelsBottomKey: &'static cf::String;
        static kCVPixelBufferCGImageCompatibilityKey: &'static cf::String;
    }
}

#[cfg(test)]
mod tests {
    use crate::cv::PixelFormat;

    #[test]
    fn compressed() {
        assert!(PixelFormat::LOSSY_420V.is_compressed_format_avaliable());
        assert!(PixelFormat::LOSSY_420F.is_compressed_format_avaliable());
        assert!(PixelFormat::LOSSY_PACKED_10_420V.is_compressed_format_avaliable());
    }
}
