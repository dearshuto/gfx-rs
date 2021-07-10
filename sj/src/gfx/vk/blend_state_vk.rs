use crate::gfx::blend_state_api::IBlendStateImpl;
use crate::gfx::Device;

pub struct BlendStateVk {
    _blend_state: vulkano::pipeline::blend::Blend,
}

impl IBlendStateImpl for BlendStateVk {
    fn new(device: &Device, info: &crate::gfx::BlendStateInfo) -> Self {
        todo!()
    }
}
