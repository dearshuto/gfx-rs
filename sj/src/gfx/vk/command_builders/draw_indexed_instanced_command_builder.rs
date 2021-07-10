use crate::gfx::{GpuAddress, IndexFormat, PrimitiveTopology};

use super::VkAutoCommandBufferBuilder;

pub struct DrawIndexedInstancedCommandBuilder {}

impl DrawIndexedInstancedCommandBuilder {
    pub fn new(
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) -> Self {
        Self {}
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        command_builder
    }
}
