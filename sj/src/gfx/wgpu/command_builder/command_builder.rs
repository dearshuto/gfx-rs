use crate::gfx::{
    ColorTargetView, DepthStencilView, GpuAddress, IndexFormat, Pipeline, PrimitiveTopology,
    ShaderStage, ViewportScissorState,
};

pub trait ICommandBuilder<'a> {
    fn build(&self, command_encoder: &mut wgpu::CommandEncoder);

    fn is_end(&self) -> bool;

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState);

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>);

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: GpuAddress<'a>,
        size: usize,
    );

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    );

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    );

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress);

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    );

    fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    );

    fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    );

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    );

    fn draw_indirect(&mut self, gpu_address: &GpuAddress);

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32);
}
