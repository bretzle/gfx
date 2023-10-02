#![allow(unused)]

use crate::buffer::BufferId;
use crate::cache::{CachedAttribute, GlCache, VertexAttributeInternal};
use glow::HasContext;
use std::mem::transmute;
use texture::TextureId;

pub mod buffer;
pub mod cache;
pub mod color;
pub mod glue;
pub mod pass;
pub mod pipeline;
pub mod shader;
pub mod state;
pub mod texture;
pub mod uniform;

pub use state::QuadContext;

pub const MAX_VERTEX_ATTRIBUTES: usize = 16;
pub const MAX_SHADERSTAGE_IMAGES: usize = 12;

type ColorMask = (bool, bool, bool, bool);

/// Geometry bindings
#[derive(Clone, Debug)]
pub struct Bindings {
    /// Vertex buffers. Data contained in the buffer must match layout
    /// specified in the `Pipeline`.
    ///
    /// Most commonly vertex buffer will contain `(x,y,z,w)` coordinates of the
    /// vertex in 3d space, as well as `(u,v)` coordinates that map the vertex
    /// to some position in the corresponding `Texture`.
    pub vertex_buffers: Vec<BufferId>,
    /// Textures to be used with when drawing the geometry in the fragment
    /// shader.
    pub images: Vec<TextureId>,
}

pub fn convert_framebuffer(data: i32) -> Option<glow::Framebuffer> {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe {
        transmute(data)
    }

    #[cfg(target_arch = "wasm32")]
    match data {
        0 => None,
        _ => Some(slotmap::KeyData::from_ffi(data as _).into()),
    }
}
