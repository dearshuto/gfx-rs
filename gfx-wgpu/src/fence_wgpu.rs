use sjgfx_interface::IFence;

use crate::DeviceWgpu;

pub struct FenceWgpu {}

impl<'a> IFence<'a> for FenceWgpu {
    type DeviceType = DeviceWgpu;

    fn new(_device: &'a Self::DeviceType, _info: &sjgfx_interface::FenceInfo) -> Self {
        FenceWgpu{}
    }
}
