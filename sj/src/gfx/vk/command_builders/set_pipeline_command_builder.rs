use super::VkAutoCommandBufferBuilder;
use crate::gfx::Pipeline;

pub struct SetPipelineCommandBuilder {}

impl<'a> SetPipelineCommandBuilder {
    pub fn new(_pipeline: &'a Pipeline<'a>) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
