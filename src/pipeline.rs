use glow::HasContext;

use crate::{shader::ShaderId, cache::VertexAttributeInternal, buffer::{BufferLayout, VertexStep}, ColorMask};


pub(crate) struct PipelineInternal {
    pub layout: Vec<Option<VertexAttributeInternal>>,
    pub shader: ShaderId,
    pub params: PipelineParams,
}
impl PipelineInternal {
    pub(crate) fn new(
        gl: &glow::Context,
        buffer_layout: &[BufferLayout],
        attributes: &[VertexAttribute],
        shader: ShaderId,
        program: glow::Program,
        params: PipelineParams,
    ) -> PipelineInternal {
        #[derive(Clone, Copy, Default)]
        struct BufferCacheData {
            stride: i32,
            offset: i64,
        }

        let mut buffer_cache = vec![BufferCacheData::default(); buffer_layout.len()];

        for VertexAttribute {
            format,
            buffer_index,
            ..
        } in attributes
        {
            let layout = &buffer_layout[*buffer_index];
            let cache = &mut buffer_cache[*buffer_index];

            if layout.stride == 0 {
                cache.stride += format.size_bytes();
            } else {
                cache.stride = layout.stride;
            }
            // WebGL 1 limitation
            assert!(cache.stride <= 255);
        }

        let attributes_len = attributes
            .iter()
            .map(|layout| match layout.format {
                VertexFormat::Mat4 => 4,
                _ => 1,
            })
            .sum();

        let mut vertex_layout = vec![None; attributes_len];

        for VertexAttribute {
            name,
            format,
            buffer_index,
        } in attributes
        {
            let buffer_data = &mut buffer_cache[*buffer_index];
            let layout = &buffer_layout[*buffer_index];

            let attr_loc = unsafe { gl.get_attrib_location(program, name) };
            let divisor = match layout.step_func {
                VertexStep::PerVertex => 0,
                _ => layout.step_rate,
            };

            let mut attributes_count: usize = 1;
            let mut format = *format;

            if format == VertexFormat::Mat4 {
                format = VertexFormat::Float4;
                attributes_count = 4;
            }
            for i in 0..attributes_count {
                if let Some(attr_loc) = attr_loc {
                    let attr_loc = attr_loc + i as u32;

                    let attr = VertexAttributeInternal {
                        attr_loc,
                        size: format.components(),
                        type_: format.type_(),
                        offset: buffer_data.offset,
                        stride: buffer_data.stride,
                        buffer_index: *buffer_index,
                        divisor,
                    };

                    assert!(
                        attr_loc < vertex_layout.len() as u32,
                        "attribute: {name} outside of allocated attributes array len: {}",
                        vertex_layout.len()
                    );
                    vertex_layout[attr_loc as usize] = Some(attr);
                }
                buffer_data.offset += format.size_bytes() as i64
            }
        }

        Self {
            layout: vertex_layout,
            shader,
            params,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct PipelineParams {
    pub cull_face: CullFace,
    pub front_face_order: FrontFaceOrder,
    pub depth_test: Comparison,
    pub depth_write: bool,
    pub depth_write_offset: Option<(f32, f32)>,
    /// Color (RGB) blend function. If None - blending will be disabled for this pipeline.
    /// Usual use case to get alpha-blending:
    ///```
    ///PipelineParams {
    ///    color_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Value(BlendValue::SourceAlpha),
    ///        BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
    ///    ),
    ///    ..Default::default()
    ///};
    ///```
    pub color_blend: Option<BlendState>,
    /// Alpha blend function. If None - alpha will be blended with same equation than RGB colors.
    /// One of possible separate alpha channel blend settings is to avoid blending with WebGl background.
    /// On webgl canvas's resulting alpha channel will be used to blend the whole canvas background.
    /// To avoid modifying only alpha channel, but keep usual transparency:
    ///```
    ///PipelineParams {
    ///    color_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Value(BlendValue::SourceAlpha),
    ///        BlendFactor::OneMinusValue(BlendValue::SourceAlpha))
    ///    ),
    ///    alpha_blend: Some(BlendState::new(
    ///        Equation::Add,
    ///        BlendFactor::Zero,
    ///        BlendFactor::One)
    ///    ),
    ///    ..Default::default()
    ///};
    ///```
    /// The same results may be achieved with ColorMask(true, true, true, false)
    pub alpha_blend: Option<BlendState>,
    pub stencil_test: Option<StencilState>,
    pub color_write: ColorMask,
    pub primitive_type: PrimitiveType,
}

#[derive(Copy, Clone, Debug)]
pub struct Pipeline(pub(crate) usize);

impl Default for PipelineParams {
    fn default() -> PipelineParams {
        PipelineParams {
            cull_face: CullFace::Nothing,
            front_face_order: FrontFaceOrder::CounterClockwise,
            depth_test: Comparison::Always, // no depth test,
            depth_write: false,             // no depth write,
            depth_write_offset: None,
            color_blend: None,
            alpha_blend: None,
            stencil_test: None,
            color_write: (true, true, true, true),
            primitive_type: PrimitiveType::Triangles,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PipelineLayout {
    pub buffers: &'static [BufferLayout],
    pub attributes: &'static [VertexAttribute],
}

#[derive(Clone, Debug)]
pub struct VertexAttribute {
    pub name: &'static str,
    pub format: VertexFormat,
    pub buffer_index: usize,
}

impl VertexAttribute {
    pub const fn new(name: &'static str, format: VertexFormat) -> VertexAttribute {
        Self::with_buffer(name, format, 0)
    }

    pub const fn with_buffer(
        name: &'static str,
        format: VertexFormat,
        buffer_index: usize,
    ) -> VertexAttribute {
        VertexAttribute {
            name,
            format,
            buffer_index,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum VertexFormat {
    /// One 32-bit wide float (equivalent to `f32`)
    Float1,
    /// Two 32-bit wide floats (equivalent to `[f32; 2]`)
    Float2,
    /// Three 32-bit wide floats (equivalent to `[f32; 3]`)
    Float3,
    /// Four 32-bit wide floats (equivalent to `[f32; 4]`)
    Float4,
    /// One unsigned 8-bit integer (equivalent to `u8`)
    Byte1,
    /// Two unsigned 8-bit integers (equivalent to `[u8; 2]`)
    Byte2,
    /// Three unsigned 8-bit integers (equivalent to `[u8; 3]`)
    Byte3,
    /// Four unsigned 8-bit integers (equivalent to `[u8; 4]`)
    Byte4,
    /// One unsigned 16-bit integer (equivalent to `u16`)
    Short1,
    /// Two unsigned 16-bit integers (equivalent to `[u16; 2]`)
    Short2,
    /// Tree unsigned 16-bit integers (equivalent to `[u16; 3]`)
    Short3,
    /// Four unsigned 16-bit integers (equivalent to `[u16; 4]`)
    Short4,
    /// One unsigned 32-bit integers (equivalent to `[u32; 1]`)
    Int1,
    /// Two unsigned 32-bit integers (equivalent to `[u32; 2]`)
    Int2,
    /// Three unsigned 32-bit integers (equivalent to `[u32; 3]`)
    Int3,
    /// Four unsigned 32-bit integers (equivalent to `[u32; 4]`)
    Int4,
    /// Four by four matrix of 32-bit floats
    Mat4,
}

impl VertexFormat {
    /// Number of components in this VertexFormat
    /// it is called size in OpenGl, but do not confuse this with bytes size
    /// basically, its an N from FloatN/IntN
    pub(super) fn components(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 1,
            VertexFormat::Float2 => 2,
            VertexFormat::Float3 => 3,
            VertexFormat::Float4 => 4,
            VertexFormat::Byte1 => 1,
            VertexFormat::Byte2 => 2,
            VertexFormat::Byte3 => 3,
            VertexFormat::Byte4 => 4,
            VertexFormat::Short1 => 1,
            VertexFormat::Short2 => 2,
            VertexFormat::Short3 => 3,
            VertexFormat::Short4 => 4,
            VertexFormat::Int1 => 1,
            VertexFormat::Int2 => 2,
            VertexFormat::Int3 => 3,
            VertexFormat::Int4 => 4,
            VertexFormat::Mat4 => 16,
        }
    }

    /// Size in bytes
    pub(super) fn size_bytes(&self) -> i32 {
        match self {
            VertexFormat::Float1 => 4,
            VertexFormat::Float2 => 2 * 4,
            VertexFormat::Float3 => 3 * 4,
            VertexFormat::Float4 => 4 * 4,
            VertexFormat::Byte1 => 1,
            VertexFormat::Byte2 => 2,
            VertexFormat::Byte3 => 3,
            VertexFormat::Byte4 => 4,
            VertexFormat::Short1 => 2,
            VertexFormat::Short2 => 2 * 2,
            VertexFormat::Short3 => 3 * 2,
            VertexFormat::Short4 => 4 * 2,
            VertexFormat::Int1 => 4,
            VertexFormat::Int2 => 2 * 4,
            VertexFormat::Int3 => 3 * 4,
            VertexFormat::Int4 => 4 * 4,
            VertexFormat::Mat4 => 16 * 4,
        }
    }

    pub(super) fn type_(&self) -> u32 {
        match self {
            VertexFormat::Float1 => glow::FLOAT,
            VertexFormat::Float2 => glow::FLOAT,
            VertexFormat::Float3 => glow::FLOAT,
            VertexFormat::Float4 => glow::FLOAT,
            VertexFormat::Byte1 => glow::UNSIGNED_BYTE,
            VertexFormat::Byte2 => glow::UNSIGNED_BYTE,
            VertexFormat::Byte3 => glow::UNSIGNED_BYTE,
            VertexFormat::Byte4 => glow::UNSIGNED_BYTE,
            VertexFormat::Short1 => glow::UNSIGNED_SHORT,
            VertexFormat::Short2 => glow::UNSIGNED_SHORT,
            VertexFormat::Short3 => glow::UNSIGNED_SHORT,
            VertexFormat::Short4 => glow::UNSIGNED_SHORT,
            VertexFormat::Int1 => glow::UNSIGNED_INT,
            VertexFormat::Int2 => glow::UNSIGNED_INT,
            VertexFormat::Int3 => glow::UNSIGNED_INT,
            VertexFormat::Int4 => glow::UNSIGNED_INT,
            VertexFormat::Mat4 => glow::FLOAT,
        }
    }
}

/// Pixel arithmetic description for blending operations.
/// Will be used in an equation:
/// `equation(sfactor * source_color, dfactor * destination_color)`
/// Where source_color is the new pixel color and destination color is color from the destination buffer.
///
/// Example:
///```
///BlendState::new(
///    Equation::Add,
///    BlendFactor::Value(BlendValue::SourceAlpha),
///    BlendFactor::OneMinusValue(BlendValue::SourceAlpha)
///);
///```
/// This will be `source_color * source_color.a + destination_color * (1 - source_color.a)`
/// Wich is quite common set up for alpha blending.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct BlendState {
    pub(super) equation: Equation,
    pub(super) sfactor: BlendFactor,
    pub(super) dfactor: BlendFactor,
}

impl BlendState {
    pub const fn new(equation: Equation, sfactor: BlendFactor, dfactor: BlendFactor) -> Self {
        Self {
            equation,
            sfactor,
            dfactor,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilState {
    pub front: StencilFaceState,
    pub back: StencilFaceState,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct StencilFaceState {
    /// Operation to use when stencil test fails
    pub fail_op: StencilOp,

    /// Operation to use when stencil test passes, but depth test fails
    pub depth_fail_op: StencilOp,

    /// Operation to use when both stencil and depth test pass,
    /// or when stencil pass and no depth or depth disabled
    pub pass_op: StencilOp,

    /// Used for stencil testing with test_ref and test_mask: if (test_ref & test_mask) *test_func* (*stencil* && test_mask)
    /// Default is Always, which means "always pass"
    pub test_func: CompareFunc,

    /// Default value: 0
    pub test_ref: i32,

    /// Default value: all 1s
    pub test_mask: u32,

    /// Specifies a bit mask to enable or disable writing of individual bits in the stencil planes
    /// Default value: all 1s
    pub write_mask: u32,
}

/// Operations performed on current stencil value when comparison test passes or fails.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum StencilOp {
    /// Default value
    Keep = glow::KEEP as _,
    Zero = glow::ZERO as _,
    Replace = glow::REPLACE as _,
    IncrementClamp = glow::INCR as _,
    DecrementClamp = glow::DECR as _,
    Invert = glow::INVERT as _,
    IncrementWrap = glow::INCR_WRAP as _,
    DecrementWrap = glow::DECR_WRAP as _,
}

/// Depth and stencil compare function
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CompareFunc {
    /// Default value
    Always = glow::ALWAYS as _,
    Never = glow::NEVER as _,
    Less = glow::LESS as _,
    Equal = glow::EQUAL as _,
    LessOrEqual = glow::LEQUAL as _,
    Greater = glow::GREATER as _,
    NotEqual = glow::NOTEQUAL as _,
    GreaterOrEqual = glow::GEQUAL as _,
}

/// Specify whether front- or back-facing polygons can be culled.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum CullFace {
    Nothing,
    Front,
    Back,
}

/// Define front- and back-facing polygons.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FrontFaceOrder {
    Clockwise,
    CounterClockwise,
}

/// A pixel-wise comparison function.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Comparison {
    Never = glow::NEVER as _,
    Less = glow::LESS as _,
    LessOrEqual = glow::LEQUAL as _,
    Greater = glow::GREATER as _,
    GreaterOrEqual = glow::GEQUAL as _,
    Equal = glow::EQUAL as _,
    NotEqual = glow::NOTEQUAL as _,
    Always = glow::ALWAYS as _,
}

/// Specifies how incoming RGBA values (source) and the RGBA in framebuffer (destination)
/// are combined.
#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Equation {
    /// Adds source and destination. Source and destination are multiplied
    /// by blending parameters before addition.
    #[default]
    Add = glow::FUNC_ADD as _,
    /// Subtracts destination from source. Source and destination are
    /// multiplied by blending parameters before subtraction.
    Subtract = glow::FUNC_SUBTRACT as _,
    /// Subtracts source from destination. Source and destination are
    /// multiplied by blending parameters before subtraction.
    ReverseSubtract = glow::FUNC_REVERSE_SUBTRACT as _,
}

/// Blend values.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendValue {
    SourceColor,
    SourceAlpha,
    DestinationColor,
    DestinationAlpha,
}

/// Blend factors.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BlendFactor {
    Zero,
    One,
    Value(BlendValue),
    OneMinusValue(BlendValue),
    SourceAlphaSaturate,
}

impl From<BlendFactor> for u32 {
    fn from(factor: BlendFactor) -> u32 {
        match factor {
            BlendFactor::Zero => glow::ZERO,
            BlendFactor::One => glow::ONE,
            BlendFactor::Value(BlendValue::SourceColor) => glow::SRC_COLOR,
            BlendFactor::Value(BlendValue::SourceAlpha) => glow::SRC_ALPHA,
            BlendFactor::Value(BlendValue::DestinationColor) => glow::DST_COLOR,
            BlendFactor::Value(BlendValue::DestinationAlpha) => glow::DST_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::SourceColor) => glow::ONE_MINUS_SRC_COLOR,
            BlendFactor::OneMinusValue(BlendValue::SourceAlpha) => glow::ONE_MINUS_SRC_ALPHA,
            BlendFactor::OneMinusValue(BlendValue::DestinationColor) => glow::ONE_MINUS_DST_COLOR,
            BlendFactor::OneMinusValue(BlendValue::DestinationAlpha) => glow::ONE_MINUS_DST_ALPHA,
            BlendFactor::SourceAlphaSaturate => glow::SRC_ALPHA_SATURATE,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum PrimitiveType {
    Triangles = glow::TRIANGLES as _,
    Lines = glow::LINES as _,
}
