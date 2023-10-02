use std::mem::size_of;

#[derive(Clone, Copy, Debug)]
pub(crate) struct Buffer {
    pub gl_buf: Option<glow::Buffer>,
    pub buffer_type: BufferType,
    pub size: usize,
    // Dimension of the indices for this buffer,
    // used only as a type argument for glDrawElements and can be
    // 1, 2 or 4
    pub index_type: Option<u32>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BufferType {
    VertexBuffer = glow::ARRAY_BUFFER as _,
    IndexBuffer = glow::ELEMENT_ARRAY_BUFFER as _,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BufferUsage {
    Immutable = glow::STATIC_DRAW as _,
    Dynamic = glow::DYNAMIC_DRAW as _,
    Stream = glow::STREAM_DRAW as _,
}

#[derive(Debug, Clone, Copy)]
pub struct BufferId(pub(crate) usize);

/// A vtable-erased generic argument.
/// Basically, the same thing as `fn f<U>(a: &U)`, but
/// trait-object friendly.
pub struct Arg<'a> {
    pub ptr: *const std::ffi::c_void,
    pub element_size: usize,
    pub size: usize,
    pub is_slice: bool,
    pub(crate) _phantom: std::marker::PhantomData<&'a ()>,
}
impl<'a> Arg<'a> {
    pub fn as_slice<T>(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr.cast(), self.size / size_of::<T>()) }
    }
}

pub enum BufferSource<'a> {
    Slice(Arg<'a>),
    Empty { size: usize, element_size: usize },
}
impl<'a> BufferSource<'a> {
    /// Empty buffer of `size * size_of::<T>` bytes
    ///
    /// Platform specific note, OpenGL:
    /// For VertexBuffer T could be anything, it is only used to calculate total size,
    /// but for IndexBuffers T should be either u8, u16 or u32, other
    /// types are not supported.
    ///
    /// For vertex buffers ff the type is not yet known, only total byte size,
    /// it is OK to use `empty::<u8>(byte_size);`
    pub fn empty<T>(size: usize) -> BufferSource<'a> {
        let element_size = std::mem::size_of::<T>();
        BufferSource::Empty {
            size: size * std::mem::size_of::<T>(),
            element_size,
        }
    }

    pub fn slice<T>(data: &'a [T]) -> BufferSource<'a> {
        BufferSource::Slice(Arg {
            ptr: data.as_ptr() as _,
            size: std::mem::size_of_val(data),
            element_size: std::mem::size_of::<T>(),
            is_slice: true,
            _phantom: std::marker::PhantomData,
        })
    }
}

#[derive(Clone, Debug)]
pub struct BufferLayout {
    pub stride: i32,
    pub step_func: VertexStep,
    pub step_rate: i32,
}

impl Default for BufferLayout {
    fn default() -> BufferLayout {
        BufferLayout {
            stride: 0,
            step_func: VertexStep::PerVertex,
            step_rate: 1,
        }
    }
}

#[derive(Default, Clone, Copy, PartialEq, Debug)]
pub enum VertexStep {
    #[default]
    PerVertex,
    PerInstance,
}
