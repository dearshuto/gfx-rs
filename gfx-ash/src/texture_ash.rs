use sjgfx_interface::ITexture;

use crate::DeviceAsh;

pub struct TextureAsh;

impl ITexture for TextureAsh {
    type DeviceType = DeviceAsh;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::TextureInfo) -> Self {
        todo!()
    }
}
