use crate::gfx::command_buffer_api::IScanBufferViewCommandBuffer;
use crate::gfx::{ScanBufferCommandBuffer, ScanBufferView};

use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::{
    Buffer, ColorTargetView, DepthStencilView, Device, GpuAddress, IndexFormat, Pipeline,
    PrimitiveTopology, ShaderStage, ViewportScissorState,
};
use super::command_builder::compute_pass_command_builder::ComputePassCommandBuilder;
use super::command_builder::graphics_pass_command_builder::{
    DrawCommand, GraphicsPassCommandBuilder,
};
use super::command_builder::CommandBuilder;

pub struct CommandBufferWgpu<'a> {
    _device: &'a Device,
    _commands: Vec<CommandBuilder<'a>>,
}

impl<'a> CommandBufferWgpu<'a> {
    pub fn is_graphics_command(&self, index: usize) -> bool {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(_) => true,
            CommandBuilder::Compute(_) => false,
        }
    }

    pub fn get_command_count(&self) -> u32 {
        self._commands.len() as u32
    }

    pub fn is_render_target_scan_buffer_view(&self) -> bool {
        true // TODO
    }

    pub fn get_render_target(&self, _index: usize) -> &wgpu::TextureView {
        todo!()
    }
    pub fn get_compute_pipeline(&self, index: usize) -> &wgpu::ComputePipeline {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(_) => todo!(),
            CommandBuilder::Compute(builder) => builder.get_pipeline(),
        }
    }

    pub fn get_graphics_pipeline(&self, index: usize) -> &wgpu::RenderPipeline {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(builder) => builder.get_render_pipeline(),
            CommandBuilder::Compute(_) => panic!(),
        }
    }

    pub fn get_bind_group(&self, index: usize) -> &wgpu::BindGroup {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(builder) => builder.get_bind_group(),
            CommandBuilder::Compute(builder) => builder.get_bind_group(),
        }
    }

    pub fn get_vertex_buffer(&self, index: usize) -> &wgpu::Buffer {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(builder) => builder.get_vertex_buffer(),
            CommandBuilder::Compute(_) => todo!(),
        }
    }

    pub fn get_draw_command(&self, index: usize) -> &DrawCommand {
        match &self._commands[index] {
            CommandBuilder::Graphics(builder) => builder.get_command(),
            CommandBuilder::Compute(_) => panic!(),
        }
    }

    pub fn get_dispatch_count(&self, index: usize) -> (u32, u32, u32) {
        match &self._commands[index as usize] {
            CommandBuilder::Graphics(_) => panic!(),
            CommandBuilder::Compute(builder) => builder.get_dispatch_count(),
        }
    }
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferWgpu<'a> {
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self {
        Self {
            _device: device,
            _commands: Vec::new(),
        }
    }

    fn begin(&mut self) {}

    fn end(&mut self) {
        self._commands.last_mut().unwrap().build();
    }

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
            let builder = ComputePassCommandBuilder::new(self._device);
            let mut command = CommandBuilder::Compute(builder);
            command.set_pipeline(pipeline);
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
        gpu_address: GpuAddress<'a>,
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
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .set_unordered_access_buffer(slot, stage, gpu_address, size);
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
        todo!()
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

    fn set_scan_buffer_view_as_render_target(&mut self, view: ScanBufferView) {
        self._commands
            .last_mut()
            .unwrap()
            .set_scan_buffer_view_as_render_target(&view);
    }

    fn set_scan_buffer_view(
        mut self,
        scan_buffer_view: ScanBufferView,
    ) -> ScanBufferCommandBuffer<'a> {
        self._commands
            .last_mut()
            .unwrap()
            .set_scan_buffer_view_as_render_target(&scan_buffer_view);
        let instance = ScanBufferCommandBufferWgpu::new_internal(
            scan_buffer_view.move_data().move_frame(),
            self,
        );
        ScanBufferCommandBuffer::new(instance)
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&'a ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .set_render_targets(color_target_views, depth_stencil_state_view);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: GpuAddress<'a>) {
        self._commands
            .last_mut()
            .unwrap()
            .set_vertex_buffer(buffer_index, gpu_address);
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
        gpu_address: GpuAddress<'a>,
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
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: GpuAddress<'a>,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self._commands.last_mut().unwrap().draw_indexed_instanced(
            primitive_topology,
            index_format,
            gpu_address,
            index_count,
            base_vertex,
            instance_count,
            base_instance,
        );
    }

    fn draw_indirect(&mut self, gpu_address: &GpuAddress) {
        self._commands
            .last_mut()
            .unwrap()
            .draw_indirect(gpu_address);
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
        todo!()
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
        todo!()
    }

    fn flush_memory(&mut self, _gpu_access_flags: crate::gfx::GpuAccess) {}
}

pub struct ScanBufferCommandBufferWgpu<'a> {
    _frame: wgpu::SurfaceFrame,
    _command_buffer: CommandBufferWgpu<'a>,
}

impl<'a> ScanBufferCommandBufferWgpu<'a> {
    pub fn new_internal(frame: wgpu::SurfaceFrame, command_buffer: CommandBufferWgpu<'a>) -> Self {
        Self {
            _frame: frame,
            _command_buffer: command_buffer,
        }
    }

    pub fn create_texture_view(&self) -> wgpu::TextureView {
        self._frame
            .output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default())
    }

    pub fn get_graphics_pipeline(&self) -> &wgpu::RenderPipeline {
        self._command_buffer.get_graphics_pipeline(0)
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self._command_buffer.get_bind_group(0)
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        self._command_buffer.get_vertex_buffer(0)
    }

    pub fn get_draw_command(&self) -> &DrawCommand {
        self._command_buffer.get_draw_command(0)
    }
}

impl<'a> IScanBufferViewCommandBuffer<'a> for ScanBufferCommandBufferWgpu<'a> {
    fn end(&mut self) {
        self._command_buffer.end();
    }
}
