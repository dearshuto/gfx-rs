use sjgfx_interface::{ISampler, SamplerInfo};

use crate::DeviceAsh;

pub struct SamplerAsh {
    sampler: ash::vk::Sampler,
}

impl SamplerAsh {
    pub fn new(device: &DeviceAsh, _info: &SamplerInfo) -> Self {
        let create_info = ash::vk::SamplerCreateInfo::builder().build();
        let sampler = unsafe { device.get_device().create_sampler(&create_info, None) }.unwrap();

        Self { sampler }
    }

    pub fn get_sampler(&self) -> ash::vk::Sampler {
        self.sampler.clone()
    }
}

impl ISampler for SamplerAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &mut Self::DeviceType, info: &SamplerInfo) -> Self {
        Self::new(device, info)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, SamplerInfo};

    use crate::{DeviceAsh, SamplerAsh};

    #[test]
    fn new() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _sampler = SamplerAsh::new(&device, &SamplerInfo::new());
    }
}
