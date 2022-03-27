use sjgfx_interface::ISampler;

use crate::DeviceVk;

pub struct SamplerVk;

impl ISampler for SamplerVk {
    type DeviceType = DeviceVk;

    fn new(_device: &Self::DeviceType, _info: &sjgfx_interface::SamplerInfo) -> Self {
        todo!()
    }
}
