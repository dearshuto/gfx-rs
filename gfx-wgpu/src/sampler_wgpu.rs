use sjgfx_interface::SamplerInfo;

use crate::DeviceWgpu;

pub struct SamplerWgpu{
    sampler: wgpu::Sampler,
}

impl SamplerWgpu {
    pub fn new(device: &DeviceWgpu, _info: &SamplerInfo) -> Self {
        let sampler = device.get_device().create_sampler(&wgpu::SamplerDescriptor::default());
        Self{ sampler }
    }

    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }
}
