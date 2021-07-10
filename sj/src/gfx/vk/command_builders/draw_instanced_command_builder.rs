use vulkano::pipeline::vertex::VertexDefinition;

use super::super::common::Data64;
use super::super::common::Float3232;
use crate::gfx::vk::common::IData;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{
    AutoCommandBufferBuilder, CommandBufferUsage, DynamicState, SubpassContents,
};
use vulkano::pipeline::vertex::BufferlessDefinition;
use vulkano::pipeline::vertex::SingleBufferDefinition;
use vulkano::pipeline::GraphicsPipeline;

use super::VkAutoCommandBufferBuilder;

pub struct DrawInstancedCommandBuilder {
    _pipeline: std::sync::Arc<vulkano::pipeline::GraphicsPipeline<BufferlessDefinition>>,
    //_vertex_buffer: std::sync::Arc<vulkano::buffer::BufferView<U>>,
}

impl DrawInstancedCommandBuilder {
    pub fn new(
        pipeline: std::sync::Arc<vulkano::pipeline::GraphicsPipeline<BufferlessDefinition>>,
        //buffer: std::sync::Arc<vulkano::buffer::BufferView<Float3232>>,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) -> Self {
        Self {
            _pipeline: pipeline,
            //            _vertex_buffer: buffer,
        }
    }

    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        let buffer: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Float3232>>;
        let dynamic_state = vulkano::command_buffer::DynamicState::none();
        // let bbb = std::sync::Arc::new(
        //     GraphicsPipeline::start()
        //         .vertex_input_single_buffer()
        //         .build()
        //         .unwrap(),
        // );

        // command_builder
        //     .draw(
        //         self._pipeline.clone(),
        //         &dynamic_state,
        //         BufferlessDefinition {},
        //         (),
        //         (),
        //         vec![],
        //     )
        //     .unwrap();

        command_builder
    }
}
