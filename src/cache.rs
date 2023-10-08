use crate::MAX_SHADERSTAGE_IMAGES;
use crate::MAX_VERTEX_ATTRIBUTES;
use crate::{
    pipeline::{BlendState, CullFace, Pipeline, StencilState},
    ColorMask,
};
use glow::HasContext;

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct VertexAttributeInternal {
    pub attr_loc: u32,
    pub size: i32,
    pub type_: u32,
    pub offset: i64,
    pub stride: i32,
    pub buffer_index: usize,
    pub divisor: i32,
}

#[derive(Default, Copy, Clone)]
pub struct CachedAttribute {
    pub attribute: VertexAttributeInternal,
    pub gl_vbuf: Option<glow::Buffer>,
}

pub struct GlCache {
    pub stored_index_buffer: Option<glow::Buffer>,
    pub stored_index_type: Option<u32>,
    pub stored_vertex_buffer: Option<glow::Buffer>,
    pub stored_texture: Option<glow::Texture>,
    pub index_buffer: Option<glow::Buffer>,
    pub index_type: Option<u32>,
    pub vertex_buffer: Option<glow::Buffer>,
    pub textures: [Option<glow::Texture>; MAX_SHADERSTAGE_IMAGES],
    pub cur_pipeline: Option<Pipeline>,
    pub color_blend: Option<BlendState>,
    pub alpha_blend: Option<BlendState>,
    pub stencil: Option<StencilState>,
    pub color_write: ColorMask,
    pub cull_face: CullFace,
    pub attributes: [Option<CachedAttribute>; MAX_VERTEX_ATTRIBUTES],
}

impl GlCache {
    pub fn bind_buffer(&mut self, gl: &glow::Context, target: u32, buffer: Option<glow::Buffer>, index_type: Option<u32>) {
        if target == glow::ARRAY_BUFFER {
            if self.vertex_buffer != buffer {
                self.vertex_buffer = buffer;
                unsafe { gl.bind_buffer(target, buffer) }
            }
        } else {
            if self.index_buffer != buffer {
                self.index_buffer = buffer;
                unsafe { gl.bind_buffer(target, buffer) }
            }
            self.index_type = index_type;
        }
    }

    pub fn store_buffer_binding(&mut self, target: u32) {
        if target == glow::ARRAY_BUFFER {
            self.stored_vertex_buffer = self.vertex_buffer;
        } else {
            self.stored_index_buffer = self.index_buffer;
            self.stored_index_type = self.index_type;
        }
    }

    pub fn restore_buffer_binding(&mut self, gl: &glow::Context, target: u32) {
        if target == glow::ARRAY_BUFFER {
            if self.stored_vertex_buffer.is_some() {
                self.bind_buffer(gl, target, self.stored_vertex_buffer, None);
                self.stored_vertex_buffer = None;
            }
        } else if self.stored_index_buffer.is_some() {
            self.bind_buffer(gl, target, self.stored_index_buffer, self.stored_index_type);
            self.stored_index_buffer = None;
        }
    }

    pub fn bind_texture(&mut self, gl: &glow::Context, slot_index: usize, texture: Option<glow::Texture>) {
        unsafe {
            gl.active_texture(glow::TEXTURE0 + slot_index as u32);
            if self.textures[slot_index] != texture {
                gl.bind_texture(glow::TEXTURE_2D, texture);
                self.textures[slot_index] = texture;
            }
        }
    }

    pub fn store_texture_binding(&mut self, slot_index: usize) {
        self.stored_texture = self.textures[slot_index];
    }

    pub fn restore_texture_binding(&mut self, gl: &glow::Context, slot_index: usize) {
        self.bind_texture(gl, slot_index, self.stored_texture);
    }

    pub fn clear_buffer_bindings(&mut self, gl: &glow::Context) {
        self.bind_buffer(gl, glow::ARRAY_BUFFER, None, None);
        self.vertex_buffer = None;

        self.bind_buffer(gl, glow::ELEMENT_ARRAY_BUFFER, None, None);
        self.index_buffer = None;
    }

    pub fn clear_texture_bindings(&mut self, gl: &glow::Context) {
        for ix in 0..MAX_SHADERSTAGE_IMAGES {
            if self.textures[ix].is_some() {
                self.bind_texture(gl, ix, None);
                self.textures[ix] = None;
            }
        }
    }

    pub fn clear_vertex_attributes(&mut self) {
        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.attributes[attr_index];
            *cached_attr = None;
        }
    }
}
