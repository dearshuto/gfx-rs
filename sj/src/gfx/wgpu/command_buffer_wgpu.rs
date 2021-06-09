use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::{
    Buffer, ColorTargetView, DepthStencilView, Device, GpuAddress, IndexFormat, Pipeline,
    PrimitiveTopology, ShaderStage, ViewportScissorState,
};

pub struct CommandBuffer<'a> {
    device: &'a wgpu::Device,
}

impl<'a> ICommandBufferImpl<'a> for CommandBuffer<'a> {
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self {
        CommandBuffer {
            device: device.to_data().get_device(),
        }
    }

    fn begin(&mut self) {
        todo!();
    }

    fn end(&mut self) {
        todo!();
    }

    fn reset(&mut self) {
        todo!();
    }

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        todo!();
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        todo!();
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: usize,
    ) {
        todo!()
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        todo!();
    }

    fn clear_color(
        &mut self,
        color_target_view: &mut ColorTargetView,
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
        depth_stencil: &mut DepthStencilView,
        depth: f32,
        stencil: i32,
        clear_mode: &crate::gfx::DepthStencilClearMode,
        texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        todo!();
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        todo!();
    }

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        todo!();
    }

    fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        todo!();
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        todo!();
    }

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        todo!();
    }

    fn draw_indirect(&mut self, gpu_address: &GpuAddress) {
        todo!();
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        todo!();
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
        dst_buffer: &mut Buffer,
        src_texture: &crate::gfx::Texture,
        copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) {
        todo!()
    }

    fn flush_memory(&mut self, gpu_access_flags: crate::gfx::GpuAccess) {
        todo!()
    }
}

impl<'a> CommandBuffer<'a> {
    pub fn begin(&mut self) {}

    pub fn end(&mut self) {}

    pub fn set_pipeline(&self, pipeline: &'a Pipeline) {
        let a: &super::pipeline_wgpu::Pipeline = pipeline.to_data();
    }

    // pub fn get_command_buffer(&self) -> wgpu::CommandBuffer {
    //     let mut command_encoder = self
    //         .device
    //         .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    //     {
    //         let mut compute_pass =
    //             command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
    //         // for item in &self.commands {
    //         // 	item.push(&mut command_encoder);
    //         // }
    //     }

    //     command_encoder.finish()
    // }
}

trait ICommand {
    fn push(&self, command_encoder: &mut wgpu::CommandEncoder);
}

struct SetPipelineCommand<'a> {
    compute_pipeline: &'a wgpu::ComputePipeline,
}

impl<'a> SetPipelineCommand<'a> {
    fn push(
        &self,
        command_encoder: &mut wgpu::RenderPipeline,
        compute_pass: &'a mut wgpu::ComputePass<'a>,
    ) {
        compute_pass.set_pipeline(self.compute_pipeline);
    }
}
