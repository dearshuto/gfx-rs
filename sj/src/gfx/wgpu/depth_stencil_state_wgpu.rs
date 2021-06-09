use super::super::depth_stencil_state_api::{DepthStencilStateInfo, IDepthStencilStateImpl};
use super::super::Device;

pub struct DepthStencilStateWgpu {}

impl IDepthStencilStateImpl for DepthStencilStateWgpu {
    fn new(device: &Device, info: &DepthStencilStateInfo) -> Self {
        todo!();
    }
}
