use crate::gfx::{texture_api::TextureArrayRange, ColorTargetView};

use super::VkAutoCommandBufferBuilder;

pub struct ClearColorCommandBuilder {}

impl ClearColorCommandBuilder {
    pub fn new(
        _color_target_view: &mut ColorTargetView,
        _red: f32,
        _green: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: Option<&TextureArrayRange>,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
