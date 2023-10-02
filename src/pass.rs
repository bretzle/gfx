use crate::{color::Color, texture::{TextureId, Texture}};
use glow::HasContext;

#[derive(Clone, Copy)]
pub enum PassAction {
    Nothing,
    Clear {
        color: Option<Color>,
        depth: Option<f32>,
        stencil: Option<i32>,
    },
}

impl PassAction {
    pub fn clear_color(c: impl Into<Color>) -> PassAction {
        PassAction::Clear {
            color: Some(c.into()),
            depth: Some(1.),
            stencil: None,
        }
    }
}

impl Default for PassAction {
    fn default() -> PassAction {
        PassAction::Clear {
            color: Some(Color::TRANSPARENT),
            depth: Some(1.),
            stencil: None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RenderPass(pub(super) usize);

pub(crate) struct RenderPassInternal {
    pub gl_fb: Option<glow::Framebuffer>,
    pub texture: TextureId,
    pub depth_texture: Option<TextureId>,
}
impl RenderPassInternal {
    pub fn new(
        gl: &glow::Context,
        textures: &[Texture],
        default_framebuffer: Option<glow::Framebuffer>,
        color_img: TextureId,
        depth_img: Option<TextureId>,
    ) -> Self {
        unsafe {
            let gl_fb = gl.create_framebuffer().ok();
            gl.bind_framebuffer(glow::FRAMEBUFFER, gl_fb);
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                textures[color_img.0].raw,
                0,
            );
            if let Some(depth_img) = depth_img {
                gl.framebuffer_texture_2d(
                    glow::FRAMEBUFFER,
                    glow::DEPTH_ATTACHMENT,
                    glow::TEXTURE_2D,
                    textures[depth_img.0].raw,
                    0,
                );
            }
            gl.bind_framebuffer(glow::FRAMEBUFFER, default_framebuffer);
            Self {
                gl_fb,
                texture: color_img,
                depth_texture: depth_img,
            }
        }
    }
}
