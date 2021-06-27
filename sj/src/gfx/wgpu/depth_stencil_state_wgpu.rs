use super::super::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};
use super::super::Device;

pub struct DepthStencilStateWgpu {
    _depth_stencil_state_descriptor: wgpu::DepthStencilStateDescriptor,
}

impl IDepthStencilStateImpl for DepthStencilStateWgpu {
    fn new(_device: &Device, info: &DepthStencilStateInfo) -> Self {
        let depth_stencil_state_descriptor = wgpu::DepthStencilStateDescriptor {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: info.is_depth_write_enabled(),
            depth_compare: wgpu::CompareFunction::Less,
            stencil: wgpu::StencilStateDescriptor {
                ..Default::default()
            },
        };

        Self {
            _depth_stencil_state_descriptor: depth_stencil_state_descriptor,
        }
    }
}
