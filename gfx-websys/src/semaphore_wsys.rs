use sjgfx_interface::ISemaphore;

use crate::DeviceWsys;

pub struct SemaphoreWsys;
impl ISemaphore for SemaphoreWsys {
    type DeviceType = DeviceWsys;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::SemaphoreInfo) -> Self {
        Self {}
    }
}
