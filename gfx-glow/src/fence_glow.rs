use sjgfx_interface::IFence;

use crate::DeviceGlow;

pub struct FenceGlow;

impl IFence for FenceGlow {
    type DeviceType = DeviceGlow;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::FenceInfo) -> Self {
        FenceGlow {}
    }
}
