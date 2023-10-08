use super::{GlConfig, GlError, Profile};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle};
use std::ffi::{c_void, CString};
use std::os::raw::{c_int, c_ulong};
use winit::window::Window;
use x11::glx;
use x11::xlib;

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/GLX_ARB_create_context.txt

type GlXCreateContextAttribsARB = unsafe extern "C" fn(
    dpy: *mut xlib::Display,
    fbc: glx::GLXFBConfig,
    share_context: glx::GLXContext,
    direct: xlib::Bool,
    attribs: *const c_int,
) -> glx::GLXContext;

// See https://www.khronos.org/registry/OpenGL/extensions/EXT/EXT_swap_control.txt

type GlXSwapIntervalEXT = unsafe extern "C" fn(dpy: *mut xlib::Display, drawable: glx::GLXDrawable, interval: i32);

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_framebuffer_sRGB.txt

const GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20B2;

extern "C" fn err_handler(_dpy: *mut xlib::Display, _err: *mut xlib::XErrorEvent) -> i32 {
    0
}

unsafe fn get_proc_address(symbol: &str) -> *const c_void {
    let symbol = CString::new(symbol).unwrap();
    glx::glXGetProcAddress(symbol.as_ptr() as *const u8).unwrap() as *const c_void
}

pub struct Impl {
    window: c_ulong,
    display: *mut xlib::_XDisplay,
    context: glx::GLXContext,
}

impl Impl {
    pub unsafe fn create(config: GlConfig, parent: &Window) -> Result<Impl, GlError> {
        let window_handle = if let RawWindowHandle::Xlib(handle) = parent.raw_window_handle() {
            handle
        } else {
            return Err(GlError::InvalidWindowHandle);
        };

        let display_handle = if let RawDisplayHandle::Xlib(handle) = parent.raw_display_handle() {
            handle
        } else {
            return Err(GlError::InvalidWindowHandle);
        };

        if display_handle.display.is_null() {
            return Err(GlError::InvalidWindowHandle);
        }

        let prev_callback = xlib::XSetErrorHandler(Some(err_handler));

        let display = display_handle.display as *mut xlib::_XDisplay;

        let screen = xlib::XDefaultScreen(display);

        #[rustfmt::skip]
        let fb_attribs = [
            glx::GLX_X_RENDERABLE, 1,
            glx::GLX_X_VISUAL_TYPE, glx::GLX_TRUE_COLOR,
            glx::GLX_DRAWABLE_TYPE, glx::GLX_WINDOW_BIT,
            glx::GLX_RENDER_TYPE, glx::GLX_RGBA_BIT,
            glx::GLX_RED_SIZE, config.red_bits as i32,
            glx::GLX_GREEN_SIZE, config.green_bits as i32,
            glx::GLX_BLUE_SIZE, config.blue_bits as i32,
            glx::GLX_ALPHA_SIZE, config.alpha_bits as i32,
            glx::GLX_DEPTH_SIZE, config.depth_bits as i32,
            glx::GLX_STENCIL_SIZE, config.stencil_bits as i32,
            glx::GLX_DOUBLEBUFFER, config.double_buffer as i32,
            glx::GLX_SAMPLE_BUFFERS, config.samples.is_some() as i32,
            glx::GLX_SAMPLES, config.samples.unwrap_or(0) as i32,
            GLX_FRAMEBUFFER_SRGB_CAPABLE_ARB, config.srgb as i32,
            0,
        ];

        let mut n_configs = 0;
        let fb_config = glx::glXChooseFBConfig(display, screen, fb_attribs.as_ptr(), &mut n_configs);

        if n_configs <= 0 {
            return Err(GlError::CreationFailed);
        }

        #[allow(non_snake_case)]
        let glXCreateContextAttribsARB: GlXCreateContextAttribsARB = {
            let addr = get_proc_address("glXCreateContextAttribsARB");
            if addr.is_null() {
                return Err(GlError::CreationFailed);
            } else {
                std::mem::transmute(addr)
            }
        };

        let profile_mask = match config.profile {
            Profile::Core => glx::arb::GLX_CONTEXT_CORE_PROFILE_BIT_ARB,
            Profile::Compatibility => glx::arb::GLX_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
        };

        #[rustfmt::skip]
        let ctx_attribs = [
            glx::arb::GLX_CONTEXT_MAJOR_VERSION_ARB, config.version.0 as i32,
            glx::arb::GLX_CONTEXT_MINOR_VERSION_ARB, config.version.1 as i32,
            glx::arb::GLX_CONTEXT_PROFILE_MASK_ARB, profile_mask,
            0,
        ];

        let context = glXCreateContextAttribsARB(display, *fb_config, std::ptr::null_mut(), 1, ctx_attribs.as_ptr());

        if context.is_null() {
            return Err(GlError::CreationFailed);
        }

        glx::glXMakeCurrent(display, window_handle.window, context);
        glx::glXMakeCurrent(display, 0, std::ptr::null_mut());

        xlib::XSetErrorHandler(prev_callback);

        Ok(Impl {
            window: window_handle.window,
            display,
            context,
        })
    }

    pub unsafe fn make_current(&self) {
        glx::glXMakeCurrent(self.display, self.window, self.context);
    }

    pub unsafe fn make_not_current(&self) {
        glx::glXMakeCurrent(self.display, 0, std::ptr::null_mut());
    }

    pub unsafe fn get_proc_address(&self, symbol: &str) -> *const c_void {
        get_proc_address(symbol)
    }

    pub unsafe fn swap_buffers(&self) {
        glx::glXSwapBuffers(self.display, self.window)
    }

    pub unsafe fn set_swap_interval(&self, vsync: bool) {
        let addr = get_proc_address("glXSwapIntervalEXT");
        let f: GlXSwapIntervalEXT = std::mem::transmute(addr);
        f(self.display, self.window, vsync as i32);
    }
}

impl Drop for Impl {
    fn drop(&mut self) {}
}
