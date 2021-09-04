use super::VkAutoCommandBufferBuilder;
use crate::gfx::ViewportScissorState;

pub struct SetViewportScissorStateBuilder {}

impl<'a> SetViewportScissorStateBuilder {
    pub fn new(_viewport_scissor_state: &'a ViewportScissorState) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
