use crate::gfx::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};

pub struct DepthStencilStateVk {
    _depth_stencil_state_info: DepthStencilStateInfo,
}

impl IDepthStencilStateImpl for DepthStencilStateVk {
    fn new(_device: &crate::gfx::Device, info: &DepthStencilStateInfo) -> Self {
        Self {
            _depth_stencil_state_info: info.clone(),
        }
    }
}
