use crate::IDevice;

pub struct SamplerInfo {}

impl SamplerInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISampler {
    type DeviceType: IDevice;

    fn new(device: &Self::DeviceType, info: &SamplerInfo) -> Self;
}
