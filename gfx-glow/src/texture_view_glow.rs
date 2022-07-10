use sjgfx_interface::ITextureView;

use crate::{DeviceGlow, TextureGlow};

pub struct TextureViewGlow {
    texture: glow::Texture,
}

impl TextureViewGlow {
    pub fn get_handle(&self) -> glow::Texture {
        self.texture
    }
}

impl ITextureView for TextureViewGlow {
    type DeviceType = DeviceGlow;
    type TextureType = TextureGlow;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::TextureViewInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        todo!()
    }
}
