use super::*;

#[derive(Clone, Debug, Copy, PartialEq)]
pub struct TextureId(pub usize);

#[derive(Clone, Copy, Debug)]
pub(crate) struct Texture {
    pub raw: Option<glow::Texture>,
    pub params: TextureParams,
}

impl Texture {
    pub fn new(ctx: &mut QuadContext, _access: TextureAccess, bytes: Option<&[u8]>, params: TextureParams) -> Texture {
        if let Some(bytes_data) = bytes {
            assert_eq!(params.format.size(params.width, params.height) as usize, bytes_data.len());
        }

        let (internal_format, format, pixel_type) = params.format.into();

        ctx.cache.store_texture_binding(0);

        let texture;

        unsafe {
            texture = ctx.gl.create_texture().ok();
            ctx.cache.bind_texture(&ctx.gl, 0, texture);
            ctx.gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

            if cfg!(not(target_arch = "wasm32")) {
                if params.format == TextureFormat::Alpha {
                    ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_SWIZZLE_A, glow::RED as _);
                } else {
                    ctx.gl
                        .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_SWIZZLE_A, glow::ALPHA as _);
                }
            }

            ctx.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal_format as i32,
                params.width as i32,
                params.height as i32,
                0,
                format,
                pixel_type,
                bytes,
            );

            let wrap = match params.wrap {
                TextureWrap::Repeat => glow::REPEAT,
                TextureWrap::Mirror => glow::MIRRORED_REPEAT,
                TextureWrap::Clamp => glow::CLAMP_TO_EDGE,
            };

            let filter = match params.filter {
                FilterMode::Nearest => glow::NEAREST,
                FilterMode::Linear => glow::LINEAR,
            };

            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap as i32);
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap as i32);
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter as i32);
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter as i32);
        }
        ctx.cache.restore_texture_binding(&ctx.gl, 0);

        Texture { raw: texture, params }
    }

    pub fn set_filter(&self, ctx: &mut QuadContext, filter: FilterMode) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(&ctx.gl, 0, self.raw);

        let filter = match filter {
            FilterMode::Nearest => glow::NEAREST,
            FilterMode::Linear => glow::LINEAR,
        };
        unsafe {
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, filter as i32);
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, filter as i32);
        }
        ctx.cache.restore_texture_binding(&ctx.gl, 0);
    }

    pub fn set_wrap(&self, ctx: &mut QuadContext, wrap: TextureWrap) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(&ctx.gl, 0, self.raw);
        let wrap = match wrap {
            TextureWrap::Repeat => glow::REPEAT,
            TextureWrap::Mirror => glow::MIRRORED_REPEAT,
            TextureWrap::Clamp => glow::CLAMP_TO_EDGE,
        };

        unsafe {
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, wrap as i32);
            ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, wrap as i32);
        }
        ctx.cache.restore_texture_binding(&ctx.gl, 0);
    }

    pub fn resize(&mut self, ctx: &mut QuadContext, width: u32, height: u32, bytes: Option<&[u8]>) {
        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(&ctx.gl, 0, self.raw);

        let (internal_format, format, pixel_type) = self.params.format.into();

        self.params.width = width;
        self.params.height = height;

        unsafe {
            ctx.gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                internal_format as i32,
                self.params.width as i32,
                self.params.height as i32,
                0,
                format,
                pixel_type,
                bytes,
            );
        }

        ctx.cache.restore_texture_binding(&ctx.gl, 0);
    }

    pub fn update_texture_part(&self, ctx: &mut QuadContext, x_offset: i32, y_offset: i32, width: i32, height: i32, bytes: &[u8]) {
        assert_eq!(self.size(width as _, height as _), bytes.len());
        assert!(x_offset + width <= self.params.width as _);
        assert!(y_offset + height <= self.params.height as _);

        ctx.cache.store_texture_binding(0);
        ctx.cache.bind_texture(&ctx.gl, 0, self.raw);

        let (_, format, pixel_type) = self.params.format.into();

        unsafe {
            ctx.gl.pixel_store_i32(glow::UNPACK_ALIGNMENT, 1);

            if cfg!(not(target_arch = "wasm32")) {
                if self.params.format == TextureFormat::Alpha {
                    ctx.gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_SWIZZLE_A, glow::RED as _);
                } else {
                    ctx.gl
                        .tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_SWIZZLE_A, glow::ALPHA as _);
                }
            }

            ctx.gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x_offset as _,
                y_offset as _,
                width as _,
                height as _,
                format,
                pixel_type,
                glow::PixelUnpackData::Slice(bytes),
            );
        }

        ctx.cache.restore_texture_binding(&ctx.gl, 0);
    }

    /// Read texture data into CPU memory
    pub fn read_pixels(&self, ctx: &QuadContext, bytes: &mut [u8]) {
        let (_, format, pixel_type) = self.params.format.into();
        unsafe {
            let binded_fbo = convert_framebuffer(ctx.gl.get_parameter_i32(glow::DRAW_FRAMEBUFFER_BINDING));

            let fbo = ctx.gl.create_framebuffer().ok();
            ctx.gl.bind_framebuffer(glow::FRAMEBUFFER, fbo);
            ctx.gl
                .framebuffer_texture_2d(glow::FRAMEBUFFER, glow::COLOR_ATTACHMENT0, glow::TEXTURE_2D, self.raw, 0);

            ctx.gl.read_pixels(
                0,
                0,
                self.params.width as _,
                self.params.height as _,
                format,
                pixel_type,
                glow::PixelPackData::Slice(bytes),
            );

            ctx.gl.bind_framebuffer(glow::FRAMEBUFFER, binded_fbo);
            ctx.gl.delete_framebuffer(fbo.unwrap());
        }
    }

    #[inline]
    fn size(&self, width: u32, height: u32) -> usize {
        self.params.format.size(width, height) as usize
    }
}

/// List of all the possible formats of input data when uploading to texture.
/// The list is built by intersection of texture formats supported by 3.3 core profile and webgl1.
#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TextureFormat {
    RGB8,
    RGBA8,
    Depth,
    Alpha,
}
impl TextureFormat {
    /// Returns the size in bytes of texture with `dimensions`.
    pub fn size(self, width: u32, height: u32) -> u32 {
        let square = width * height;
        match self {
            TextureFormat::RGB8 => 3 * square,
            TextureFormat::RGBA8 => 4 * square,
            TextureFormat::Depth => 2 * square,
            TextureFormat::Alpha => square,
        }
    }
}

/// Converts from TextureFormat to (internal_format, format, pixel_type)
impl From<TextureFormat> for (u32, u32, u32) {
    fn from(format: TextureFormat) -> Self {
        match format {
            TextureFormat::RGB8 => (glow::RGB, glow::RGB, glow::UNSIGNED_BYTE),
            TextureFormat::RGBA8 => (glow::RGBA, glow::RGBA, glow::UNSIGNED_BYTE),
            TextureFormat::Depth => (glow::DEPTH_COMPONENT, glow::DEPTH_COMPONENT, glow::UNSIGNED_SHORT),
            #[cfg(target_arch = "wasm32")]
            TextureFormat::Alpha => (glow::ALPHA, glow::ALPHA, glow::UNSIGNED_BYTE),
            #[cfg(not(target_arch = "wasm32"))]
            TextureFormat::Alpha => (glow::R8, glow::RED, glow::UNSIGNED_BYTE), // texture updates will swizzle Red -> Alpha to match WASM
        }
    }
}

/// Sets the wrap parameter for texture.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureWrap {
    /// Samples at coord x + 1 map to coord x.
    Repeat,
    /// Samples at coord x + 1 map to coord 1 - x.
    Mirror,
    /// Samples at coord x + 1 map to coord 1.
    Clamp,
}

#[derive(Clone, Copy, Debug, PartialEq, Hash)]
pub enum FilterMode {
    Linear,
    Nearest,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TextureAccess {
    /// Used as read-only from GPU
    Static,
    /// Can be written to from GPU
    RenderTarget,
}

#[derive(Debug, Copy, Clone)]
pub struct TextureParams {
    pub format: TextureFormat,
    pub wrap: TextureWrap,
    pub filter: FilterMode,
    pub width: u32,
    pub height: u32,
}
impl Default for TextureParams {
    fn default() -> Self {
        TextureParams {
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            filter: FilterMode::Linear,
            width: 0,
            height: 0,
        }
    }
}
