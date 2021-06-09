use super::super::Device;

use super::super::viewport_scissor_state_api::IViewportScissorState;

pub struct ViewportScissorStateWgpu<'a> {
    _device: &'a Device,
}

impl<'a> IViewportScissorState<'a> for ViewportScissorStateWgpu<'a> {
    fn new(device: &'a crate::gfx::Device, info: &crate::gfx::ViewportScissorStateInfo) -> Self {
        todo!()
    }
}
