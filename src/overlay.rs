use ::{Overlay, get_string};
use ::{openvr_sys as sys, TrackedDeviceIndex};
use std::fmt;
use std::ffi::{CStr, CString};
use compositor::Texture;
use compositor::texture::Handle::{Vulkan, OpenGLTexture, OpenGLRenderBuffer};

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OverlayError(sys::EVROverlayError);

impl OverlayError {
    pub fn from_sys(err: sys::EVROverlayError) -> Result<(), Self> {
        match err {
            0 => Ok(()),
            _ => Err(OverlayError(err))
        }
    }
}

pub mod overlay_error {
    use super::{sys, OverlayError};

    pub const SUCCESS: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_None);
    pub const UNKNOWN_OVERLAY: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_UnknownOverlay);
    pub const INVALID_HANDLE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_InvalidHandle);
    pub const PERMISSION_DENIED: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_PermissionDenied);
    pub const OVERLAY_LIMIT_EXCEEDED: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_OverlayLimitExceeded);
    pub const WRONG_VISIBILITY_TYPE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_WrongVisibilityType);
    pub const KEY_TOO_LONG: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_KeyTooLong);
    pub const NAME_TOO_LONG: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_NameTooLong);
    pub const KEY_IN_USE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_KeyInUse);
    pub const WRONG_TRANSFORM_TYPE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_WrongTransformType);
    pub const INVALID_TRACKED_DEVICE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_InvalidTrackedDevice);
    pub const INVALID_PARAMETER: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_InvalidParameter);
    pub const THUMBNAIL_CANT_BE_DESTROYED: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_ThumbnailCantBeDestroyed);
    pub const ARRAY_TOO_SMALL: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_ArrayTooSmall);
    pub const REQUEST_FAILED: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_RequestFailed);
    pub const INVALID_TEXTURE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_InvalidTexture);
    pub const UNABLE_TO_LOAD_FILE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_UnableToLoadFile);
    pub const KEYBOARD_ALREADY_IN_USE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_KeyboardAlreadyInUse);
    pub const NO_NEIGHBOR: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_NoNeighbor);
    pub const TOO_MANY_MASK_PRIMITIVES: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_TooManyMaskPrimitives);
    pub const BAD_MASK_PRIMITIVE: OverlayError =
        OverlayError(sys::EVROverlayError_VROverlayError_BadMaskPrimitive);
    pub const TEXTURE_ALREADY_LOCKED: OverlayError = OverlayError(sys::EVROverlayError_VROverlayError_TextureAlreadyLocked);
    pub const TEXTURE_LOCK_CAPACITY_REACHED: OverlayError = OverlayError(sys::EVROverlayError_VROverlayError_TextureLockCapacityReached);
    pub const TEXTURE_NOT_LOCKED: OverlayError = OverlayError(sys::EVROverlayError_VROverlayError_TextureNotLocked);
}

impl fmt::Debug for OverlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        <Self as fmt::Display>::fmt(self, f)
    }
}

impl ::std::error::Error for OverlayError {}

impl fmt::Display for OverlayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::overlay_error::*;

        match *self {
            SUCCESS => write!(f, "SUCCESS")?,
            UNKNOWN_OVERLAY => write!(f, "UNKNOWN_OVERLAY")?,
            INVALID_HANDLE => write!(f, "INVALID_HANDLE")?,
            PERMISSION_DENIED => write!(f, "PERMISSION_DENIED")?,
            OVERLAY_LIMIT_EXCEEDED => write!(f, "OVERLAY_LIMIT_EXCEEDED")?,
            WRONG_VISIBILITY_TYPE => write!(f, "WRONG_VISIBILITY_TYPE")?,
            KEY_TOO_LONG => write!(f, "KEY_TOO_LONG")?,
            NAME_TOO_LONG => write!(f, "NAME_TOO_LONG")?,
            KEY_IN_USE => write!(f, "KEY_IN_USE")?,
            WRONG_TRANSFORM_TYPE => write!(f, "WRONG_TRANSFORM_TYPE")?,
            INVALID_TRACKED_DEVICE => write!(f, "INVALID_TRACKED_DEVICE")?,
            INVALID_PARAMETER => write!(f, "INVALID_PARAMETER")?,
            THUMBNAIL_CANT_BE_DESTROYED => write!(f, "THUMBNAIL_CANT_BE_DESTROYED")?,
            ARRAY_TOO_SMALL => write!(f, "ARRAY_TOO_SMALL")?,
            REQUEST_FAILED => write!(f, "REQUEST_FAILED")?,
            INVALID_TEXTURE => write!(f, "INVALID_TEXTURE")?,
            UNABLE_TO_LOAD_FILE => write!(f, "UNABLE_TO_LOAD_FILE")?,
            KEYBOARD_ALREADY_IN_USE => write!(f, "KEYBOARD_ALREADY_IN_USE")?,
            NO_NEIGHBOR => write!(f, "NO_NEIGHBOR")?,
            TOO_MANY_MASK_PRIMITIVES => write!(f, "TOO_MANY_MASK_PRIMITIVES")?,
            BAD_MASK_PRIMITIVE => write!(f, "BAD_MASK_PRIMITIVE")?,
            TEXTURE_ALREADY_LOCKED => write!(f, "TEXTURE_ALREADY_LOCKED")?,
            TEXTURE_LOCK_CAPACITY_REACHED => write!(f, "TEXTURE_LOCK_CAPACITY_REACHED")?,
            TEXTURE_NOT_LOCKED => write!(f, "TEXTURE_NOT_LOCKED")?,
            _ => write!(f, "UNKNOWN ({})", self.0)?,
        }
        Ok(())
    }
}

#[repr(u32)]
pub enum OverlayFlag {
    None = sys::VROverlayFlags_None,
    NoDashboardTab = sys::VROverlayFlags_NoDashboardTab,
    AcceptsGamepadEvents = sys::VROverlayFlags_AcceptsGamepadEvents,
    ShowGamepadFocus = sys::VROverlayFlags_ShowGamepadFocus,
    SendVRDiscreteScrollEvents = sys::VROverlayFlags_SendVRDiscreteScrollEvents,
    SendVRTouchPadEvents = sys::VROverlayFlags_SendVRTouchpadEvents,
    ShowTouchPadScrollWheel = sys::VROverlayFlags_ShowTouchPadScrollWheel,
    TransferOwnershipToInternalProcess = sys::VROverlayFlags_TransferOwnershipToInternalProcess,
    SideBySideParallel = sys::VROverlayFlags_SideBySide_Parallel,
    SideBySideCrossed = sys::VROverlayFlags_SideBySide_Crossed,
    Panorama = sys::VROverlayFlags_Panorama,
    StereoPanorama = sys::VROverlayFlags_StereoPanorama,
    SortWithNonSceneOverlays = sys::VROverlayFlags_SortWithNonSceneOverlays,
    VisibleInDashboard = sys::VROverlayFlags_VisibleInDashboard,
    MakeOverlaysInteractiveIfVisible = sys::VROverlayFlags_MakeOverlaysInteractiveIfVisible,
    SendVRSmoothScrollEvents = sys::VROverlayFlags_SendVRSmoothScrollEvents,
}

unsafe fn get_string_error<F: FnMut(*mut std::os::raw::c_char, u32, &mut u32) -> u32>(mut f: F) -> Result<CString, OverlayError> {
    let mut err = 0;

    let string = get_string(|ptr, n| {
        let count = f(ptr, n, &mut err);

        // exit early on errors
        if err > 0 {
            0
        } else {
            count
        }
    });

    OverlayError::from_sys(err)?;
    Ok(string.unwrap())
}

#[repr(C)]
struct OverlayHandle<'a> {
    handle: sys::VROverlayHandle_t,
    sys: &'a sys::VR_IVROverlay_FnTable,
}

impl fmt::Debug for OverlayHandle<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.handle.fmt(f)
    }
}

impl OverlayHandle<'_> {
    fn get_key(&self) -> Result<CString, OverlayError> {
        unsafe {
            get_string_error(|ptr, n, err| self.sys.GetOverlayKey.unwrap()(self.handle, ptr, n, err))
        }
    }

    fn get_name(&self) -> Result<CString, OverlayError> {
        unsafe {
            get_string_error(|ptr, n, err| self.sys.GetOverlayName.unwrap()(self.handle, ptr, n, err))
        }
    }

    fn set_flag(&mut self, flag: OverlayFlag, enabled: bool) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayFlag.unwrap()(self.handle, flag as u32, enabled) })
    }

    fn get_flag(&self, flag: OverlayFlag) -> Result<bool, OverlayError> {
        let mut enabled = true;
        OverlayError::from_sys(unsafe { self.sys.GetOverlayFlag.unwrap()(self.handle, flag as u32, &mut enabled) })?;
        Ok(enabled)
    }

    fn set_overlay_color(&mut self, red: f32, green: f32, blue: f32) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayColor.unwrap()(self.handle, red, green, blue) })
    }

    fn set_overlay_alpha(&mut self, alpha: f32) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayAlpha.unwrap()(self.handle, alpha) })
    }

    fn get_overlay_color(&self) -> Result<(f32, f32, f32), OverlayError> {
        let mut red = 0.0;
        let mut green = 0.0;
        let mut blue = 0.0;
        OverlayError::from_sys(unsafe { self.sys.GetOverlayColor.unwrap()(self.handle, &mut red, &mut green, &mut blue) })?;
        Ok((red, green, blue))
    }

    fn get_overlay_alpha(&self) -> Result<f32, OverlayError> {
        let mut alpha = 0.0;
        OverlayError::from_sys(unsafe { self.sys.GetOverlayAlpha.unwrap()(self.handle, &mut alpha) })?;
        Ok(alpha)
    }

    fn show(&mut self) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.ShowOverlay.unwrap()(self.handle) })
    }

    fn hide(&mut self) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.HideOverlay.unwrap()(self.handle) })
    }

    fn is_visible(&self) -> bool {
        unsafe { self.sys.IsOverlayVisible.unwrap()(self.handle) }
    }

    fn set_texture(&mut self, texture: &Texture) -> Result<(), OverlayError> {
        let texture = sys::Texture_t {
            handle: match texture.handle {
                Vulkan(ref x) => x as *const _ as *mut _,
                OpenGLTexture(x) => x as *mut _,
                OpenGLRenderBuffer(x) => x as *mut _,
            },
            eType: match texture.handle {
                Vulkan(_) => sys::ETextureType_TextureType_Vulkan,
                OpenGLTexture(_) => sys::ETextureType_TextureType_OpenGL,
                OpenGLRenderBuffer(_) => sys::ETextureType_TextureType_OpenGL,
            },
            eColorSpace: texture.color_space as sys::EColorSpace,
        };
        OverlayError::from_sys(unsafe { self.sys.SetOverlayTexture.unwrap()(self.handle, &texture as *const _ as *mut _) })
    }

    fn set_overlay_width(&mut self, width: f32) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayWidthInMeters.unwrap()(self.handle, width) })
    }

    fn set_transform_device_relative(&mut self, device: TrackedDeviceIndex, transform: [[f32; 4]; 3]) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayTransformTrackedDeviceRelative.unwrap()(self.handle, device, &transform as *const _ as *mut _) })
    }

    fn set_from_file(&mut self, path: &CStr) -> Result<(), OverlayError> {
        OverlayError::from_sys(unsafe { self.sys.SetOverlayFromFile.unwrap()(self.handle, path as *const _ as *mut _) })
    }
}

impl Drop for OverlayHandle<'_> {
    fn drop(&mut self) {
        unsafe { self.sys.DestroyOverlay.unwrap()(self.handle) };
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct FloatingOverlay<'a> {
    handle: OverlayHandle<'a>,
}

impl FloatingOverlay<'_> {
    pub fn get_key(&self) -> Result<CString, OverlayError> {
        self.handle.get_key()
    }

    pub fn get_name(&self) -> Result<CString, OverlayError> {
        self.handle.get_name()
    }

    pub fn set_flag(&mut self, flag: OverlayFlag, enabled: bool) -> Result<(), OverlayError> {
        self.handle.set_flag(flag, enabled)
    }

    pub fn get_flag(&self, flag: OverlayFlag) -> Result<bool, OverlayError> {
        self.handle.get_flag(flag)
    }

    pub fn set_overlay_color(&mut self, red: f32, green: f32, blue: f32) -> Result<(), OverlayError> {
        self.handle.set_overlay_color(red, green, blue)
    }

    pub fn set_overlay_alpha(&mut self, alpha: f32) -> Result<(), OverlayError> {
        self.handle.set_overlay_alpha(alpha)
    }

    pub fn get_overlay_color(&self) -> Result<(f32, f32, f32), OverlayError> {
        self.handle.get_overlay_color()
    }

    pub fn get_overlay_alpha(&self) -> Result<f32, OverlayError> {
        self.handle.get_overlay_alpha()
    }

    pub fn show(&mut self) -> Result<(), OverlayError> {
        self.handle.show()
    }

    pub fn hide(&mut self) -> Result<(), OverlayError> {
        self.handle.hide()
    }

    pub fn is_visible(&self) -> bool {
        self.handle.is_visible()
    }

    pub fn set_texture(&mut self, texture: &Texture) -> Result<(), OverlayError> {
        self.handle.set_texture(texture)
    }

    pub fn set_overlay_width(&mut self, width: f32) -> Result<(), OverlayError> {
        self.handle.set_overlay_width(width)
    }

    pub fn set_transform_device_relative(&mut self, device: TrackedDeviceIndex, transform: [[f32; 4]; 3]) -> Result<(), OverlayError> {
        self.handle.set_transform_device_relative(device, transform)
    }

    pub fn set_from_file(&mut self, path: &CStr) -> Result<(), OverlayError> {
        self.handle.set_from_file(path)
    }
}

#[derive(Debug)]
pub struct DashboardOverlay<'a> {
    handle: OverlayHandle<'a>,
    thumbnail: OverlayHandle<'a>,
}

impl<'a> Overlay<'a> {
    /// Create a new floating overlay
    pub fn create_floating(&self, key: &CStr, name: &CStr) -> Result<FloatingOverlay<'a>, OverlayError> {
        let mut handle = 0;
        OverlayError::from_sys(unsafe { self.0.CreateOverlay.unwrap()(key.as_ptr() as *mut _, name.as_ptr() as *mut _, &mut handle) })?;
        Ok(FloatingOverlay {
            handle: OverlayHandle {
                handle,
                sys: self.0,
            }
        })
    }

    // Create a new floating overlay
    pub fn create_dashboard(&self, key: &str, name: &str) -> Result<DashboardOverlay<'a>, OverlayError> {
        let mut handle = 0;
        let mut thumbnail = 0;
        OverlayError::from_sys(unsafe { self.0.CreateDashboardOverlay.unwrap()(key.as_ptr() as *mut _, name.as_ptr() as *mut _, &mut handle, &mut thumbnail) })?;
        Ok(DashboardOverlay {
            handle: OverlayHandle {
                handle,
                sys: self.0,
            },
            thumbnail: OverlayHandle {
                handle: thumbnail,
                sys: self.0,
            },
        })
    }
}