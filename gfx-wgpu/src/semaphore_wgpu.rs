use sjgfx_interface::ISemaphore;

use crate::DeviceWgpu;

pub struct SemaphoreWgpu;

impl SemaphoreWgpu {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> ISemaphore<'a> for SemaphoreWgpu {
    type DeviceType = DeviceWgpu;

    fn new(_device: &'a Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        SemaphoreWgpu::new()
    }
}
