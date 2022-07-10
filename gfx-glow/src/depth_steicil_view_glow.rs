use sjgfx_interface::{DepthStencilStateInfo, IDepthStencilView, ImageFormat};

use crate::{DeviceGlow, TextureGlow};

pub struct DepthStencilViewGlow {
    texture: glow::Texture,
}

impl DepthStencilViewGlow {
    pub fn get_texture(&self) -> glow::Texture {
        self.texture
    }

    pub fn get_format(&self) -> ImageFormat {
        ImageFormat::D32
    }
}

impl IDepthStencilView for DepthStencilViewGlow {
    type DeviceType = DeviceGlow;
    type TextureType = TextureGlow;

    fn new(
        _device: &Self::DeviceType,
        _info: &DepthStencilStateInfo,
        texture: &Self::TextureType,
    ) -> Self {
        Self {
            texture: texture.get_handle(),
        }
    }
}
