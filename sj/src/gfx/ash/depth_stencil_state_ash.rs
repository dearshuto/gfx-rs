use super::super::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};
use super::super::Device;

pub struct DepthStencilStateImpl {
    _depth_stencl_state_create_info: ash::vk::PipelineDepthStencilStateCreateInfo,
}

impl IDepthStencilStateImpl for DepthStencilStateImpl {
    fn new(_device: &Device, info: &DepthStencilStateInfo) -> Self {
        Self {
            _depth_stencl_state_create_info: info.as_ash(),
        }
    }
}

impl DepthStencilStateInfo {
    pub fn as_ash(&self) -> ash::vk::PipelineDepthStencilStateCreateInfo {
        let noop_stencil_state = ash::vk::StencilOpState {
            fail_op: ash::vk::StencilOp::KEEP,
            pass_op: ash::vk::StencilOp::KEEP,
            depth_fail_op: ash::vk::StencilOp::KEEP,
            compare_op: ash::vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        ash::vk::PipelineDepthStencilStateCreateInfo::builder()
            .depth_test_enable(self.is_depth_test_enabled())
            .depth_write_enable(self.is_depth_write_enabled())
            .depth_compare_op(ash::vk::CompareOp::LESS)
            .front(noop_stencil_state)
            .back(noop_stencil_state)
            .max_depth_bounds(1.0)
            .build()
    }
}
