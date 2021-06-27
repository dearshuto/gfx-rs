use super::super::rasterizer_state_api::{IRasterizerStateImpl, RasterizerStateInfo};
use super::super::Device;

pub struct RasterizerStateWgpu {
    _rasterizer_state_descriptor: wgpu::RasterizationStateDescriptor,
}

impl IRasterizerStateImpl for RasterizerStateWgpu {
    fn new(_device: &Device, _info: RasterizerStateInfo) -> Self {
        let rasterization_state_descriptor = wgpu::RasterizationStateDescriptor {
            front_face: wgpu::FrontFace::Cw,
            cull_mode: wgpu::CullMode::None,
            ..Default::default()
        };

        Self {
            _rasterizer_state_descriptor: rasterization_state_descriptor,
        }
    }
}
