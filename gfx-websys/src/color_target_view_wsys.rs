use sjgfx_interface::IColorTargetView;

use crate::{DeviceWsys, TextureWsys};

pub struct ColorTargetViewWsys {
    
}

impl ColorTargetViewWsys {
    pub fn new_direct() -> Self {
        Self{}
    }
}

impl IColorTargetView for ColorTargetViewWsys {
    type DeviceType = DeviceWsys;
    type TextureType = TextureWsys;

    fn new(
        _device: &Self::DeviceType,
        _info: &sjgfx_interface::ColorTargetViewInfo,
        _texture: &Self::TextureType,
    ) -> Self {
        todo!()
    }
}
