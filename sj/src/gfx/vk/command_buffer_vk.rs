use vulkano::sync;
use vulkano::sync::GpuFuture;

use super::command_builders::{
    ClearColorCommandBuilder, ClearDepthStencilCommandBuilder, Command, CopyImageCommandBuilder,
    CopyImageToBufferCommandBuilder, DispatchCommandBuilder, DrawIndexedInstancedCommandBuilder,
    DrawInstancedCommandBuilder, SetConstnatBufferCommandBuilder, SetPipelineCommandBuilder,
    SetRenderTargetsCommandBuilder, SetUnorderedAccessBufferCommandBuilder,
    SetVertexBufferCommandBuilder, SetViewportScissorStateBuilder,
};
use crate::gfx::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use crate::gfx::{Device, GpuAccess, Pipeline, ViewportScissorState};

pub struct CommandBufferVk<'a> {
    _device: &'a Device,
    _commands: Vec<Command>,
    _command_buffer: Option<vulkano::command_buffer::PrimaryAutoCommandBuffer>,
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferVk<'a> {
    fn new(device: &'a crate::gfx::Device, _info: &CommandBufferInfo) -> Self {
        Self {
            _device: device,
            _commands: Vec::new(),
            _command_buffer: None,
        }
    }

    fn begin(&mut self) {}

    fn end(&mut self) {
        let device_vk = self._device.to_data().get_device_impl();
        let queue_vk = self._device.to_data().get_queue();

        let mut command_builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            device_vk.clone(),
            queue_vk.family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        for command in &self._commands {
            command_builder = command.build(command_builder);
        }

        self._command_buffer = Some(command_builder.build().unwrap());
    }

    fn reset(&mut self) {
        self._commands.clear();
    }

    fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        let builder = SetViewportScissorStateBuilder::new(viewport_scissor_state);
        let command = Command::SetViewportScissorState(builder);
        self._commands.push(command);
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        let builder = SetPipelineCommandBuilder::new(pipeline);
        let command = Command::SetPipeline(builder);
        self._commands.push(command);
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    ) {
        let builder = SetConstnatBufferCommandBuilder::new(slot, stage, gpu_address, size);
        let command = Command::SetConstantBuffer(builder);
        self._commands.push(command);
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        let builder = SetUnorderedAccessBufferCommandBuilder::new(slot, stage, gpu_address, size);
        let command = Command::SetUnorderedAccessBuffer(builder);
        self._commands.push(command);
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
        let builder = ClearColorCommandBuilder::new(
            color_target_view,
            red,
            green,
            blue,
            alpha,
            texture_array_range,
        );
        let command = Command::ClearColor(builder);
        self._commands.push(command);
    }

    fn clear_depth_stencil(
        &mut self,
        depth_stencil: &mut crate::gfx::DepthStencilView,
        depth: f32,
        stencil: i32,
        clear_mode: &crate::gfx::DepthStencilClearMode,
        texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        let builder = ClearDepthStencilCommandBuilder::new(
            depth_stencil,
            depth,
            stencil,
            clear_mode,
            texture_array_range,
        );
        let command = Command::ClearDepthStencil(builder);
        self._commands.push(command);
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    ) {
        let builder =
            SetRenderTargetsCommandBuilder::new(color_target_views, depth_stencil_state_view);
        let command = Command::SetRenderTargets(builder);
        self._commands.push(command);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        let builder = SetVertexBufferCommandBuilder::new(buffer_index, gpu_address);
        let command = Command::SetVertexBuffer(builder);
        self._commands.push(command);
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw_instanced(primitive_topology, vertex_count, vertex_offset, 1, 0);
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _vertex_count: i32,
        _vertex_offset: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        // let builder = DrawInstancedCommandBuilder::new(
        //     primitive_topology,
        //     vertex_count,
        //     vertex_offset,
        //     instance_count,
        //     base_instance,
        // );
        // let command = Command::DrawInstanced(builder);
        // self._commands.push(command);
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        self.draw_indexed_instanced(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
            1,
            0,
        );
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
        let builder = DrawIndexedInstancedCommandBuilder::new(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
            instance_count,
            base_instance,
        );
        let command = Command::DrawIndexedInstanced(builder);
        self._commands.push(command);
    }

    fn draw_indirect(&mut self, _gpu_address: &crate::gfx::GpuAddress) {
        todo!()
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let builder = DispatchCommandBuilder::new(group_count_x, group_count_y, group_count_z);
        let command = Command::Dispatch(builder);
        self._commands.push(command);
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
        let builder = CopyImageCommandBuilder::new(
            dst_texture,
            dst_subresource,
            dst_offset_u,
            dst_offset_v,
            dst_offset_w,
            src_texture,
            src_copy_range,
        );
        let command = Command::CopyImage(builder);
        self._commands.push(command);
    }

    fn copy_image_to_buffer(
        &mut self,
        dst_buffer: &mut crate::gfx::Buffer,
        src_texture: &crate::gfx::Texture,
        copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) {
        let builder = CopyImageToBufferCommandBuilder::new(dst_buffer, src_texture, copy_region);
        let command = Command::CopyImageToBuffer(builder);
        self._commands.push(command);
    }

    fn flush_memory(&mut self, _gpu_access_flags: GpuAccess) {}
}

impl<'a> CommandBufferVk<'a> {
    pub fn build(&self, queue: std::sync::Arc<vulkano::device::Queue>) {
        let builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            self._device.to_data().get_device_impl().clone(),
            self._device.to_data().get_queue().family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )
        .unwrap();

        let command_buffer = builder.build().unwrap();

        let future = vulkano::sync::now(self._device.to_data().get_device_impl().clone())
            .then_execute(queue, command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();
    }
}
