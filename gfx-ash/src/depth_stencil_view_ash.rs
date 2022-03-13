use crate::DeviceAsh;
use sjgfx_interface::IDepthStencilView;

pub struct DepthStencilViewAsh;

impl IDepthStencilView for DepthStencilViewAsh {
    type DeviceType = DeviceAsh;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::DepthStencilStateInfo) -> Self {
        Self {}
    }
}
