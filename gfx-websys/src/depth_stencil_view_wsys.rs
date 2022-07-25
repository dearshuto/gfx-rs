use sjgfx_interface::IDepthStencilView;

use crate::{DeviceWsys, TextureWsys};

pub struct DepthStencilViewWsys;

impl IDepthStencilView for DepthStencilViewWsys {
    type DeviceType = DeviceWsys;
    type TextureType = TextureWsys;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::DepthStencilStateInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        todo!()
    }
}
