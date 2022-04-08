use sjgfx_interface::ISemaphore;

use crate::DeviceVk;

pub struct SemaphoreVk;

impl ISemaphore for SemaphoreVk {
    type DeviceType = DeviceVk;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        Self {}
    }
}
