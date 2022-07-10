use std::sync::Arc;

use sjgfx_interface::{ISampler, SamplerInfo};

use crate::DeviceWgpu;

pub struct SamplerWgpu {
    sampler: Arc<wgpu::Sampler>,
}

impl SamplerWgpu {
    pub fn new(device: &DeviceWgpu, _info: &SamplerInfo) -> Self {
        let sampler = device
            .get_device()
            .create_sampler(&wgpu::SamplerDescriptor::default());
        Self {
            sampler: Arc::new(sampler),
        }
    }

    pub fn get_sampler(&self) -> &wgpu::Sampler {
        &self.sampler
    }

    pub fn clone_sampler(&self) -> Arc<wgpu::Sampler> {
        self.sampler.clone()
    }
}

impl ISampler for SamplerWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &mut Self::DeviceType, info: &SamplerInfo) -> Self {
        Self::new(device, info)
    }
}
