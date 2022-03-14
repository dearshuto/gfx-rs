use crate::{DeviceAsh, TextureAsh};
use sjgfx_interface::IDepthStencilView;

pub struct DepthStencilViewAsh;

impl IDepthStencilView for DepthStencilViewAsh {
    type DeviceType = DeviceAsh;
    type TextureType = TextureAsh;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::DepthStencilStateInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        Self {}
    }
}
