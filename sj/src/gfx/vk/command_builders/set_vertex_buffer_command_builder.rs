use super::VkAutoCommandBufferBuilder;

pub struct SetVertexBufferCommandBuilder {}

impl SetVertexBufferCommandBuilder {
    pub fn new(_buffer_index: i32, _gpu_address: &crate::gfx::GpuAddress) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
