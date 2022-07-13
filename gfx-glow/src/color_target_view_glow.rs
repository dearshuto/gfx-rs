use sjgfx_interface::{ColorTargetViewInfo, IColorTargetView, ImageFormat};

use crate::{DeviceGlow, TextureGlow};

pub struct ColorTargetViewGlow {
    texture: glow::Texture,
    format: ImageFormat,
}

impl ColorTargetViewGlow {
    pub(crate) fn new_direct(texture: glow::Texture, format: ImageFormat) -> Self {
        Self{ texture, format }
    }

    pub fn get_texture(&self) -> glow::Texture {
        self.texture
    }

    pub fn get_format(&self) -> ImageFormat {
        self.format.clone()
    }
}

impl IColorTargetView for ColorTargetViewGlow {
    type DeviceType = DeviceGlow;
    type TextureType = TextureGlow;

    fn new(
        _device: &Self::DeviceType,
        info: &ColorTargetViewInfo,
        texture: &Self::TextureType,
    ) -> Self {
        // OpenGL の仕様だとレンダーターゲット (glow::Framebuffer) はメインループの途中で変えにくい
        // ということでコマンドバッファ内で glow::Framebuffer を作ることにする
        Self {
            texture: texture.get_handle(),
            format: info.get_image_format(),
        }
    }
}
