use crate::gfx::{texture_api::TextureArrayRange, DepthStencilClearMode, DepthStencilView};

use super::VkAutoCommandBufferBuilder;

pub struct ClearDepthStencilCommandBuilder {}

impl ClearDepthStencilCommandBuilder {
    pub fn new(
        _depth_stencil: &mut DepthStencilView,
        _depth: f32,
        _stencil: i32,
        _clear_mode: &DepthStencilClearMode,
        _texture_array_range: Option<&TextureArrayRange>,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
