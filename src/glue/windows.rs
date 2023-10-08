#![allow(clippy::all)]

use super::{GlConfig, GlError, Profile};
use raw_window_handle::{HasRawWindowHandle, RawWindowHandle};
use std::{
    ffi::{CString, OsStr},
    mem::transmute,
    os::windows::prelude::OsStrExt,
};
use winapi::shared::minwindef::{HINSTANCE, HMODULE};
use winapi::shared::ntdef::WCHAR;
use winapi::shared::windef::{HDC, HGLRC, HWND};
use winapi::um::libloaderapi::{FreeLibrary, GetProcAddress, LoadLibraryA};
use winapi::um::wingdi::{
    wglCreateContext, wglDeleteContext, wglGetProcAddress, wglMakeCurrent, ChoosePixelFormat, DescribePixelFormat, SetPixelFormat,
    SwapBuffers, PFD_DOUBLEBUFFER, PFD_DRAW_TO_WINDOW, PFD_MAIN_PLANE, PFD_SUPPORT_OPENGL, PFD_TYPE_RGBA, PIXELFORMATDESCRIPTOR,
};
use winapi::um::winnt::IMAGE_DOS_HEADER;
use winapi::um::winuser::{
    CreateWindowExW, DefWindowProcW, DestroyWindow, GetDC, RegisterClassW, ReleaseDC, UnregisterClassW, CS_OWNDC, CW_USEDEFAULT, WNDCLASSW,
};

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_create_context.txt

type WglCreateContextAttribsARB = extern "system" fn(HDC, HGLRC, *const i32) -> HGLRC;

const WGL_CONTEXT_MAJOR_VERSION_ARB: i32 = 0x2091;
const WGL_CONTEXT_MINOR_VERSION_ARB: i32 = 0x2092;
const WGL_CONTEXT_PROFILE_MASK_ARB: i32 = 0x9126;

const WGL_CONTEXT_CORE_PROFILE_BIT_ARB: i32 = 0x00000001;
const WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB: i32 = 0x00000002;
const WGL_CONTEXT_OPENGL_NO_ERROR_ARB: i32 = 0x31B3;

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/WGL_ARB_pixel_format.txt

type WglChoosePixelFormatARB = extern "system" fn(HDC, *const i32, *const f32, u32, *mut i32, *mut u32) -> i32;

const WGL_DRAW_TO_WINDOW_ARB: i32 = 0x2001;
const WGL_ACCELERATION_ARB: i32 = 0x2003;
const WGL_SUPPORT_OPENGL_ARB: i32 = 0x2010;
const WGL_DOUBLE_BUFFER_ARB: i32 = 0x2011;
const WGL_PIXEL_TYPE_ARB: i32 = 0x2013;
const WGL_RED_BITS_ARB: i32 = 0x2015;
const WGL_GREEN_BITS_ARB: i32 = 0x2017;
const WGL_BLUE_BITS_ARB: i32 = 0x2019;
const WGL_ALPHA_BITS_ARB: i32 = 0x201B;
const WGL_DEPTH_BITS_ARB: i32 = 0x2022;
const WGL_STENCIL_BITS_ARB: i32 = 0x2023;

const WGL_FULL_ACCELERATION_ARB: i32 = 0x2027;
const WGL_TYPE_RGBA_ARB: i32 = 0x202B;

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_multisample.txt

const WGL_SAMPLE_BUFFERS_ARB: i32 = 0x2041;
const WGL_SAMPLES_ARB: i32 = 0x2042;

// See https://www.khronos.org/registry/OpenGL/extensions/ARB/ARB_framebuffer_sRGB.txt

const WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB: i32 = 0x20A9;

// See https://www.khronos.org/registry/OpenGL/extensions/EXT/WGL_EXT_swap_control.txt

type WglSwapIntervalEXT = extern "system" fn(i32) -> i32;

extern "C" {
    static __ImageBase: IMAGE_DOS_HEADER;
}

pub struct Impl {
    _hwnd: HWND,
    hdc: HDC,
    hglrc: HGLRC,
    gl_library: HMODULE,
}

impl Impl {
    pub unsafe fn create(config: GlConfig, parent: &impl HasRawWindowHandle) -> Result<Self, GlError> {
        let handle = if let RawWindowHandle::Win32(handle) = parent.raw_window_handle() {
            handle
        } else {
            return Err(GlError::InvalidWindowHandle);
        };

        if handle.hwnd.is_null() {
            return Err(GlError::InvalidWindowHandle);
        }

        // Create temporary window and context to load function pointers
        let class_name = OsStr::new("gl-context-window\0").encode_wide().collect::<Vec<_>>();

        let hinstance = &__ImageBase as *const IMAGE_DOS_HEADER as HINSTANCE;

        let wnd_class = WNDCLASSW {
            style: CS_OWNDC,
            lpfnWndProc: Some(DefWindowProcW),
            hInstance: hinstance,
            lpszClassName: class_name.as_ptr(),
            ..std::mem::zeroed()
        };

        let class = RegisterClassW(&wnd_class);
        if class == 0 {
            return Err(GlError::CreationFailed);
        }

        let hwnd_tmp = CreateWindowExW(
            0,
            class as *const WCHAR,
            [0].as_ptr(),
            0,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            0 as _,
            0 as _,
            hinstance,
            std::ptr::null_mut(),
        );

        if hwnd_tmp.is_null() {
            return Err(GlError::CreationFailed);
        }

        let hdc_tmp = GetDC(hwnd_tmp);

        let pfd_tmp = PIXELFORMATDESCRIPTOR {
            nSize: std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u16,
            nVersion: 1,
            dwFlags: PFD_DRAW_TO_WINDOW | PFD_SUPPORT_OPENGL | PFD_DOUBLEBUFFER,
            iPixelType: PFD_TYPE_RGBA,
            cColorBits: 32,
            cAlphaBits: 8,
            cDepthBits: 24,
            cStencilBits: 8,
            iLayerType: PFD_MAIN_PLANE,
            ..std::mem::zeroed()
        };

        SetPixelFormat(hdc_tmp, ChoosePixelFormat(hdc_tmp, &pfd_tmp), &pfd_tmp);

        let hglrc_tmp = wglCreateContext(hdc_tmp);
        if hglrc_tmp.is_null() {
            ReleaseDC(hwnd_tmp, hdc_tmp);
            UnregisterClassW(class as *const WCHAR, hinstance);
            DestroyWindow(hwnd_tmp);
            return Err(GlError::CreationFailed);
        }

        wglMakeCurrent(hdc_tmp, hglrc_tmp);

        #[allow(non_snake_case)]
        let wglCreateContextAttribsARB: Option<WglCreateContextAttribsARB> = {
            let symbol = CString::new("wglCreateContextAttribsARB").unwrap();
            let addr = wglGetProcAddress(symbol.as_ptr().cast());
            if !addr.is_null() {
                Some(std::mem::transmute(addr))
            } else {
                None
            }
        };

        #[allow(non_snake_case)]
        let wglChoosePixelFormatARB: Option<WglChoosePixelFormatARB> = {
            let symbol = CString::new("wglChoosePixelFormatARB").unwrap();
            let addr = wglGetProcAddress(symbol.as_ptr() as _);
            if !addr.is_null() {
                Some(std::mem::transmute(addr))
            } else {
                None
            }
        };

        wglMakeCurrent(hdc_tmp, 0 as _);
        ReleaseDC(hwnd_tmp, hdc_tmp);
        UnregisterClassW(class as *const WCHAR, hinstance);
        DestroyWindow(hwnd_tmp);

        // Create actual context

        let hwnd = handle.hwnd as HWND;

        let hdc = GetDC(hwnd);

        #[rustfmt::skip]
        let pixel_format_attribs = [
            WGL_DRAW_TO_WINDOW_ARB, 1,
            WGL_ACCELERATION_ARB, WGL_FULL_ACCELERATION_ARB,
            WGL_SUPPORT_OPENGL_ARB, 1,
            WGL_DOUBLE_BUFFER_ARB, config.double_buffer as i32,
            WGL_PIXEL_TYPE_ARB, WGL_TYPE_RGBA_ARB,
            WGL_RED_BITS_ARB, config.red_bits as i32,
            WGL_GREEN_BITS_ARB, config.green_bits as i32,
            WGL_BLUE_BITS_ARB, config.blue_bits as i32,
            WGL_ALPHA_BITS_ARB, config.alpha_bits as i32,
            WGL_DEPTH_BITS_ARB, config.depth_bits as i32,
            WGL_STENCIL_BITS_ARB, config.stencil_bits as i32,
            WGL_SAMPLE_BUFFERS_ARB, config.samples.is_some() as i32,
            WGL_SAMPLES_ARB, config.samples.unwrap_or(0) as i32,
            WGL_FRAMEBUFFER_SRGB_CAPABLE_ARB, config.srgb as i32,
            0,
        ];

        let mut pixel_format = 0;
        let mut num_formats = 0;
        wglChoosePixelFormatARB.unwrap()(
            hdc,
            pixel_format_attribs.as_ptr(),
            std::ptr::null(),
            1,
            &mut pixel_format,
            &mut num_formats,
        );

        let mut pfd: PIXELFORMATDESCRIPTOR = std::mem::zeroed();
        DescribePixelFormat(hdc, pixel_format, std::mem::size_of::<PIXELFORMATDESCRIPTOR>() as u32, &mut pfd);
        SetPixelFormat(hdc, pixel_format, &pfd);

        let profile_mask = match config.profile {
            Profile::Core => WGL_CONTEXT_CORE_PROFILE_BIT_ARB,
            Profile::Compatibility => WGL_CONTEXT_COMPATIBILITY_PROFILE_BIT_ARB,
        };

        #[rustfmt::skip]
        let ctx_attribs = [
            WGL_CONTEXT_MAJOR_VERSION_ARB, config.version.0 as i32,
            WGL_CONTEXT_MINOR_VERSION_ARB, config.version.1 as i32,
            WGL_CONTEXT_PROFILE_MASK_ARB, profile_mask,
            WGL_CONTEXT_OPENGL_NO_ERROR_ARB, 1,
            0
        ];

        let hglrc = wglCreateContextAttribsARB.unwrap()(hdc, 0 as _, ctx_attribs.as_ptr());
        if hglrc.is_null() {
            return Err(GlError::CreationFailed);
        }

        let gl_library_name = CString::new("opengl32.dll").unwrap();
        let gl_library = LoadLibraryA(gl_library_name.as_ptr().cast());

        wglMakeCurrent(hdc, hglrc);
        wglMakeCurrent(hdc, 0 as _);

        Ok(Impl {
            _hwnd: hwnd,
            hdc,
            hglrc,
            gl_library,
        })
    }

    pub unsafe fn make_current(&self) {
        wglMakeCurrent(self.hdc, self.hglrc);
    }

    #[allow(unused)]
    pub unsafe fn make_not_current(&self) {
        wglMakeCurrent(self.hdc, 0 as _);
    }

    pub unsafe fn get_proc_address(&self, symbol: &str) -> *const std::ffi::c_void {
        let symbol = CString::new(symbol).unwrap();
        let addr = wglGetProcAddress(symbol.as_ptr().cast());
        if addr.is_null() {
            let addr = GetProcAddress(self.gl_library, symbol.as_ptr().cast());
            addr.cast()
        } else {
            addr.cast()
        }
    }

    pub unsafe fn swap_buffers(&self) {
        SwapBuffers(self.hdc);
    }

    pub unsafe fn set_swap_interval(&self, vsync: bool) {
        let symbol = CString::new("wglSwapIntervalEXT").unwrap();
        let addr = wglGetProcAddress(symbol.as_ptr() as _);
        if !addr.is_null() {
            let f: WglSwapIntervalEXT = transmute(addr);
            f(vsync as i32);
        }
    }
}
