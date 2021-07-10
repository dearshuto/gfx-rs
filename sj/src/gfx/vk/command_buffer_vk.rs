use crate::gfx::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use crate::gfx::{Device, Pipeline, ViewportScissorState};

pub struct CommandBufferVk<'a> {
    _device: &'a Device,
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferVk<'a> {
    fn new(device: &'a crate::gfx::Device, info: &CommandBufferInfo) -> Self {
        todo!()
    }

    fn begin(&mut self) {
        todo!()
    }

    fn end(&mut self) {
        todo!()
    }

    fn reset(&mut self) {
        todo!()
    }

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        todo!()
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        todo!()
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    ) {
        todo!()
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        todo!()
    }

    fn clear_color(
        &mut self,
        color_target_view: &mut crate::gfx::ColorTargetView,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
        texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        todo!()
    }

    fn clear_depth_stencil(
        &mut self,
        depth_stencil: &mut crate::gfx::DepthStencilView,
        depth: f32,
        stencil: i32,
        clear_mode: &crate::gfx::DepthStencilClearMode,
        texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    ) {
        todo!()
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        todo!()
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        todo!()
    }

    fn draw_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        todo!()
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        todo!()
    }

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        todo!()
    }

    fn draw_indirect(&mut self, gpu_address: &crate::gfx::GpuAddress) {
        todo!()
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        todo!()
    }

    fn set_texture_state_transition(
        &mut self,
        texture: &crate::gfx::Texture,
        range: &crate::gfx::TextureSubresourceRange,
        old_state: crate::gfx::TextureState,
        old_stage_bit: crate::gfx::PipelineStageBit,
        new_state: crate::gfx::TextureState,
        new_stage_bit: crate::gfx::PipelineStageBit,
    ) {
        todo!()
    }

    fn copy_image(
        &mut self,
        dst_texture: &mut crate::gfx::Texture,
        dst_subresource: &crate::gfx::TextureSubresource,
        dst_offset_u: i32,
        dst_offset_v: i32,
        dst_offset_w: i32,
        src_texture: &crate::gfx::Texture,
        src_copy_range: crate::gfx::TextureCopyRegion,
    ) {
        todo!()
    }

    fn copy_image_to_buffer(
        &mut self,
        dst_buffer: &mut crate::gfx::Buffer,
        src_texture: &crate::gfx::Texture,
        copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) {
        todo!()
    }

    fn flush_memory(&mut self, gpu_access_flags: crate::gfx::GpuAccess) {
        todo!()
    }
}
