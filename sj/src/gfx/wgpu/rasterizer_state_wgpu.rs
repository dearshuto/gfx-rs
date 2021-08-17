use super::super::rasterizer_state_api::{IRasterizerStateImpl, RasterizerStateInfo};
use super::super::Device;

pub struct RasterizerStateWgpu {}

impl IRasterizerStateImpl for RasterizerStateWgpu {
    fn new(_device: &Device, _info: RasterizerStateInfo) -> Self {
        Self {}
    }
}
