use super::super::blend_state_api::IBlendStateImpl;
use super::super::{BlendStateInfo, Device};

pub struct BlendStateWgpu {
    _blend_descriptor: wgpu::BlendDescriptor,
}

impl IBlendStateImpl for BlendStateWgpu {
    fn new(_device: &Device, _info: &BlendStateInfo) -> Self {
        let blend_descriptor = wgpu::BlendDescriptor {
            src_factor: wgpu::BlendFactor::Zero,
            dst_factor: wgpu::BlendFactor::Zero,
            operation: wgpu::BlendOperation::Add,
        };

        Self {
            _blend_descriptor: blend_descriptor,
        }
    }
}
