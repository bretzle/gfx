use super::*;
use wasm_bindgen::JsCast;
use web_sys::*;
use winit::platform::web::WindowExtWebSys;
use winit::window::Window;

pub struct Impl {
    canvas: HtmlCanvasElement,
    gl2_ctx: WebGl2RenderingContext,
}

#[allow(unused)]
impl Impl {
    pub unsafe fn create(config: GlConfig, window: &Window) -> Result<Self, GlError> {
        let canvas = window.canvas().unwrap();

        let gl2_ctx = canvas.get_context("webgl2").expect("Failed to query about WebGL2 context");

        let gl2_ctx = gl2_ctx.unwrap().dyn_into::<WebGl2RenderingContext>().unwrap();

        Ok(Self { canvas, gl2_ctx })
    }

    pub fn get_proc_address(&self, s: &str) -> *const c_void {
        unimplemented!()
    }

    pub fn make_current(&self) {}

    pub fn make_not_current(&self) {}

    pub fn swap_buffers(&self) {}

    pub fn set_swap_interval(&self, _vsync: bool) {}

    pub fn glow(&self) -> glow::Context {
        glow::Context::from_webgl2_context(self.gl2_ctx.clone())
    }
}
