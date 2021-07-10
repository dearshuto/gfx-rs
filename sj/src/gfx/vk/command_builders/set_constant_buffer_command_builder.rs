use super::VkAutoCommandBufferBuilder;
use crate::gfx::{GpuAddress, ShaderStage};

pub struct SetConstnatBufferCommandBuilder {}

impl SetConstnatBufferCommandBuilder {
    pub fn new(_slot: i32, _stage: ShaderStage, _gpu_address: &GpuAddress, _size: usize) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
