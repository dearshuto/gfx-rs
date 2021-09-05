use crate::gfx::blend_state_api::IBlendStateImpl;
use crate::gfx::{BlendTargetStateInfo, Device};

pub struct BlendStateVk {
    _blend_target_state_info_array: Vec<BlendTargetStateInfo>,
}

impl IBlendStateImpl for BlendStateVk {
    fn new(_device: &Device, info: &crate::gfx::BlendStateInfo) -> Self {
        Self {
            _blend_target_state_info_array: info.get_target_state_info().to_vec(),
        }
    }
}
