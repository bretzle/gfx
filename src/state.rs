use crate::{buffer::*, cache::*, color::*, pass::*, pipeline::*, shader::*, texture::*, uniform::*, *};
use glow::HasContext;

pub struct Features {
    pub instancing: bool,
}

pub struct QuadContext {
    pub(crate) gl: glow::Context,
    shaders: Vec<ShaderInternal>,
    pipelines: Vec<PipelineInternal>,
    passes: Vec<RenderPassInternal>,
    buffers: Vec<Buffer>,
    textures: Vec<Texture>,
    default_framebuffer: Option<glow::Framebuffer>,
    pub(crate) cache: GlCache,

    pub(crate) features: Features,
    width: i32,
    height: i32,
}

impl QuadContext {
    pub fn new(gl: glow::Context) -> Self {
        unsafe {
            let default_framebuffer = convert_framebuffer(gl.get_parameter_i32(glow::FRAMEBUFFER_BINDING));
            let vao = gl.create_vertex_array().ok();
            gl.bind_vertex_array(vao);

            let instancing = gl.version().major >= 3;

            Self {
                gl,
                default_framebuffer,
                shaders: vec![],
                pipelines: vec![],
                passes: vec![],
                buffers: vec![],
                textures: vec![],
                features: Features { instancing },
                cache: GlCache {
                    stored_index_buffer: None,
                    stored_index_type: None,
                    stored_vertex_buffer: None,
                    index_buffer: None,
                    index_type: None,
                    vertex_buffer: None,
                    cur_pipeline: None,
                    color_blend: None,
                    alpha_blend: None,
                    stencil: None,
                    color_write: (true, true, true, true),
                    cull_face: CullFace::Nothing,
                    stored_texture: None,
                    textures: [None; MAX_SHADERSTAGE_IMAGES],
                    attributes: [None; MAX_VERTEX_ATTRIBUTES],
                },
                width: 0,
                height: 0,
            }
        }
    }

    unsafe fn set_blend(&mut self, color_blend: Option<BlendState>, alpha_blend: Option<BlendState>) {
        if color_blend.is_none() && alpha_blend.is_some() {
            panic!("AlphaBlend without ColorBlend");
        }
        if self.cache.color_blend == color_blend && self.cache.alpha_blend == alpha_blend {
            return;
        }

        if let Some(color_blend) = color_blend {
            if self.cache.color_blend.is_none() {
                self.gl.enable(glow::BLEND);
            }

            let BlendState {
                equation: eq_rgb,
                sfactor: src_rgb,
                dfactor: dst_rgb,
            } = color_blend;

            if let Some(BlendState {
                equation: eq_alpha,
                sfactor: src_alpha,
                dfactor: dst_alpha,
            }) = alpha_blend
            {
                self.gl
                    .blend_func_separate(src_rgb.into(), dst_rgb.into(), src_alpha.into(), dst_alpha.into());
                self.gl.blend_equation_separate(eq_rgb as _, eq_alpha as _);
            } else {
                self.gl.blend_func(src_rgb.into(), dst_rgb.into());
                self.gl.blend_equation_separate(eq_rgb as _, eq_rgb as _);
            }
        } else if self.cache.color_blend.is_some() {
            self.gl.disable(glow::BLEND);
        }

        self.cache.color_blend = color_blend;
        self.cache.alpha_blend = alpha_blend;
    }

    unsafe fn set_stencil(&mut self, stencil_test: Option<StencilState>) {
        if self.cache.stencil == stencil_test {
            return;
        }

        if let Some(stencil) = stencil_test {
            if self.cache.stencil.is_none() {
                self.gl.enable(glow::STENCIL_TEST);
            }

            let front = &stencil.front;
            self.gl
                .stencil_op_separate(glow::FRONT, front.fail_op as _, front.depth_fail_op as _, front.pass_op as _);
            self.gl
                .stencil_func_separate(glow::FRONT, front.test_func as _, front.test_ref, front.test_mask);
            self.gl.stencil_mask_separate(glow::FRONT, front.write_mask);

            let back = &stencil.back;
            self.gl
                .stencil_op_separate(glow::BACK, back.fail_op as _, back.depth_fail_op as _, back.pass_op as _);
            self.gl
                .stencil_func_separate(glow::BACK, back.test_func as _, back.test_ref, back.test_mask);
            self.gl.stencil_mask_separate(glow::BACK, back.write_mask);
        } else if self.cache.stencil.is_some() {
            self.gl.disable(glow::STENCIL_TEST);
        }

        self.cache.stencil = stencil_test;
    }

    unsafe fn set_cull_face(&mut self, cull_face: CullFace) {
        if self.cache.cull_face == cull_face {
            return;
        }

        match cull_face {
            CullFace::Nothing => self.gl.disable(glow::CULL_FACE),
            CullFace::Front => {
                self.gl.enable(glow::CULL_FACE);
                self.gl.cull_face(glow::FRONT);
            }
            CullFace::Back => {
                self.gl.enable(glow::CULL_FACE);
                self.gl.cull_face(glow::BACK);
            }
        }
        self.cache.cull_face = cull_face;
    }

    unsafe fn set_color_write(&mut self, color_write: ColorMask) {
        if self.cache.color_write == color_write {
            return;
        }
        let (r, g, b, a) = color_write;
        self.gl.color_mask(r, g, b, a);
        self.cache.color_write = color_write;
    }
}

impl QuadContext {
    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
    }

    pub fn new_shader(&mut self, shader: ShaderSource, meta: ShaderMeta) -> Result<ShaderId, ShaderError> {
        let shader = ShaderInternal::new(&self.gl, shader, meta)?;
        self.shaders.push(shader);
        Ok(ShaderId(self.shaders.len() - 1))
    }

    pub fn new_texture(&mut self, access: TextureAccess, bytes: Option<&[u8]>, params: TextureParams) -> TextureId {
        let texture = Texture::new(self, access, bytes, params);
        self.textures.push(texture);
        TextureId(self.textures.len() - 1)
    }

    fn new_texture_from_data_and_format(&mut self, bytes: &[u8], params: TextureParams) -> TextureId {
        self.new_texture(TextureAccess::Static, Some(bytes), params)
    }

    pub fn delete_texture(&mut self, texture: TextureId) {
        let t = &mut self.textures[texture.0];
        unsafe { self.gl.delete_texture(t.raw.take().unwrap()) }
    }

    pub fn texture_size(&self, texture: TextureId) -> (u32, u32) {
        let params = self.texture_params(texture);
        (params.width, params.height)
    }

    pub fn texture_set_filter(&mut self, texture: TextureId, filter: FilterMode) {
        let t = self.textures[texture.0];
        t.set_filter(self, filter);
    }

    pub fn texture_set_wrap(&mut self, texture: TextureId, wrap: TextureWrap) {
        let t = self.textures[texture.0];
        t.set_wrap(self, wrap);
    }

    pub fn texture_resize(&mut self, texture: TextureId, width: u32, height: u32, bytes: Option<&[u8]>) {
        let mut t = self.textures[texture.0];
        t.resize(self, width, height, bytes);
    }

    pub fn texture_read_pixels(&mut self, texture: TextureId, bytes: &mut [u8]) {
        let t = self.textures[texture.0];
        t.read_pixels(self, bytes);
    }

    /// Update whole texture content
    /// bytes should be width * height * 4 size - non rgba8 textures are not supported yet anyway
    pub fn texture_update(&mut self, texture: TextureId, bytes: &[u8]) {
        let (width, height) = self.texture_size(texture);
        self.texture_update_part(texture, 0 as _, 0 as _, width as _, height as _, bytes)
    }

    pub fn texture_update_part(&mut self, texture: TextureId, x_offset: i32, y_offset: i32, width: i32, height: i32, bytes: &[u8]) {
        let t = self.textures[texture.0];
        t.update_texture_part(self, x_offset, y_offset, width, height, bytes);
    }

    pub fn new_texture_from_rgba8(&mut self, width: u16, height: u16, bytes: &[u8]) -> TextureId {
        assert_eq!(width as usize * height as usize * 4, bytes.len());

        self.new_texture_from_data_and_format(
            bytes,
            TextureParams {
                width: width as _,
                height: height as _,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                filter: FilterMode::Nearest,
            },
        )
    }

    pub fn texture_params(&self, texture: TextureId) -> TextureParams {
        let texture = self.textures[texture.0];
        texture.params
    }

    pub fn new_render_pass(&mut self, color_img: TextureId, depth_img: Option<TextureId>) -> RenderPass {
        let pass = RenderPassInternal::new(&self.gl, &self.textures, self.default_framebuffer, color_img, depth_img);
        self.passes.push(pass);
        RenderPass(self.passes.len() - 1)
    }

    pub fn render_pass_texture(&self, pass: RenderPass) -> TextureId {
        self.passes[pass.0].texture
    }

    pub fn delete_render_pass(&mut self, pass: RenderPass) {
        unsafe { self.gl.delete_framebuffer(self.passes[pass.0].gl_fb.take().unwrap()) }

        self.delete_texture(self.passes[pass.0].texture);
        if let Some(depth_texture) = self.passes[pass.0].depth_texture {
            self.delete_texture(depth_texture);
        }
    }

    pub fn new_pipeline(&mut self, buffer_layout: &[BufferLayout], attributes: &[VertexAttribute], shader: ShaderId) -> Pipeline {
        self.new_pipeline_with_params(buffer_layout, attributes, shader, Default::default())
    }

    pub fn new_pipeline_with_params(
        &mut self,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: ShaderId,
        params: PipelineParams,
    ) -> Pipeline {
        let pipeline = PipelineInternal::new(&self.gl, buffer_layout, attributes, shader, self.shaders[shader.0].program, params);
        self.pipelines.push(pipeline);
        Pipeline(self.pipelines.len() - 1)
    }

    pub fn apply_pipeline(&mut self, pipeline: &Pipeline) {
        self.cache.cur_pipeline = Some(*pipeline);

        unsafe {
            let internal = &self.pipelines[pipeline.0];
            let shader = &mut self.shaders[internal.shader.0];

            self.gl.use_program(Some(shader.program));
            self.gl.enable(glow::SCISSOR_TEST);

            if internal.params.depth_write {
                self.gl.enable(glow::DEPTH_TEST);
                self.gl.depth_func(internal.params.depth_test as u32)
            } else {
                self.gl.disable(glow::DEPTH_TEST);
            }

            match internal.params.front_face_order {
                FrontFaceOrder::Clockwise => self.gl.front_face(glow::CW),
                FrontFaceOrder::CounterClockwise => self.gl.front_face(glow::CCW),
            }

            self.set_cull_face(self.pipelines[pipeline.0].params.cull_face);
            self.set_blend(
                self.pipelines[pipeline.0].params.color_blend,
                self.pipelines[pipeline.0].params.alpha_blend,
            );

            self.set_stencil(self.pipelines[pipeline.0].params.stencil_test);
            self.set_color_write(self.pipelines[pipeline.0].params.color_write);
        }
    }

    pub fn new_buffer(&mut self, type_: BufferType, usage: BufferUsage, data: BufferSource) -> BufferId {
        let gl_target = type_ as u32;
        let gl_usage = usage as u32;
        let (size, element_size) = match &data {
            BufferSource::Slice(data) => (data.size, data.element_size),
            BufferSource::Empty { size, element_size } => (*size, *element_size),
        };
        let index_type = match type_ {
            BufferType::IndexBuffer if element_size == 1 || element_size == 2 || element_size == 4 => Some(element_size as u32),
            BufferType::IndexBuffer => panic!("unsupported index buffer dimension"),
            BufferType::VertexBuffer => None,
        };
        let gl_buf;

        unsafe {
            gl_buf = self.gl.create_buffer().ok();
            self.cache.store_buffer_binding(gl_target);
            self.cache.bind_buffer(&self.gl, gl_target, gl_buf, index_type);

            self.gl.buffer_data_size(gl_target, size as _, gl_usage);
            if let BufferSource::Slice(data) = data {
                debug_assert!(data.is_slice);
                self.gl.buffer_sub_data_u8_slice(gl_target, 0, data.as_slice());
            }
            self.cache.restore_buffer_binding(&self.gl, gl_target);
        }

        let buffer = Buffer {
            gl_buf,
            buffer_type: type_,
            size,
            index_type,
        };
        self.buffers.push(buffer);
        BufferId(self.buffers.len() - 1)
    }

    pub fn buffer_update(&mut self, buffer: BufferId, data: BufferSource) {
        let data = match data {
            BufferSource::Slice(data) => data,
            _ => panic!("buffer_update expects BufferSource::slice"),
        };
        debug_assert!(data.is_slice);
        let buffer = &self.buffers[buffer.0];

        if matches!(buffer.buffer_type, BufferType::IndexBuffer) {
            assert!(buffer.index_type.is_some());
            assert!(data.element_size as u32 == buffer.index_type.unwrap());
        };

        let size = data.size;

        assert!(size <= buffer.size);

        let gl_target = buffer.buffer_type as u32;
        self.cache.store_buffer_binding(gl_target);
        self.cache.bind_buffer(&self.gl, gl_target, buffer.gl_buf, buffer.index_type);
        unsafe { self.gl.buffer_sub_data_u8_slice(gl_target, 0, data.as_slice()) };
        self.cache.restore_buffer_binding(&self.gl, gl_target);
    }

    /// Size of buffer in bytes
    pub fn buffer_size(&mut self, buffer: BufferId) -> usize {
        self.buffers[buffer.0].size
    }

    /// Delete GPU buffer, leaving handle unmodified.
    ///
    /// There is no protection against using deleted textures later. However its not an UB in OpenGl and thats why
    /// this function is not marked as unsafe
    pub fn delete_buffer(&mut self, buffer: BufferId) {
        unsafe { self.gl.delete_buffer(self.buffers[buffer.0].gl_buf.take().unwrap()) }
        self.cache.clear_buffer_bindings(&self.gl);
        self.cache.clear_vertex_attributes();
    }

    /// Set a new viewport rectangle.
    /// Should be applied after begin_pass.
    pub fn apply_viewport(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { self.gl.viewport(x, y, w, h) }
    }

    /// Set a new scissor rectangle.
    /// Should be applied after begin_pass.
    pub fn apply_scissor_rect(&mut self, x: i32, y: i32, w: i32, h: i32) {
        unsafe { self.gl.scissor(x, y, w, h) }
    }

    pub fn apply_bindings(&mut self, bindings: &Bindings) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        for (n, shader_image) in shader.images.iter().enumerate() {
            let bindings_image = bindings
                .images
                .get(n)
                .unwrap_or_else(|| panic!("Image count in bindings and shader did not match!"));
            if shader_image.gl_loc.is_some() {
                unsafe {
                    self.cache.bind_texture(&self.gl, n, self.textures[bindings_image.0].raw);
                    self.gl.uniform_1_i32(shader_image.gl_loc.as_ref(), n as i32);
                }
            }
        }

        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];

        for attr_index in 0..MAX_VERTEX_ATTRIBUTES {
            let cached_attr = &mut self.cache.attributes[attr_index];

            let pip_attribute = pip.layout.get(attr_index).copied();

            if let Some(Some(attribute)) = pip_attribute {
                let vb = bindings.vertex_buffers[attribute.buffer_index];
                let vb = self.buffers[vb.0];

                if cached_attr.map_or(true, |cached_attr| {
                    attribute != cached_attr.attribute || cached_attr.gl_vbuf != vb.gl_buf
                }) {
                    self.cache.bind_buffer(&self.gl, glow::ARRAY_BUFFER, vb.gl_buf, vb.index_type);

                    unsafe {
                        self.gl.vertex_attrib_pointer_f32(
                            attr_index as u32,
                            attribute.size,
                            attribute.type_,
                            false,
                            attribute.stride,
                            attribute.offset as i32,
                        );
                        if self.features.instancing {
                            self.gl.vertex_attrib_divisor(attr_index as u32, attribute.divisor as u32);
                        }
                        self.gl.enable_vertex_attrib_array(attr_index as u32);
                    };

                    let cached_attr = &mut self.cache.attributes[attr_index];
                    *cached_attr = Some(CachedAttribute {
                        attribute,
                        gl_vbuf: vb.gl_buf,
                    });
                }
            } else if cached_attr.is_some() {
                unsafe { self.gl.disable_vertex_attrib_array(attr_index as u32) }
                *cached_attr = None;
            }
        }
    }

    pub fn apply_uniforms(&mut self, uniforms: UniformsSource) {
        self.apply_uniforms_from_bytes(uniforms.0.as_slice(), uniforms.0.size)
    }

    #[rustfmt::skip]
    pub fn apply_uniforms_from_bytes(&mut self, uniforms: &[u8], size: usize) {
        let pip = &self.pipelines[self.cache.cur_pipeline.unwrap().0];
        let shader = &self.shaders[pip.shader.0];

        let mut offset = 0;

        for (_, uniform) in shader.uniforms.iter().enumerate() {
            use UniformType::*;

            assert!(
                offset <= size - uniform.uniform_type.size() / 4,
                "Uniforms struct does not match shader uniforms layout"
            );

            unsafe {
                let f = bytemuck::cast_slice(&uniforms[offset..]);
                let i = bytemuck::cast_slice(&uniforms[offset..]);

                if let location @ Some(_) = uniform.gl_loc.as_ref() {
                    match uniform.uniform_type {
                        Float1 => self.gl.uniform_1_f32_slice(location, &f[..1]),
                        Float2 => self.gl.uniform_2_f32_slice(location, &f[..2]),
                        Float3 => self.gl.uniform_3_f32_slice(location, &f[..3]),
                        Float4 => self.gl.uniform_4_f32_slice(location, &f[..4]),
                        Int1 => self.gl.uniform_1_i32_slice(location, &i[..1]),
                        Int2 => self.gl.uniform_2_i32_slice(location, &i[..2]),
                        Int3 => self.gl.uniform_3_i32_slice(location, &i[..3]),
                        Int4 => self.gl.uniform_4_i32_slice(location, &i[..4]),
                        Mat4 => self.gl.uniform_matrix_4_f32_slice(location, false, &f[..16]),
                    }
                }
            }
            offset += uniform.uniform_type.size() * uniform.array_count as usize;
        }
    }

    pub fn clear(&mut self, color: Option<Color>, depth: Option<f32>, stencil: Option<i32>) {
        let mut bits = 0;
        unsafe {
            if let Some(c) = color {
                bits |= glow::COLOR_BUFFER_BIT;
                self.gl
                    .clear_color(c.r as f32 / 255.0, c.g as f32 / 255.0, c.b as f32 / 255.0, c.a as f32 / 255.0)
            }

            if let Some(v) = depth {
                bits |= glow::DEPTH_BUFFER_BIT;
                self.gl.clear_depth_f32(v)
            }

            if let Some(v) = stencil {
                bits |= glow::STENCIL_BUFFER_BIT;
                self.gl.clear_stencil(v)
            }

            if bits != 0 {
                self.gl.clear(bits)
            }
        }
    }

    /// start rendering to the default frame buffer
    pub fn begin_default_pass(&mut self, action: PassAction) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, self.default_framebuffer);
            self.gl.viewport(0, 0, self.width, self.height);
            self.gl.scissor(0, 0, self.width, self.height);
        }
        match action {
            PassAction::Nothing => {}
            PassAction::Clear { color, depth, stencil } => {
                self.clear(color, depth, stencil);
            }
        }
    }

    /// start rendering to an offscreen framebuffer
    pub fn begin_pass(&mut self, pass: RenderPass, action: PassAction) {
        let pass = &self.passes[pass.0];
        let (framebuffer, w, h) = (
            pass.gl_fb,
            self.textures[pass.texture.0].params.width as i32,
            self.textures[pass.texture.0].params.height as i32,
        );

        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, framebuffer);
            self.gl.viewport(0, 0, w, h);
            self.gl.scissor(0, 0, w, h);
        }
        match action {
            PassAction::Nothing => {}
            PassAction::Clear { color, depth, stencil } => {
                self.clear(color, depth, stencil);
            }
        }
    }

    pub fn end_render_pass(&mut self) {
        unsafe {
            self.gl.bind_framebuffer(glow::FRAMEBUFFER, self.default_framebuffer);
            self.cache.bind_buffer(&self.gl, glow::ARRAY_BUFFER, None, None);
            self.cache.bind_buffer(&self.gl, glow::ELEMENT_ARRAY_BUFFER, None, None);
        }
    }

    pub fn commit_frame(&mut self) {
        self.cache.clear_buffer_bindings(&self.gl);
        self.cache.clear_texture_bindings(&self.gl);
    }

    pub fn draw(&self, first: i32, count: i32, instance_count: i32) {
        assert!(self.cache.cur_pipeline.is_some(), "Drawing without any binded pipeline");

        if !self.features.instancing && instance_count != 1 {
            eprintln!("Instanced rendering is not supported by the GPU");
            eprintln!("Ignoring this draw call");
            return;
        }

        unsafe { self.gl.draw_arrays_instanced(glow::TRIANGLES, first, count, instance_count) }
    }
}
