use crate::IDevice;

pub struct FenceInfo {}

impl FenceInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IFence {
    type DeviceType: IDevice;

    fn new(device: &Self::DeviceType, info: &FenceInfo) -> Self;
}
