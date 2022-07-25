use sjgfx_interface::ITextureView;
use web_sys::WebGlTexture;

use crate::{DeviceWsys, TextureWsys};

pub struct TextureViewWsys {
    texture: WebGlTexture,
}

impl TextureViewWsys {
    pub fn clone_texture(&self) -> WebGlTexture {
        self.texture.clone()
    }
}

impl ITextureView for TextureViewWsys {
    type DeviceType = DeviceWsys;
    type TextureType = TextureWsys;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::TextureViewInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        todo!()
    }
}
