use crate::gfx::{GpuAddress, ShaderStage};

use super::VkAutoCommandBufferBuilder;

pub struct SetUnorderedAccessBufferCommandBuilder {}

impl SetUnorderedAccessBufferCommandBuilder {
    pub fn new(_slot: i32, _stage: ShaderStage, _gpu_address: &GpuAddress, _size: u64) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
