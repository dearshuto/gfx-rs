use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::ISampler;

use crate::DeviceGlow;

pub struct SamplerGlow {
    gl: Arc<glow::Context>,
    sampler: glow::Sampler,
}

impl SamplerGlow {
    pub fn get_handle(&self) -> glow::Sampler {
        self.sampler
    }
}

impl ISampler for SamplerGlow {
    type DeviceType = DeviceGlow;

    fn new(device: &mut Self::DeviceType, _info: &sjgfx_interface::SamplerInfo) -> Self {
        device.make_current();
        let gl = device.clone_context();
        let sampler = unsafe { gl.create_sampler() }.unwrap();

        Self { gl, sampler }
    }
}

impl Drop for SamplerGlow {
    fn drop(&mut self) {
        unsafe { self.gl.delete_sampler(self.sampler) }
    }
}
