pub mod compute_pass_command_builder;
pub mod graphics_pass_command_builder;

use self::compute_pass_command_builder::ComputePassCommandBuilder;
use self::graphics_pass_command_builder::GraphicsPassCommandBuilder;
use crate::gfx::{
    ColorTargetView, DepthStencilView, GpuAddress, IndexFormat, PrimitiveTopology, ShaderStage,
    ViewportScissorState,
};

pub enum CommandBuilder<'a> {
    Graphics(GraphicsPassCommandBuilder<'a>),
    Compute(ComputePassCommandBuilder<'a>),
}

impl<'a> CommandBuilder<'a> {
    pub fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {
        match &self {
            Self::Graphics(ref builder) => builder.build(command_encoder),
            Self::Compute(ref builder) => builder.build(command_encoder),
        }
    }

    pub fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_viewport_scissor_state(viewport_scissor_state)
            }
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: GpuAddress<'a>,
        size: usize,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_constant_buffer(slot, stage, gpu_address, size)
            }
            Self::Compute(ref mut builder) => {
                builder.set_constant_buffer(slot, stage, gpu_address, size)
            }
        }
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_unordered_access_buffer(slot, stage, gpu_address, size)
            }
            Self::Compute(ref mut builder) => {
                builder.set_unordered_access_buffer(slot, stage, gpu_address, size)
            }
        }
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.set_render_targets(color_target_views, depth_stencil_state_view)
            }
            Self::Compute(ref mut _builder) => panic!(),
        }
    }

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        match self {
            Self::Graphics(ref mut builder) => {} // builder.set_vertex_buffer(buffer_index, gpu_address),
            Self::Compute(ref mut builder) => builder.set_vertex_buffer(buffer_index, gpu_address),
        }
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => {
                builder.draw(primitive_topology, vertex_count, vertex_offset)
            }
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_instanced(
                primitive_topology,
                vertex_count,
                vertex_offset,
                instance_count,
                base_instance,
            ),
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_indexed(
                primitive_topology,
                index_format,
                gpu_address,
                index_count,
                base_vertex,
            ),
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_indexed_instanced(
                primitive_topology,
                index_format,
                gpu_address,
                index_count,
                base_vertex,
                instance_count,
                base_instance,
            ),
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn draw_indirect(&mut self, gpu_address: &GpuAddress) {
        match self {
            Self::Graphics(ref mut builder) => builder.draw_indirect(gpu_address),
            Self::Compute(ref _builder) => panic!(),
        }
    }

    pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        match self {
            Self::Graphics(ref mut _builder) => panic!(),
            Self::Compute(ref mut builder) => {
                builder.dispatch(group_count_x, group_count_y, group_count_z)
            }
        }
    }
}
