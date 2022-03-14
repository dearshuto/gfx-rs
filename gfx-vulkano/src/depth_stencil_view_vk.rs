use sjgfx_interface::IDepthStencilView;

use crate::{DeviceVk, TextureVk};

pub struct DepthStencilViewVk {}

impl IDepthStencilView for DepthStencilViewVk {
    type DeviceType = DeviceVk;
    type TextureType = TextureVk;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::DepthStencilStateInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        Self {}
    }
}
