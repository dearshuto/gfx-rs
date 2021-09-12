use crate::gfx::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::{
    Buffer, ColorTargetView, DepthStencilView, Device, GpuAddress, IndexFormat, Pipeline,
    PrimitiveTopology, ShaderStage, ViewportScissorState,
};
//use super::command_builder::compute_pass_command_builder::ComputePassCommandBuilder
//;use super::command_builder::graphics_pass_command_builder::GraphicsPassCommandBuilder;

use crate::gfx::common::command_builder::CommandBuilder;
use super::command_builder::GraphicsPassCommandBuilder;
use super::command_builder::ComputePassCommandBuilder;

pub struct CommandBuffer<'a> {
    _device: &'a Device,
    _commands: Vec<CommandBuilder<'a, GraphicsPassCommandBuilder<'a>, ComputePassCommandBuilder<'a>>>,
}

impl<'a> ICommandBufferImpl<'a> for CommandBuffer<'a> {
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self {
        CommandBuffer {
            _device: device,
            _commands: Vec::new(),
        }
    }

    fn begin(&mut self) {}

    fn end(&mut self) {}

    fn reset(&mut self) {
        self._commands.clear();
    }

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        self._commands
            .last_mut()
            .unwrap()
            .set_viewport_scissor_state(viewport_scissor_state);
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        if pipeline.to_data().is_compute() {
            let builder = ComputePassCommandBuilder::new(self._device, pipeline);
            let command = CommandBuilder::Compute(builder);
            self._commands.push(command);
        } else {
            let builder = GraphicsPassCommandBuilder::new(self._device, pipeline);
            let command = CommandBuilder::Graphics(builder);
            self._commands.push(command);
        }
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: usize,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .set_constant_buffer(slot, stage, gpu_address, size);
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: & GpuAddress,
        size: u64,
    ) {
        // self._commands
        //     .last_mut()
        //     .unwrap()
        //     .set_unordered_access_buffer(slot, stage, gpu_address, size);
    }

    fn clear_color(
        &mut self,
        _color_target_view: &mut ColorTargetView,
        _red: f32,
        _green: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        //todo!()
    }

    fn clear_depth_stencil(
        &mut self,
        _depth_stencil: &mut DepthStencilView,
        _depth: f32,
        _stencil: i32,
        _clear_mode: &crate::gfx::DepthStencilClearMode,
        _texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        color_target_views: & [& ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        // self._commands
        //     .last_mut()
        //     .unwrap()
        //     .set_render_targets(color_target_views, depth_stencil_state_view);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        // self._commands
        //     .last_mut()
        //     .unwrap()
        //     .set_vertex_buffer(buffer_index, gpu_address);
    }

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .draw(primitive_topology, vertex_count, vertex_offset);
    }

    fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self._commands.last_mut().unwrap().draw_instanced(
            primitive_topology,
            vertex_count,
            vertex_offset,
            instance_count,
            base_instance,
        );
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        self._commands.last_mut().unwrap().draw_indexed(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
        );
    }

    fn draw_indexed_instanced(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: IndexFormat,
        _gpu_address: &GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
		todo!();
        // self._commands.last_mut().unwrap().draw_indexed_instanced(
        //     primitive_topology,
        //     index_format,
        //     gpu_address,
        //     index_count,
        //     base_vertex,
        //     instance_count,
        //     base_instance,
        // );
    }

    fn draw_indirect(&mut self, _gpu_address: &GpuAddress) {
		todo!()
        // self._commands
        //     .last_mut()
        //     .unwrap()
        //     .draw_indirect(gpu_address);
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self._commands
            .last_mut()
            .unwrap()
            .dispatch(group_count_x, group_count_y, group_count_z);
    }

    fn set_texture_state_transition(
        &mut self,
        _texture: &crate::gfx::Texture,
        _range: &crate::gfx::TextureSubresourceRange,
        _old_state: crate::gfx::TextureState,
        _old_stage_bit: crate::gfx::PipelineStageBit,
        _new_state: crate::gfx::TextureState,
        _new_stage_bit: crate::gfx::PipelineStageBit,
    ) {
    }

    fn copy_image(
        &mut self,
        _dst_texture: &mut crate::gfx::Texture,
        _dst_subresource: &crate::gfx::TextureSubresource,
        _dst_offset_u: i32,
        _dst_offset_v: i32,
        _dst_offset_w: i32,
        _src_texture: &crate::gfx::Texture,
        _src_copy_range: crate::gfx::TextureCopyRegion,
    ) {
        todo!()
    }

    fn copy_image_to_buffer(
        &mut self,
        _dst_buffer: &mut Buffer,
        _src_texture: &crate::gfx::Texture,
        _copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) {
        //todo!()
    }

    fn flush_memory(&mut self, _gpu_access_flags: crate::gfx::GpuAccess) {}
}

impl<'a> CommandBuffer<'a> {
    pub fn create_command_encoder(&self) -> wgpu::CommandEncoder {
        let command_encoder = self
            ._device
            .to_data()
            .get_device()
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

		for _command in &self._commands {
			
		}
		
        command_encoder
    }
}
