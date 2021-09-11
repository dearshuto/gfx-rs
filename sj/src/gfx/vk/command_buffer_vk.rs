use vulkano::sync;
use vulkano::sync::GpuFuture;

use crate::gfx::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use crate::gfx::{Device, GpuAccess, Pipeline, ViewportScissorState};

//use crate::gfx::common::command_builder::CommandBuilder;
use super::command_builders::ComputeCommandBuilder;
use super::command_builders::GraphicsCommandBuilder;
type CommandBuilder<'a> = crate::gfx::common::command_builder::CommandBuilder<
    'a,
    GraphicsCommandBuilder<'a>,
    ComputeCommandBuilder,
>;

pub struct CommandBufferVk<'a> {
    _device: &'a Device,
    _commands: Vec<CommandBuilder<'a>>,
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
        let command = if pipeline.to_data().is_graphics() {
            let builder = GraphicsCommandBuilder::new(
                self._device.to_data().get_device_impl().clone(),
                pipeline,
            );
            let command = CommandBuilder::Graphics(builder);
            command
        } else {
            let builder = ComputeCommandBuilder {};
            let command = CommandBuilder::Compute(builder);
            command
        };

        self._commands.push(command);
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
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
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .set_unordered_access_buffer(slot, stage, gpu_address, size);
    }

    fn clear_color(
        &mut self,
        _color_target_view: &mut crate::gfx::ColorTargetView,
        _green: f32,
        _red: f32,
        _blue: f32,
        _alpha: f32,
        _texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        // let builder = ClearColorCommandBuilder::new(
        //     color_target_view,
        //     red,
        //     green,
        //     blue,
        //     alpha,
        //     texture_array_range,
        // );
        // let command = Command::ClearColor(builder);
        // self._commands.push(command);
    }

    fn clear_depth_stencil(
        &mut self,
        _depth_stencil: &mut crate::gfx::DepthStencilView,
        _depth: f32,
        _stencil: i32,
        _clear_mode: &crate::gfx::DepthStencilClearMode,
        _texture_array_range: Option<&crate::gfx::texture_api::TextureArrayRange>,
    ) {
        // let builder = ClearDepthStencilCommandBuilder::new(
        //     depth_stencil,
        //     depth,
        //     stencil,
        //     clear_mode,
        //     texture_array_range,
        // );
        // let command = Command::ClearDepthStencil(builder);
        // self._commands.push(command);
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    ) {
        self._commands
            .last_mut()
            .unwrap()
            .set_render_targets(color_target_views, depth_stencil_state_view);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        self._commands
            .last_mut()
            .unwrap()
            .set_vertex_buffer(buffer_index, gpu_address);
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
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
        primitive_topology: crate::gfx::PrimitiveTopology,
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
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
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
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _index_format: crate::gfx::IndexFormat,
        _gpu_address: &crate::gfx::GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!();
    }

    fn draw_indirect(&mut self, _gpu_address: &crate::gfx::GpuAddress) {
        todo!()
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
        // let builder = CopyImageCommandBuilder::new(
        //     dst_texture,
        //     dst_subresource,
        //     dst_offset_u,
        //     dst_offset_v,
        //     dst_offset_w,
        //     src_texture,
        //     src_copy_range,
        // );
        // let command = Command::CopyImage(builder);
        // self._commands.push(command);
    }

    fn copy_image_to_buffer<TType: 'static>(
        &mut self,
        _dst_buffer: &mut crate::gfx::Buffer<TType>,
        _src_texture: &crate::gfx::Texture,
        _copy_region: &crate::gfx::BufferTextureCopyRegion,
    ) {
        //     let builder = CopyImageToBufferCommandBuilder::new(dst_buffer, src_texture, copy_region);
        //     let command = Command::CopyImageToBuffer(builder);
        //     self._commands.push(command);
    }

    fn flush_memory(&mut self, _gpu_access_flags: GpuAccess) {}
}

impl<'a> CommandBufferVk<'a> {
    pub fn build(&self, queue: std::sync::Arc<vulkano::device::Queue>) {
        let mut builder = vulkano::command_buffer::AutoCommandBufferBuilder::primary(
            self._device.to_data().get_device_impl().clone(),
            self._device.to_data().get_queue().family(),
            vulkano::command_buffer::CommandBufferUsage::OneTimeSubmit,
        )           
			.unwrap();
		
		for command in &self._commands {
			match command {
				CommandBuilder::Graphics(ref command_builder) => command_builder.build(&mut builder),
				CommandBuilder::Compute(ref _command_builder) => panic!(),
				CommandBuilder::Phantom(_) => todo!(),
			};
		}
        let command_buffer = builder.build().unwrap();
		
        let future = vulkano::sync::now(self._device.to_data().get_device_impl().clone())
            .then_execute(queue, command_buffer)
            .unwrap()
            .then_signal_fence_and_flush()
            .unwrap();

        future.wait(None).unwrap();
    }
}
