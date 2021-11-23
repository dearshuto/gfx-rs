use super::super::Device;
use crate::gfx::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};

pub struct DepthStencilStateWgpu {
    _depth_stencil_state: wgpu::DepthStencilState,
}

impl DepthStencilStateWgpu {
    pub fn get_depth_stencil_state(&self) -> &wgpu::DepthStencilState {
        &self._depth_stencil_state
    }
}

impl IDepthStencilStateImpl for DepthStencilStateWgpu {
    fn new(_device: &Device, info: &DepthStencilStateInfo) -> Self {
        let depth_stencil_state = wgpu::DepthStencilState {
            format: wgpu::TextureFormat::Depth32Float,
            depth_write_enabled: info.is_depth_write_enabled(),
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        };
        Self {
            _depth_stencil_state: depth_stencil_state,
        }
    }
}
