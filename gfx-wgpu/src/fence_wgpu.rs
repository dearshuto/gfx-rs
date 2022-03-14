use sjgfx_interface::IFence;

use crate::DeviceWgpu;

pub struct FenceWgpu {}

impl IFence for FenceWgpu {
    type DeviceType = DeviceWgpu;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::FenceInfo) -> Self {
        Self{}
    }
}
