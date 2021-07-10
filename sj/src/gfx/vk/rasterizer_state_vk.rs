use crate::gfx::rasterizer_state_api::{IRasterizerStateImpl, RasterizerStateInfo};
use crate::gfx::Device;

pub struct RasterizerStateVk {
    _rasterizer_state_info: RasterizerStateInfo,
}

impl IRasterizerStateImpl for RasterizerStateVk {
    fn new(_device: &Device, info: RasterizerStateInfo) -> Self {
        Self {
            _rasterizer_state_info: info,
        }
    }
}
