use std::{ffi::c_void, marker::PhantomData};

use raw_window_handle::HasRawWindowHandle;

#[cfg(windows)]
#[path = "windows.rs"]
mod platform;

#[cfg(target_os = "linux")]
#[path = "linux.rs"]
mod platform;

#[cfg(target_arch = "wasm32")]
#[path = "web.rs"]
mod platform;

#[derive(Clone, Copy, Debug)]
pub struct GlConfig {
    pub version: (u8, u8),
    pub profile: Profile,
    pub red_bits: u8,
    pub blue_bits: u8,
    pub green_bits: u8,
    pub alpha_bits: u8,
    pub depth_bits: u8,
    pub stencil_bits: u8,
    pub samples: Option<u8>,
    pub srgb: bool,
    pub double_buffer: bool,
}

impl Default for GlConfig {
    fn default() -> Self {
        Self {
            version: (3, 3),
            profile: Profile::Core,
            red_bits: 8,
            blue_bits: 8,
            green_bits: 8,
            alpha_bits: 8,
            depth_bits: 24,
            stencil_bits: 8,
            samples: Some(16),
            srgb: true,
            double_buffer: true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Compatibility,
    Core,
}

#[derive(Debug)]
pub enum GlError {
    InvalidWindowHandle,
    CreationFailed,
}

pub struct GlContext {
    inner: platform::Impl,
    marker: PhantomData<*const ()>,
}

impl GlContext {
    pub unsafe fn create(config: GlConfig, window: &impl HasRawWindowHandle) -> Result<Self, GlError> {
        platform::Impl::create(config, window).map(|inner| Self {
            inner,
            marker: PhantomData,
        })
    }

    pub fn get_proc_address(&self, s: &str) -> *const c_void {
        unsafe { self.inner.get_proc_address(s) }
    }

    pub fn make_current(&self) {
        unsafe { self.inner.make_current() };
    }

    #[allow(unused)]
    pub fn make_not_current(&self) {
        unsafe { self.inner.make_not_current() };
    }

    pub fn swap_buffers(&self) {
        unsafe { self.inner.swap_buffers() };
    }

    pub fn set_swap_interval(&self, vsync: bool) {
        unsafe { self.inner.set_swap_interval(vsync) }
    }

    pub fn glow(&self) -> glow::Context {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe {
            glow::Context::from_loader_function(|s| self.get_proc_address(s))
        }

        #[cfg(target_arch = "wasm32")]
        self.inner.glow()
    }
}
