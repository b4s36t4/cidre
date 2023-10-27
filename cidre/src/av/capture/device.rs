use std::ops::{Deref, DerefMut};

#[cfg(any(target_os = "tvos", target_os = "ios"))]
use std::ffi::c_void;

#[cfg(feature = "blocks")]
#[cfg(any(target_os = "tvos", target_os = "ios"))]
use crate::blocks;

use crate::{
    arc,
    av::{self, MediaType},
    cf, cg, cm, define_cls, define_obj_type, ns, objc,
};

define_obj_type!(Type(ns::String));

/// ```
/// use cidre::av;
///
/// let device_type = av::CaptureDeviceType::external();
/// let device_type = av::CaptureDeviceType::built_in_microphone();
/// let device_type = av::CaptureDeviceType::built_in_wide_angle_camera();
/// let device_type = av::CaptureDeviceType::built_in_telephoto_camera();
/// let device_type = av::CaptureDeviceType::built_in_ultra_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_camera();
/// let device_type = av::CaptureDeviceType::built_in_dual_wide_camera();
/// let device_type = av::CaptureDeviceType::built_in_tripple_camera();
/// let device_type = av::CaptureDeviceType::built_in_true_depth_camera();
/// let device_type = av::CaptureDeviceType::built_in_lidar_depth_camera();
/// ```
impl Type {
    /// An external device type. On iPad, external devices are those that conform
    /// to the UVC (USB Video Class) specification.
    ///
    /// Starting in Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::external()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    ///  <key>NSCameraUseExternalDeviceType</key>
    ///  <true/>
    /// ```
    /// Otherwise, external cameras on Mac Catalyst report that their device type is
    /// 'av::CaptureDeviceType::built_in_wide_angle_camera()'.
    #[doc(alias = "AVCaptureDeviceTypeExternal")]
    pub fn external() -> &'static Self {
        unsafe { AVCaptureDeviceTypeExternal }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInMicrophone")]
    pub fn built_in_microphone() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInMicrophone }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInWideAngleCamera")]
    pub fn built_in_wide_angle_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInWideAngleCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTelephotoCamera")]
    pub fn built_in_telephoto_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTelephotoCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInUltraWideCamera")]
    pub fn built_in_ultra_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInUltraWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualCamera")]
    pub fn built_in_dual_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInDualWideCamera")]
    pub fn built_in_dual_wide_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInDualWideCamera }
    }

    #[doc(alias = "AVCaptureDeviceTypeBuiltInTripleCamera")]
    pub fn built_in_tripple_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTripleCamera }
    }

    /// A device that consists of two cameras, one YUV and one Infrared.
    /// The infrared camera provides high quality depth information that is synchronized
    /// and perspective corrected to frames produced by the YUV camera. While the resolution
    /// of the depth data and YUV frames may differ, their field of view and aspect ratio
    /// always match. Note that devices of this type may only be discovered using an
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInTrueDepthCamera")]
    pub fn built_in_true_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInTrueDepthCamera }
    }
    /// A device that consists of two cameras, one YUV and one LiDAR.
    /// The LiDAR camera provides high quality, high accuracy depth information by measuring
    /// the round trip of an artificial light signal emitted by a laser. The depth
    /// is synchronized and perspective corrected to frames produced by the paired YUV camera.
    /// While the resolution of the depth data and YUV frames may differ, their field of view
    /// and aspect ratio always match. Note that devices of this type may only be discovered
    /// using an av::CaptureDeviceDiscoverySession or
    /// `av::CaptureDevice::default_device_with_device_type_media_type_position`.
    #[doc(alias = "AVCaptureDeviceTypeBuiltInLiDARDepthCamera")]
    pub fn built_in_lidar_depth_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeBuiltInLiDARDepthCamera }
    }

    /// A continuity camera device. These devices are suitable for general purpose use.
    /// Note that devices of this type may only be discovered using an
    /// av::CaptureDeviceDiscoverySession or
    ///`av::CaptureDevice::default_device_with_device_type_media_type_position`.
    ///
    /// Starting in macOS 14.0 and Mac Catalyst 17.0, apps may opt in for using
    /// 'av::CaptureDeviceType::continuity_camera()' by adding the following key
    /// to their Info.plist:
    /// ```xml
    /// <key>NSCameraUseContinuityCameraDeviceType</key>
    /// <true/>
    /// ```
    #[doc(alias = "AVCaptureDeviceTypeContinuityCamera")]
    pub fn continuity_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeContinuityCamera }
    }

    /// A distortion corrected cut out from an ultra wide camera, made to approximate
    /// an overhead camera pointing at a desk.
    /// Supports multicam operation.
    #[doc(alias = "AVCaptureDeviceTypeDeskViewCamera")]
    #[cfg(target_os = "macos")]
    pub fn desk_view_camera() -> &'static Self {
        unsafe { AVCaptureDeviceTypeDeskViewCamera }
    }
    // #[cfg(target_os = "macos")]
    // #[doc(alias = "AVCaptureDeviceTypeExternalUnknown")]
    // pub fn external_unknown() -> &'static Self {
    //     unsafe { AVCaptureDeviceTypeExternalUnknown }
    // }
}

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {
    static AVCaptureDeviceTypeExternal: &'static Type;
    static AVCaptureDeviceTypeBuiltInMicrophone: &'static Type;
    static AVCaptureDeviceTypeBuiltInWideAngleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTelephotoCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInUltraWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInDualWideCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTripleCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInTrueDepthCamera: &'static Type;
    static AVCaptureDeviceTypeBuiltInLiDARDepthCamera: &'static Type;
    static AVCaptureDeviceTypeContinuityCamera: &'static Type;
    #[cfg(target_os = "macos")]
    static AVCaptureDeviceTypeDeskViewCamera: &'static Type;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureLensPositionCurrent: f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    static AVCaptureISOCurrent: f32;
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
#[repr(isize)]
pub enum Position {
    Unspecified = 0,
    Back = 1,
    Front = 2,
}

define_obj_type!(Device(ns::Id));

impl Device {
    define_cls!(AV_CAPTURE_DEVICE);

    #[objc::cls_msg_send(defaultDeviceWithDeviceType:mediaType:position:)]
    pub fn with_device_type_media_and_position_ar(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_device_type_media_and_position(
        device_type: &Type,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> Option<arc::R<Self>>;

    #[objc::cls_msg_send(deviceWithUniqueID:)]
    pub fn with_unique_id_ar(unique_id: ns::String) -> Option<arc::Rar<Self>>;

    #[objc::cls_rar_retain]
    pub fn with_unique_id(unique_id: ns::String) -> Option<arc::R<Self>>;

    #[objc::msg_send(uniqueID)]
    pub fn unique_id(&self) -> &ns::String;

    /// The model ID of the receiver.
    ///
    /// The value of this property is an identifier unique to all devices of the same model.
    /// The value is persistent across device connections and disconnections,
    /// and across different systems. For example, the model ID of the camera built in
    /// to two identical iPhone models will be the same even though they are different
    /// physical devices.
    #[objc::msg_send(modelID)]
    pub fn model_id(&self) -> &ns::String;

    /// A localized human-readable name for the receiver.
    ///
    /// This property can be used for displaying the name of a capture device in a user interface.
    #[objc::msg_send(localizedName)]
    pub fn localized_name(&self) -> &ns::String;

    /// The human-readable manufacturer name for the receiver.
    ///
    /// This property can be used to identify capture devices from a particular manufacturer.
    /// All Apple devices return "Apple Inc.". Devices from third party manufacturers may
    /// return an empty string.
    #[objc::msg_send(manufacturer)]
    pub fn manufacturer(&self) -> &ns::String;

    /// Returns whether the receiver provides media with the given media type.
    ///
    /// 'true' if the device outputs the given media type, 'false' otherwise.
    #[objc::msg_send(hasMediaType:)]
    pub fn has_media_type(&self, media_type: &av::MediaType) -> bool;

    #[objc::msg_send(formats)]
    pub fn formats(&self) -> &ns::Array<Format>;

    #[objc::msg_send(supportsAVCaptureSessionPreset:)]
    pub fn supports_preset(&self, preset: &av::CaptureSessionPreset) -> bool;

    #[objc::msg_send(activeFormat)]
    pub fn active_format(&self) -> &Format;

    /// Indicates whether the device is connected and available to the system.
    ///
    /// The value of this property is a 'bool' indicating whether the device represented
    /// by the receiver is connected and available for use as a capture device.
    /// Clients can key value observe the value of this property to be notified when a device
    /// is no longer available. When the value of this property becomes 'false' for a given
    /// instance, it will not become 'true' again. If the same physical device again becomes
    /// available to the system, it will be represented using a new instance of 'av::CaptureDevice'.
    #[objc::msg_send(isConnected)]
    pub fn is_connected(&self) -> bool;

    #[objc::msg_send(activeVideoMinFrameDuration)]
    pub fn active_video_min_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(activeVideoMaxFrameDuration)]
    pub fn active_video_max_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(hasTorch)]
    pub fn has_torch(&self) -> bool;

    #[objc::msg_send(isRampingVideoZoom)]
    pub fn is_ramping_video_zoom(&self) -> bool;

    pub fn configuration_lock(&mut self) -> Result<ConfigurationLockGuard, arc::R<cf::Error>> {
        let mut error = None;
        unsafe {
            let result = self.lock_for_configuration(&mut error);
            if let Some(error) = error.take() {
                return Err(error);
            }

            debug_assert!(result);

            Ok(ConfigurationLockGuard { device: self })
        }
    }

    #[objc::msg_send(lockForConfiguration:)]
    pub unsafe fn lock_for_configuration(&mut self, error: &mut Option<arc::R<cf::Error>>) -> bool;

    #[objc::msg_send(unlockForConfiguration)]
    pub unsafe fn unlock_for_configuration(&mut self);

    #[objc::msg_send(setActiveFormat:)]
    pub unsafe fn set_active_format(&mut self, value: &Format);

    #[objc::msg_send(setActiveVideoMinFrameDuration:)]
    pub unsafe fn set_active_video_min_frame_duration(&mut self, value: cm::Time);

    #[objc::msg_send(setActiveVideoMaxFrameDuration:)]
    pub unsafe fn set_active_video_max_frame_duration(&mut self, value: cm::Time);

    #[objc::msg_send(rampToVideoZoomFactor:withRate:)]
    pub unsafe fn ramp_to_video_zoom_factor_throws(&mut self, factor: cg::Float, rate: f32);

    #[objc::msg_send(cancelVideoZoomRamp)]
    pub unsafe fn cancel_video_zoom_ramp_throws(&mut self);
}

/// AVCaptureDeviceReactionEffects
impl Device {
    #[objc::cls_msg_send(reactionEffectsEnabled)]
    pub fn reaction_effects_enabled() -> bool;

    #[objc::cls_msg_send(reactionEffectGesturesEnabled)]
    pub fn reaction_effect_gestures_enabled() -> bool;

    #[objc::msg_send(canPerformReactionEffects)]
    pub fn can_perform_reaction_effects(&self) -> bool;

    /// Returns a list of reaction types which can be passed to perform_effect_for_reaction.
    ///
    /// The list may differ between devices, or be affected by changes to active format,
    /// and can be key-value observed.
    #[objc::msg_send(availableReactionTypes)]
    pub fn available_reaction_types(&self) -> &ns::Set<av::CaptureReactionType>;

    /// Triggers a specified reaction on the video stream.
    #[objc::msg_send(performEffectForReaction:)]
    pub fn perform_effect_for_reaction(&mut self, reaction_type: &av::CaptureReactionType);

    #[objc::msg_send(reactionEffectsInProgress)]
    pub fn reaction_effects_in_progress(&self) -> &ns::Array<av::CaptureReactionEffectState>;
}

/// AVCaptureDeviceContinuityCamera
impl Device {
    /// A property that reports YES if the receiver is a Continuity Camera.
    ///
    /// Access this property to discover if the receiver is
    /// a Continuity Camera (external iPhone webcam).
    #[objc::msg_send(isContinuityCamera)]
    pub fn is_continuity_camera(&self) -> bool;
}

#[doc(alias = "AVCaptureMicrophoneMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum MicMode {
    Standard = 0,
    WideSpectrum = 1,
    VoiceIsolation = 2,
}

/// AVCaptureMicrophoneMode
impl Device {
    /// Indicates the microphone mode that has been selected by the user in Control Center.
    ///
    /// This readonly property returns the microphone mode selected by the user in Control Center. It is key-value observable.
    #[objc::cls_msg_send(preferredMicrophoneMode)]
    pub fn preferred_mic_mode() -> MicMode;

    /// Indicates the currently active microphone mode.
    ///
    /// This readonly property returns the currently active microphone mode,
    /// which may differ from the 'preferred_mic_mode()' if the application's
    /// active audio route does not support the preferred microphone mode.
    /// This property is key-value observable.
    #[objc::cls_msg_send(activeMicrophoneMode)]
    pub fn active_mic_mode() -> MicMode;
}

#[doc(alias = "AVCaptureCenterStageControlMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum CenterStageControlMode {
    /// Indicates that the application is unaware of the Center Stage feature.
    /// Its enablement is entirely under user control in Control Center.
    User = 0,

    /// Indicates that the application controls the Center Stage feature,
    /// disallowing input from the user in Control Center.
    App = 1,

    /// Indicates that both the user and application cooperatively share
    /// control of the Center Stage feature.
    Cooperative = 2,
}

impl Device {
    /// Current mode of Center Stage control (user, app, or cooperative).
    ///
    /// This class property determines how the Center Stage feature is controlled.
    /// When set to the default value of 'av::CaptureCenterStageControlMode::User',
    /// 'set_center_stage_enabled' may not be set programmatically and throws an
    /// NSInvalidArgumentException. In User mode, the feature may only be set by
    /// the user in Control Center. If you wish to take Center Stage control away
    /// from the user and exclusively enable / disable it programmatically, set this property
    /// to 'av::CaptureCenterStageControlMode::App'. When under exclusive app control,
    /// Center Stage user control is disallowed (for instance, the toggle is grayed out
    /// in Control Center). If you wish to take control of Center Stage, but also cooperate
    /// with the user by listening for and appropriately reacting to their changes to the
    /// 'center_stage_enabled' property, set this property to
    /// 'av::CaptureCenterStageControlMode::Cooperative'. Note that in this mode,
    /// the onus is on you, the app developer, to honor user intent and conform your
    /// 'av::CaptureSession' configuration to make Center Stage active (see the 'av::CaptureDevice'
    /// instance property 'center_stage_active'). In cooperative mode, the 'center_stage_enabled'
    /// property may change at any time (such as when the user enables / disables the feature
    /// in Control Center).
    #[objc::cls_msg_send(centerStageControlMode)]
    pub fn center_stage_control_mode() -> CenterStageControlMode;

    #[objc::cls_msg_send(setCenterStageControlMode:)]
    pub fn set_center_stage_control_mode(value: CenterStageControlMode);

    /// Indicates whether Center Stage is currently active on a particular av::CaptureDevice.
    ///
    /// This readonly property returns 'true' when Center Stage is currently active on
    /// the receiver. When active, the camera automatically adjusts to keep people optimally
    /// framed within the field of view. The field of view may pan, tighten or widen as needed.
    /// Certain restrictions come into play when Center Stage is active:
    ///
    /// - The device's 'min_available_video_zoom_factor' and 'max_available_video_zoom_factor'
    ///   become restricted (see av::CaptureDeviceFormat's 'video_min_zoom_factor_for_center_stage'
    ///   and 'video_max_zoom_factor_for_center_stage').
    /// - The device's 'active_video_min_frame_duration' and 'active_video_max_frame_duration'
    ///   are limited (see av::CaptureDeviceFormat's 'video_frame_rate_range_for_center_stage').
    ///
    /// Center Stage may be enabled via user control or application control, depending on the
    /// current 'av::CaptureDevice::center_stage_control_mode()'.
    /// When 'av::CaptureDevice::is_center_stage_enabled()' is 'true', a particular
    /// 'av::CaptureDevice' instance may return 'true' for this property, depending whether it
    /// supports the feature in its current configuration. Some device features are mutually
    /// exclusive to Center Stage:
    ///
    /// - If depth data delivery is enabled on any output, such as 'av::CaptureDepthDataOutput',
    ///   or 'av::CapturePhotoOutput.depth_data_delivery_enabled', Center Stage is deactivated.
    /// - If 'geometric_distortion_correction_supported' is 'true',
    ///   'geometric_distortion_correction_enabled' must also be 'true', or Center Stage
    ///   is deactivated.
    ///
    /// This property is key-value observable.
    #[objc::msg_send(isCenterStageActive)]
    pub fn is_center_stage_active(&self) -> bool;

    #[objc::msg_send(centerStageRectOfInterest)]
    pub fn center_stage_rect_of_interest(&self) -> cg::Rect;

    /// Specifies the effective region within the output pixel buffer that will be used
    /// to perform Center Stage framing.
    ///
    /// Applications that wish to apply additional processing (such as cropping) on top of
    /// Center Stage's output can use this property to guide Center Stage's framing.
    ///
    /// The rectangle's origin is top left and is relative to the coordinate space of the
    /// output pixel buffer. The default value of this property is the value
    /// 'cg::Rect::new(0.0, 0.0, 1.0, 1.0)', where {0.0,0.0} represents the top left of the
    /// picture area, and {1.0,1.0} represents the bottom right on an unrotated picture.
    /// This rectangle of interest is applied prior to rotation, mirroring or scaling.
    ///
    /// Pixels outside of this rectangle of interest will be blackened out.
    ///
    /// Setting this property has no impact on objects specified in the metadata output.
    ///
    /// 'set_center_stage_rect_of_interest_throws': throws an 'ns::ExceptionName::generic()'
    /// if called without first obtaining exclusive access to the receiver using
    /// 'lock_for_configuration' 'set_center_stage_rect_of_interest_throws'
    /// throws an 'ns::ExceptionName::invalid_argument()' if none of the av::CaptureDeviceFormats
    /// supported by the receiver support CenterStage.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// '::center_stage_enabled' is 'false' on the av::CaptureDevice class.
    /// 'set_center_stage_rect_of_interest_throws' throws an 'ns::ExceptionName::invalid_argument()'
    /// if the provided rect of interest goes outside the normalized (0-1) coordinate space.
    #[objc::msg_send(setCenterStageRectOfInterest:)]
    pub unsafe fn set_center_stage_rect_of_interest_throws(&mut self, value: cg::Rect);
}

#[doc(alias = "AVCaptureSystemUserInterface")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum SystemUi {
    VideoEffects = 1,
    MicModes = 2,
}

/// AVCaptureSystemUserInterface
impl Device {
    #[objc::cls_msg_send(showSystemUserInterface:)]
    pub fn show_system_ui(system_ui: SystemUi);
}

#[derive(Debug, Eq, PartialEq)]
pub struct ColorSpace(isize);

impl ColorSpace {
    pub const SRGB: Self = Self(0);
    pub const P3_D65: Self = Self(1);
    pub const HLG_BT2020: Self = Self(2);
    pub const APPLE_LOG: Self = Self(3);
}

/// AVCaptureDeviceColorSpaceSupport
impl Device {
    /// Indicates the receiver's current active color space.
    #[objc::msg_send(activeColorSpace)]
    pub fn active_color_space(&self) -> ColorSpace;

    /// By default, an 'av::CaptureDevice' attached to an 'av::CaptureSession' is automatically
    /// configured for wide color by the 'av::CaptureSession'
    /// (see AVCaptureSession automaticallyConfiguresCaptureDeviceForWideColor).
    /// You may also set the active_ColorSpace manually. To prevent the AVCaptureSession from
    /// undoing your work, remember to set AVCaptureSession's automaticallyConfiguresCaptureDeviceForWideColor
    /// property to NO. Changing the receiver's activeColorSpace while the session is running
    /// requires a disruptive reconfiguration of the capture render pipeline. Movie captures
    /// in progress will be ended immediately; unfulfilled photo requests will be aborted;
    /// video preview will temporarily freeze. -setActiveColorSpace: throws an NSGenericException
    /// if called without first obtaining exclusive access to the receiver
    /// using -lockForConfiguration:.
    #[objc::msg_send(setActiveColorSpace:)]
    pub unsafe fn set_active_color_space_throws(&mut self, value: ColorSpace);
}

pub struct ConfigurationLockGuard<'a> {
    device: &'a mut Device,
}

impl<'a> ConfigurationLockGuard<'a> {
    pub fn set_active_format(&mut self, value: &Format) {
        unsafe { self.device.set_active_format(value) }
    }

    pub fn set_active_video_min_frame_duration(&mut self, value: cm::Time) {
        unsafe { self.device.set_active_video_min_frame_duration(value) }
    }

    pub fn set_active_video_max_frame_duration(&mut self, value: cm::Time) {
        unsafe { self.device.set_active_video_max_frame_duration(value) }
    }

    pub fn ramp_to_video_zoom_factor(&mut self, factor: cg::Float, rate: f32) {
        unsafe { self.device.ramp_to_video_zoom_factor_throws(factor, rate) }
    }

    pub fn cancel_video_zoom_ramp(&mut self) {
        unsafe { self.device.cancel_video_zoom_ramp_throws() }
    }

    pub fn set_center_stage_rect_of_interest<'ar>(
        &mut self,
        value: cg::Rect,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.device.set_center_stage_rect_of_interest_throws(value) })
    }

    pub fn set_active_color_space<'ar>(&mut self, value: ColorSpace) {
        unsafe { self.device.set_active_color_space_throws(value) }
    }

    #[inline]
    pub unsafe fn set_focus_mode_throws(&mut self, mode: FocusMode) {
        self.device.set_focus_mode_throws(mode)
    }

    pub fn set_focus_mode<'ar>(&mut self, mode: FocusMode) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_focus_mode_throws(mode) })
    }

    #[inline]
    pub unsafe fn set_focus_point_of_interest_throws(&mut self, value: cg::Point) {
        self.device.set_focus_point_of_interest_throws(value)
    }

    /// The value of this property is a 'cg::Point' that determines the receiver's focus point of interest,
    /// if it has one. A value of (0,0) indicates that the camera should focus on the top left corner of the image,
    /// while a value of (1,1) indicates that it should focus on the bottom right.
    /// The default value is (0.5,0.5).
    ///
    /// Clients can observe automatic changes to the receiver's 'focusPointOfInterest' by key value observing this property.
    /// Note that setting focusPointOfInterest alone does not initiate a focus operation. After setting
    /// 'set_focus_point_of_interest', call 'set_focus_mode()' to apply the new point of interest.
    pub fn set_focus_point_of_interest<'ar>(
        &mut self,
        value: cg::Point,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_focus_point_of_interest_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_auto_focus_range_restriction_throws(
        &mut self,
        value: AutoFocusRangeRestriction,
    ) {
        self.device.set_auto_focus_range_restriction_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_auto_focus_range_restriction<'ar>(
        &mut self,
        value: AutoFocusRangeRestriction,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_auto_focus_range_restriction_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_smooth_auto_focus_enabled_throws(&mut self, value: bool) {
        self.device.set_smooth_auto_focus_enabled_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_smooth_auto_focus_enabled<'ar>(
        &mut self,
        value: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_smooth_auto_focus_enabled_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_automatically_adjusts_face_driven_auto_focus_enabled_throws(
        &mut self,
        value: bool,
    ) {
        self.device
            .set_automatically_adjusts_face_driven_auto_focus_enabled_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_automatically_adjusts_face_driven_auto_focus_enabled<'ar>(
        &mut self,
        value: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_automatically_adjusts_face_driven_auto_focus_enabled_throws(value)
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[inline]
    pub unsafe fn set_face_driven_auto_focus_enabled_throws(&mut self, value: bool) {
        self.device.set_face_driven_auto_focus_enabled_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_face_driven_auto_focus_enabled<'ar>(
        &mut self,
        value: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_face_driven_auto_focus_enabled_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_focus_mode_locked_with_lens_position_no_completion_handler_throws(
        &mut self,
        value: f32,
    ) {
        self.device
            .set_focus_mode_locked_with_lens_position_throws(value, std::ptr::null_mut())
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_focus_mode_locked_with_lens_position_no_completion_handler<'ar>(
        &mut self,
        value: f32,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_focus_mode_locked_with_lens_position_no_completion_handler_throws(value)
        })
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_focus_mode_locked_with_lens_position_with_completion_handler_throws<F>(
        &mut self,
        value: f32,
        block: &'static mut blocks::Block<F>,
    ) where
        F: FnOnce(cm::Time),
    {
        self.device
            .set_focus_mode_locked_with_lens_position_throws(value, block.as_ptr())
    }

    #[cfg(feature = "blocks")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_focus_mode_locked_with_lens_position_with_completion_handler<'ar, F>(
        &mut self,
        value: f32,
        block: &'static mut blocks::Block<F>,
    ) -> Result<(), &'ar ns::Exception>
    where
        F: FnOnce(cm::Time),
    {
        ns::try_catch(|| unsafe {
            self.device
                .set_focus_mode_locked_with_lens_position_throws(value, block.as_ptr())
        })
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async unsafe fn set_focus_mode_locked_with_lens_position_throws(
        &mut self,
        value: f32,
    ) -> cm::Time {
        let (future, block) = blocks::comp1();
        self.set_focus_mode_locked_with_lens_position_with_completion_handler_throws(
            value,
            block.escape(),
        );
        future.await
    }

    #[cfg(feature = "async")]
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub async fn set_focus_mode_locked_with_lens_position(
        &mut self,
        value: f32,
    ) -> Result<cm::Time, arc::R<ns::Exception>> {
        let (future, block) = blocks::comp1();
        let res = ns::try_catch(move || unsafe {
            self.set_focus_mode_locked_with_lens_position_with_completion_handler_throws(
                value,
                block.escape(),
            )
        });
        if let Err(err) = res {
            return Err(err.retained());
        }
        Ok(future.await)
    }

    pub unsafe fn set_exposure_mode_throws(&mut self, value: ExposureMode) {
        self.device.set_exposure_mode_throws(value)
    }

    pub fn set_exposure_mode<'ar>(
        &mut self,
        value: ExposureMode,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_exposure_mode_throws(value) })
    }

    pub unsafe fn set_exposure_point_of_interest_throws(&mut self, value: cg::Point) {
        self.device.set_exposure_point_of_interest_throws(value)
    }

    pub fn set_exposure_point_of_interest<'ar>(
        &mut self,
        value: cg::Point,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_exposure_point_of_interest_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(
        &mut self,
        value: bool,
    ) {
        self.device
            .set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_automatically_adjusts_face_driven_auto_exposure_enabled<'ar>(
        &mut self,
        value: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe {
            self.set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(value)
        })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_face_driven_auto_exposure_enabled_throws(&mut self, value: bool) {
        self.device
            .set_face_driven_auto_exposure_enabled_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_face_driven_auto_exposure_enabled<'ar>(
        &mut self,
        value: bool,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_face_driven_auto_exposure_enabled_throws(value) })
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub unsafe fn set_active_max_exposure_duration_throws(&mut self, value: cm::Time) {
        self.device.set_active_max_exposure_duration_throws(value)
    }

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    pub fn set_active_max_exposure_duration<'ar>(
        &mut self,
        value: cm::Time,
    ) -> Result<(), &'ar ns::Exception> {
        ns::try_catch(|| unsafe { self.set_active_max_exposure_duration_throws(value) })
    }
}

impl<'a> Drop for ConfigurationLockGuard<'a> {
    fn drop(&mut self) {
        unsafe { self.device.unlock_for_configuration() }
    }
}

impl<'a> Deref for ConfigurationLockGuard<'a> {
    type Target = Device;

    fn deref(&self) -> &Self::Target {
        self.device
    }
}

impl<'a> DerefMut for ConfigurationLockGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.device
    }
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE: &'static objc::Class<Device>;
}

#[doc(alias = "AVCaptureTorchMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum TorchMode {
    Off = 0,
    On = 1,
    Auto = 2,
}

#[doc(alias = "AVCaptureFocusMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum FocusMode {
    /// Indicates that the focus should be locked at the lens' current position
    Locked = 0,

    /// Indicates that the device should autofocus once and then change the focus mode
    /// to 'av::CaptureFocusMode::Locked'.
    AutoFocus = 1,

    /// Indicates that the device should automatically focus when needed.
    ContinuousAutoFocus = 2,
}

#[doc(alias = "AVCaptureAutoFocusSystem")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum AutoFocusSystem {
    /// Indicates that autofocus is not available.
    None = 0,

    /// Indicates that autofocus is achieved by contrast detection.
    /// Contrast detection performs a focus scan to find the optimal position.
    ContrastDetection = 1,

    /// Indicates that autofocus is achieved by phase detection.
    /// Phase detection has the ability to achieve focus in many cases without a focus scan.
    /// Phase detection autofocus is typically less visually intrusive than contrast
    // detection autofocus.
    PhaseDetection = 2,
}

#[doc(alias = "AVCaptureAutoFocusRangeRestriction")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum AutoFocusRangeRestriction {
    /// Indicates that the autofocus system should not restrict the focus range.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionNone")]
    None = 0,

    /// Indicates that the autofocus system should restrict the focus range
    /// for subject matter that is near to the camera.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionNear")]
    Near = 1,

    /// Indicates that the autofocus system should restrict the focus range
    /// for subject matter that is far from the camera.
    #[doc(alias = "AVCaptureAutoFocusRangeRestrictionFar")]
    Far = 2,
}

/// AVCaptureDeviceFocus
impl Device {
    #[objc::msg_send(isFocusModeSupported:)]
    pub fn is_focus_mode_supported(&self, mode: FocusMode) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isLockingFocusWithCustomLensPositionSupported)]
    pub fn is_locking_focus_with_custom_lens_position_supported(&self) -> bool;

    #[objc::msg_send(focusMode)]
    pub fn focus_mode(&self) -> FocusMode;

    #[objc::msg_send(setFocusMode:)]
    unsafe fn set_focus_mode_throws(&mut self, mode: FocusMode);

    #[objc::msg_send(isFocusPointOfInterestSupported)]
    pub fn is_focus_point_of_interest_supported(&self) -> bool;

    #[objc::msg_send(focusPointOfInterest)]
    pub fn focus_point_of_intereset(&self) -> cg::Point;

    #[objc::msg_send(setFocusPointOfInterest:)]
    unsafe fn set_focus_point_of_interest_throws(&mut self, value: cg::Point);

    /// The value of this property is a bool indicating whether the receiver's camera focus
    /// is being automatically adjusted by means of a focus scan, because its focus mode is 'av::CaptureFocusModeAutoFocus'
    /// or 'av::CaptureFocusModeContinuousAutoFocus'. Clients can observe the value of this property to determine whether
    /// the camera's focus is stable.
    #[objc::msg_send(isAdjustingFocus)]
    pub fn is_adjusting_focus(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isAutoFocusRangeRestrictionSupported)]
    pub fn is_auto_focus_range_restriction_supported(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(autoFocusRangeRestriction)]
    pub fn auto_focus_range_restriction(&self) -> AutoFocusRangeRestriction;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutoFocusRangeRestriction:)]
    unsafe fn set_auto_focus_range_restriction_throws(&mut self, value: AutoFocusRangeRestriction);

    /// Indicates whether the receiver supports smooth autofocus.
    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isSmoothAutoFocusSupported)]
    pub fn is_smooth_auto_focus_supported(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isSmoothAutoFocusEnabled)]
    pub fn is_smooth_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setSmoothAutoFocusEnabled:)]
    unsafe fn set_smooth_auto_focus_enabled_throws(&mut self, value: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(automaticallyAdjustsFaceDrivenAutoFocusEnabled)]
    pub fn automatically_adjusts_face_driven_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutomaticallyAdjustsFaceDrivenAutoFocusEnabled:)]
    unsafe fn set_automatically_adjusts_face_driven_auto_focus_enabled_throws(
        &mut self,
        value: bool,
    );

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isFaceDrivenAutoFocusEnabled)]
    pub fn is_face_driven_auto_focus_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFaceDrivenAutoFocusEnabled:)]
    unsafe fn set_face_driven_auto_focus_enabled_throws(&mut self, value: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(lensPosition)]
    pub fn lens_position(&self) -> f32;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFocusModeLockedWithLensPosition:completionHandler:)]
    unsafe fn set_focus_mode_locked_with_lens_position_throws(
        &mut self,
        value: f32,
        completion_handler: *mut c_void,
    );

    /// A property indicating the minimum focus distance.
    ///
    /// The minimum focus distance is given in millimeters, -1 if unknown.
    /// For virtual cameras, the value reported is the smallest minimum focus distance of the auto-focus-capable
    /// cameras that it sources.
    #[objc::msg_send(minimumFocusDistance)]
    pub fn minimum_focus_distance(&self) -> ns::Integer;
}

#[doc(alias = "AVCaptureExposureMode")]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(isize)]
pub enum ExposureMode {
    /// Indicates that the exposure should be locked at its current value.
    Locked = 0,
    /// Indicates that the device should automatically adjust exposure once and then change the exposure mode to
    /// av::CaptureExposureMode::Locked
    AutoExpose = 1,
    /// Indicates that the device should automatically adjust exposure when needed.
    ContinuousAutoExposure = 2,
    /// Indicates that the device should only adjust exposure according to user provided ISO, exposureDuration values.
    Custom = 3,
}

/// AVCaptureExposureMode
impl Device {
    /// Returns whether the receiver supports the given exposure mode.
    #[objc::msg_send(isExposureModeSupported:)]
    pub fn is_exposure_mode_supported(&self, value: ExposureMode) -> bool;

    #[objc::msg_send(exposureMode)]
    pub fn exposure_mode(&self) -> ExposureMode;

    #[objc::msg_send(setExposureMode:)]
    unsafe fn set_exposure_mode_throws(&mut self, value: ExposureMode);

    #[objc::msg_send(isExposurePointOfInterestSupported)]
    pub fn is_exposure_point_of_interest_supported(&self) -> bool;

    #[objc::msg_send(exposurePointOfInterest)]
    pub fn exposure_point_of_interest(&self) -> cg::Point;

    #[objc::msg_send(setExposurePointOfInterest:)]
    unsafe fn set_exposure_point_of_interest_throws(&mut self, value: cg::Point);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(automaticallyAdjustsFaceDrivenAutoExposureEnabled)]
    pub fn automatically_adjusts_face_driven_auto_exposure_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setAutomaticallyAdjustsFaceDrivenAutoExposureEnabled:)]
    unsafe fn set_automatically_adjusts_face_driven_auto_exposure_enabled_throws(
        &mut self,
        value: bool,
    );

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(isFaceDrivenAutoExposureEnabled)]
    pub fn is_face_driven_auto_exposure_enabled(&self) -> bool;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setFaceDrivenAutoExposureEnabled:)]
    unsafe fn set_face_driven_auto_exposure_enabled_throws(&mut self, value: bool);

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(activeMaxExposureDuration)]
    pub fn active_max_exposure_duration(&self) -> cm::Time;

    #[cfg(any(target_os = "tvos", target_os = "ios"))]
    #[objc::msg_send(setActiveMaxExposureDuration:)]
    unsafe fn set_active_max_exposure_duration_throws(&mut self, value: cm::Time);
}

define_obj_type!(FrameRateRange(ns::Id));

impl FrameRateRange {
    #[objc::msg_send(minFrameRate)]
    pub fn min_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameRate)]
    pub fn max_frame_rate(&self) -> f64;

    #[objc::msg_send(maxFrameDuration)]
    pub fn max_frame_duration(&self) -> cm::Time;

    #[objc::msg_send(minFrameDuration)]
    pub fn min_frame_duration(&self) -> cm::Time;
}

define_obj_type!(Format(ns::Id));
impl Format {
    #[objc::msg_send(mediaType)]
    pub fn media_type(&self) -> &av::MediaType;

    #[objc::msg_send(formatDescription)]
    pub fn format_desc(&self) -> &cm::FormatDesc;

    #[objc::msg_send(videoSupportedFrameRateRanges)]
    pub fn video_supported_frame_rate_ranges(&self) -> &ns::Array<FrameRateRange>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoFieldOfView)]
    pub fn video_fov(&self) -> f32;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoBinned)]
    pub fn is_video_binned(&self) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isVideoStabilizationModeSupported:)]
    pub fn is_video_stabilization_mode_supported(&self, mode: VideoStabilizationMode) -> bool;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoMaxZoomFactor)]
    pub fn video_max_zoom_factor(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(videoZoomFactorUpscaleThreshold)]
    pub fn video_zoom_factor_upscale_threshold(&self) -> cg::Float;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(minExposureDuration)]
    pub fn min_exposure_duration(&self) -> cm::Time;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(maxExposureDuration)]
    pub fn max_exposure_duration(&self) -> cm::Time;

    /// A 'f32' indicating the minimum supported exposure ISO value.
    ///
    /// This read-only property indicates the minimum supported exposure ISO value.
    #[objc::msg_send(minISO)]
    pub fn min_iso(&self) -> f32;

    /// An 'f32' indicating the maximum supported exposure ISO value.
    ///
    /// This read-only property indicates the maximum supported exposure ISO value.
    #[objc::msg_send(maxISO)]
    pub fn max_iso(&self) -> f32;

    #[objc::msg_send(autoFocusSystem)]
    pub fn auto_focus_system(&self) -> AutoFocusSystem;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(isMultiCamSupported)]
    pub fn is_mutli_cam_supported(&self) -> bool;

    #[objc::msg_send(geometricDistortionCorrectedVideoFieldOfView)]
    pub fn geometric_distortion_corrected_video_field_of_view(&self) -> f32;
}

/// Center Stage
impl Format {
    #[objc::msg_send(isCenterStageSupported)]
    pub fn is_center_stage_supported(&self) -> bool;

    #[objc::msg_send(videoMinZoomFactorForCenterStage)]
    pub fn video_min_zoom_factor_for_center_stage(&self) -> cg::Float;

    #[objc::msg_send(videoFrameRateRangeForCenterStage)]
    pub fn video_frame_rate_range_for_center_stage(&self) -> Option<&FrameRateRange>;
}

/// Portrait Effect
impl Format {
    /// Indicates whether the format supports the Portrait Effect feature.
    #[objc::msg_send(isPortraitEffectSupported)]
    pub fn is_portrait_effect_supported(&self) -> bool;
}

pub mod notifications {
    use crate::cf;

    /// Posted when a device becomes available on the system.
    pub fn was_connected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasConnectedNotification }
    }

    /// Posted when a device becomes unavailable on the system.
    pub fn was_disconnected() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceWasDisconnectedNotification }
    }

    /// Posted when the instance of av::CaptureDevice has detected a substantial
    /// change to the video subject area.
    #[cfg(not(target_os = "macos"))]
    pub fn subject_area_did_change() -> &'static cf::NotificationName {
        unsafe { AVCaptureDeviceSubjectAreaDidChangeNotification }
    }

    #[link(name = "AVFoundation", kind = "framework")]
    extern "C" {
        static AVCaptureDeviceWasConnectedNotification: &'static cf::NotificationName;
        static AVCaptureDeviceWasDisconnectedNotification: &'static cf::NotificationName;
        #[cfg(not(target_os = "macos"))]
        static AVCaptureDeviceSubjectAreaDidChangeNotification: &'static cf::NotificationName;
    }
}

define_obj_type!(CaptureAudioChannel(ns::Id));

define_obj_type!(DiscoverySession(ns::Id));
impl DiscoverySession {
    define_cls!(AV_CAPTURE_DEVICE_DISCOVERY_SESSION);

    #[objc::cls_msg_send(discoverySessionWithDeviceTypes:mediaType:position:)]
    pub fn with_device_types_media_and_position_ar(
        device_types: &ns::Array<Type>,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::Rar<Self>;

    #[objc::cls_rar_retain]
    pub fn with_device_types_media_and_position(
        device_types: &ns::Array<Type>,
        media_type: Option<&MediaType>,
        position: Position,
    ) -> arc::R<Self>;

    /// The list of devices that comply to the search criteria specified
    /// on the discovery session.
    ///
    /// The returned array contains only devices that are available at the time the method
    /// is called. Applications can key-value observe this property to be notified when
    /// the list of available devices has changed. For apps linked against iOS 10,
    /// the devices returned are unsorted. For apps linked against iOS 11 or later,
    /// the devices are sorted by 'av::CaptureDeviceType', matching the order specified
    /// in the deviceTypes parameter of 'av::CaptureDeviceDiscoverySession::with_device_types_media_position`.
    /// If a position of 'av::CaptureDevicePosition::unspecified' is specified,
    /// the results are further ordered by position in the 'av::CaptureDevicePosition' enum.
    /// Starting in Mac Catalyst 14.0, clients can key value observe the value of this
    /// property to be notified when the devices change.
    #[objc::msg_send(devices)]
    pub fn devices(&self) -> &ns::Array<Device>;

    #[cfg(not(target_os = "macos"))]
    #[objc::msg_send(supportedMultiCamDeviceSets)]
    pub fn supported_multi_cam_device_sets(&self) -> &ns::Array<ns::Set<Device>>;
}

#[link(name = "av", kind = "static")]
extern "C" {
    static AV_CAPTURE_DEVICE_DISCOVERY_SESSION: &'static objc::Class<DiscoverySession>;
}

#[doc(alias = "AVCaptureVideoStabilizationMode")]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(isize)]
pub enum VideoStabilizationMode {
    /// Indicates that video should not be stabilized.
    Off = 0,

    /// Indicates that video should be stabilized using the standard
    /// video stabilization algorithm introduced with iOS 5.0. Standard video
    /// stabilization has a reduced field of view. Enabling video stabilization
    /// may introduce additional latency into the video capture pipeline.
    Standard = 1,

    /// Indicates that video should be stabilized using the cinematic stabilization
    /// algorithm for more dramatic results. Cinematic video stabilization has a reduced
    /// field of view compared to standard video stabilization. Enabling cinematic
    /// video stabilization introduces much more latency into the video capture pipeline
    /// than standard video stabilization and consumes significantly more system memory.
    /// Use narrow or identical min and max frame durations in conjunction with this mode.
    Cinematic = 2,

    /// Indicates that the video should be stabilized using the extended cinematic
    /// stabilization algorithm. Enabling extended cinematic stabilization introduces
    /// longer latency into the video capture pipeline compared to the
    /// 'av::CaptureVideoStabilizationMode::Cinematic' and consumes more memory, but yields
    /// improved stability. It is recommended to use identical or similar min and max frame
    /// durations in conjunction with this mode.
    CinematicExtended = 3,

    /// Indicates that video should be stabilized using the preview optimized stabilization
    /// algorithm. Preview stabilization is a low latency and low power algorithm
    /// which is supported only on connections which either have an associated preview layer
    /// or have a preview-sized VideoDataOutput.
    PreviewOptimized = 4,

    /// Indicates that the most appropriate video stabilization mode for the device and
    /// format should be chosen.
    Auto = -1,
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
pub fn lens_position_current() -> f32 {
    unsafe { AVCaptureLensPositionCurrent }
}

#[cfg(any(target_os = "tvos", target_os = "ios"))]
pub fn iso_current() -> f32 {
    unsafe { AVCaptureISOCurrent }
}

#[cfg(test)]
mod tests {
    use crate::{
        av::{
            self, capture,
            capture::device::{self, Device},
        },
        cm::io,
        ns,
    };

    #[test]
    fn basics() {
        let device_type = device::Type::built_in_wide_angle_camera();
        let media_type = av::MediaType::video();
        let position = device::Position::Front;
        let mut device =
            Device::with_device_type_media_and_position(device_type, Some(media_type), position)
                .expect("device");
        //device.unique_id().show();
        assert!(!device.has_torch());
        assert!(device.formats().len() > 0);
        assert!(device.supports_preset(av::CaptureSessionPreset::photo()));
        let mut _lock = device.configuration_lock().expect("locked");
    }

    #[test]
    fn session() {
        io::Object::SYSTEM
            .allow_screen_capture_devices(true)
            .unwrap();
        io::Object::SYSTEM
            .allow_wireless_screen_capture_devices(true)
            .unwrap();

        io::Object::SYSTEM.show();

        let list = ns::Array::from_slice(&[av::CaptureDeviceType::external()]);
        let session = capture::DiscoverySession::with_device_types_media_and_position(
            list.as_ref(),
            Some(av::MediaType::muxed()),
            capture::DevicePosition::Unspecified,
        );

        let devices = session.devices();
        devices.as_type_ref().show();
    }
}
