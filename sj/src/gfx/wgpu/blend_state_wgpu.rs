use super::super::blend_state_api::IBlendStateImpl;
use super::super::{BlendStateInfo, Device};

pub struct BlendStateWgpu {}

impl IBlendStateImpl for BlendStateWgpu {
    fn new(_device: &Device, _info: &BlendStateInfo) -> Self {
        Self {}
    }
}
