use sjgfx_interface::ITextureView;

use crate::{DeviceVk, TextureVk};

pub struct TextureViewVk;

impl ITextureView for TextureViewVk {
    type DeviceType = DeviceVk;
    type TextureType = TextureVk;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::TextureViewInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        todo!()
    }
}
