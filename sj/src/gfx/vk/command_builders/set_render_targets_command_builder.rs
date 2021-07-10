use crate::gfx::{ColorTargetView, DepthStencilView};

use super::VkAutoCommandBufferBuilder;

pub struct SetRenderTargetsCommandBuilder {}

impl SetRenderTargetsCommandBuilder {
    pub fn new(
        _color_target_views: &[&ColorTargetView],
        _depth_stencil_state_view: Option<&DepthStencilView>,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
