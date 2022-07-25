use sjgfx_interface::IFence;

use crate::DeviceWsys;

pub struct FenceWsys;

impl IFence for FenceWsys {
    type DeviceType = DeviceWsys;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::FenceInfo) -> Self {
        Self {}
    }
}
