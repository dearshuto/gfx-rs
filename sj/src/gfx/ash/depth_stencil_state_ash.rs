use super::super::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};
use super::super::Device;

pub struct DepthStencilStateImpl {
    _depth_stencl_state_create_info: ash::vk::PipelineDepthStencilStateCreateInfo,
}

impl IDepthStencilStateImpl for DepthStencilStateImpl {
    fn new(device: &Device, info: &DepthStencilStateInfo) -> Self {
        Self {
            _depth_stencl_state_create_info: ash::vk::PipelineDepthStencilStateCreateInfo {
                depth_bounds_test_enable: 1,
                depth_write_enable: 1,
                depth_compare_op: ash::vk::CompareOp::LESS_OR_EQUAL,
                ..Default::default()
            },
        }
    }
}
