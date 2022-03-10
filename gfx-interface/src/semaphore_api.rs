use crate::IDevice;

pub struct SemaphoreInfo;
impl SemaphoreInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISemaphore<'a> {
    type DeviceType: IDevice;

    fn new(device: &'a Self::DeviceType, info: &SemaphoreInfo) -> Self;
}
