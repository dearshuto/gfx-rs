use crate::IDevice;

pub struct FenceInfo {}

impl FenceInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IFence<'a> {
    type DeviceType: IDevice;

    fn new(device: &'a Self::DeviceType, info: &FenceInfo) -> Self;
}
