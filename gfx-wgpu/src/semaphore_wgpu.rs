use sjgfx_interface::ISemaphore;

use crate::DeviceWgpu;

pub struct SemaphoreWgpu {}

impl ISemaphore for SemaphoreWgpu {
    type DeviceType = DeviceWgpu;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        Self {}
    }
}
