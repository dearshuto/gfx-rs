use crate::IDevice;

pub struct SemaphoreInfo;
impl SemaphoreInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISemaphore {
    type DeviceType: IDevice;

    fn new(device: &Self::DeviceType, info: &SemaphoreInfo) -> Self;
}
