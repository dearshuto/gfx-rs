use sjgfx_interface::ISemaphore;

use crate::DeviceGlow;

pub struct SemaphoerGlow;

impl ISemaphore for SemaphoerGlow {
    type DeviceType = DeviceGlow;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        Self {}
    }
}
