use std::sync::Arc;

use sjgfx_interface::ISampler;
use web_sys::{WebGlSampler, WebGl2RenderingContext};
use web_sys::WebGl2RenderingContext as GL;

use crate::DeviceWsys;

pub struct SamplerWsys {
    gl: Arc<WebGl2RenderingContext>,
    sampler: WebGlSampler,
}

impl SamplerWsys {
    pub fn clone_sampler(&self) -> WebGlSampler {
        self.sampler.clone()
    }
}

impl ISampler for SamplerWsys {
    type DeviceType = DeviceWsys;

    fn new(device: &mut Self::DeviceType, _info: &sjgfx_interface::SamplerInfo) -> Self {
        let gl = device.clone_context();
        let sampler = gl.create_sampler().unwrap();
        gl.sampler_parameteri(&sampler, GL::TEXTURE_MIN_FILTER, GL::NEAREST as i32);
        gl.sampler_parameteri(&sampler, GL::TEXTURE_MAG_FILTER, GL::NEAREST as i32);
        gl.sampler_parameteri(&sampler, GL::TEXTURE_WRAP_S, GL::CLAMP_TO_EDGE as i32);
        gl.sampler_parameteri(&sampler, GL::TEXTURE_WRAP_T, GL::CLAMP_TO_EDGE as i32);

        Self {
            gl,
            sampler
        }
    }
}

impl Drop for SamplerWsys {
    fn drop(&mut self) {
        self.gl.delete_sampler(Some(&self.sampler));
    }
}
