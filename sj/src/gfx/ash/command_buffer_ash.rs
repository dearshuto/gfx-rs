use ash::version::DeviceV1_0;

use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::{
    Buffer, BufferTextureCopyRegion, ColorTargetView, DepthStencilView, Device, GpuAddress,
    IndexFormat, Pipeline, PipelineStageBit, PrimitiveTopology, Shader, ShaderStage, Texture,
    TextureState, TextureSubresourceRange,
};

use super::command_builder::{
    ClearColorCommandBuilder, Command, CopyImageToBufferCommandBuilder, DispatchParams,
    DrawCommandBuilder, EndRenderPassCommandBuilder, SetPipelineParams,
    SetRenderTargetsCommandBuilder, SetUnorderedAccessBufferParams, SetVertexBufferCommandBuilder,
    SetViewportScissorStateCommandBuilder,
};

pub struct CommandBufferImpl<'a> {
    _device: &'a Device,
    _command_pool: ash::vk::CommandPool,
    _command_buffers: Vec<ash::vk::CommandBuffer>,
    _descriptor_pool: ash::vk::DescriptorPool,
    _commands: Vec<Command<'a>>,
    _current_shader: Option<&'a Shader<'a>>,
    _current_descriptor_set: Option<ash::vk::DescriptorSet>,
    _current_render_pass: Option<ash::vk::RenderPass>,
}

impl<'a> CommandBufferImpl<'a> {
    pub fn get_command_buffers(&self) -> &Vec<ash::vk::CommandBuffer> {
        ash::vk::BufferUsageFlags::empty();
        &self._command_buffers
    }

    pub fn get_command_count(&self) -> i32 {
        self._commands.len() as i32
    }

    pub fn get_descriptor_pool(&self) -> &ash::vk::DescriptorPool {
        &self._descriptor_pool
    }

    fn is_render_pass_begining(&self) -> bool {
        self._current_render_pass.is_some()
    }

    fn push_end_render_pass_command(&mut self) {
        let command_buffer = self._command_buffers.iter().next().unwrap();
        let builder = EndRenderPassCommandBuilder::new(self._device, *command_buffer);
        let command = Command::EndRenderTargets(builder);
        self._commands.push(command);
        self._current_render_pass = None;
    }
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferImpl<'a> {
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self {
        let device_impl = device.to_data().get_device();
        unsafe {
            let command_pool = device_impl
                .create_command_pool(
                    &ash::vk::CommandPoolCreateInfo::builder()
                        .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
                        .queue_family_index(device.to_data().get_queue_family_index())
                        .build(),
                    None,
                )
                .expect("command pool");
            let command_buffers = device_impl
                .allocate_command_buffers(
                    &ash::vk::CommandBufferAllocateInfo::builder()
                        .command_buffer_count(1)
                        .command_pool(command_pool)
                        .level(ash::vk::CommandBufferLevel::PRIMARY)
                        .build(),
                )
                .unwrap();

            let descriptor_pool_sizes = [
                ash::vk::DescriptorPoolSize {
                    ty: ash::vk::DescriptorType::UNIFORM_BUFFER,
                    descriptor_count: 64,
                },
                ash::vk::DescriptorPoolSize {
                    ty: ash::vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: 64,
                },
            ];
            let descriptor_pool = device_impl
                .create_descriptor_pool(
                    &ash::vk::DescriptorPoolCreateInfo::builder()
                        .pool_sizes(&descriptor_pool_sizes)
                        .max_sets(1)
                        .build(),
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _command_pool: command_pool,
                _command_buffers: command_buffers,
                _descriptor_pool: descriptor_pool,
                _commands: Vec::<Command>::new(),
                _current_shader: None,
                _current_descriptor_set: None,
                _current_render_pass: None,
            }
        }
    }

    fn begin(&mut self) {
        let command_buffer = self._command_buffers.iter().next().unwrap();
        let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(ash::vk::CommandBufferUsageFlags::ONE_TIME_SUBMIT)
            .build();
        let device_impl = self._device.to_data().get_device();

        unsafe {
            device_impl
                .begin_command_buffer(*command_buffer, &command_buffer_begin_info)
                .expect("");
        }
    }

    fn end(&mut self) {
        // レンダーパスが存在する場合は終了する
        if self.is_render_pass_begining() {
            self.push_end_render_pass_command();
        }

        // コマンドバッファの構築
        for command in &self._commands {
            command.build();
        }

        unsafe {
            let device_impl = self._device.to_data().get_device();
            let command_buffer = self._command_buffers.iter().next().unwrap();
            device_impl.end_command_buffer(*command_buffer).expect("");
        }
    }

    fn reset(&mut self) {
        self._commands.clear();
    }

    fn set_viewport_scissor_state(
        &mut self,
        viewport_scissor_state: &'a crate::gfx::ViewportScissorState,
    ) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let builder = SetViewportScissorStateCommandBuilder::new(
            self._device,
            viewport_scissor_state,
            *command_buffer_ash,
        );
        let command = Command::SetViewportScissorState(builder);
        self._commands.push(command);
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let render_pass = if pipeline.to_data().is_graphics_pipeline() {
            Some(*self._current_render_pass.as_ref().unwrap())
        } else {
            None
        };

        if pipeline.to_data().is_graphics_pipeline() {}
        let set_pipelie_params = SetPipelineParams::new(
            self._device,
            pipeline,
            *command_buffer_ash,
            self._descriptor_pool,
            render_pass,
        );

        self._current_shader = Some(pipeline.to_data().get_shader());
        self._current_descriptor_set = Some(*set_pipelie_params.get_descriptor_set());

        let command = Command::SetPipeline(set_pipelie_params);
        self._commands.push(command);
    }

    // 使ってないコード
    fn set_buffer(&mut self, _buffer: &'a Buffer<'a>) {}

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        _stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        let buffer_ash = gpu_address.to_data().get_buffer().get_buffer();
        let offset = gpu_address.to_data().get_offset() as u64;
        let params = SetUnorderedAccessBufferParams::new(
            self._device,
            self._current_descriptor_set.unwrap(),
            buffer_ash,
            slot,
            offset,
            size,
        );
        let command = Command::SetUnorderedAccessBuffer(params);
        self._commands.push(command);
    }

    fn clear_color(
        &mut self,
        _color_target_view: &mut ColorTargetView,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let builder = ClearColorCommandBuilder::new(
            self._device,
            *command_buffer_ash,
            red,
            green,
            blue,
            alpha,
        );
        let command = Command::ClearColorCommand(builder);
        self._commands.push(command);
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        if self.is_render_pass_begining() {
            self.push_end_render_pass_command();
        }

        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let builder = SetRenderTargetsCommandBuilder::new(
            self._device,
            *command_buffer_ash,
            color_target_views,
            depth_stencil_state_view,
        );

        // いったん RenderPass は取っておく
        self._current_render_pass = Some(*builder.get_render_pass());

        let command = Command::SetRenderTargets(builder);
        self._commands.push(command);
    }

    fn set_vertex_buffer(&mut self, _buffer_index: i32, gpu_address: &GpuAddress) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let buffer = gpu_address.to_data().get_buffer().get_buffer();

        let params = SetVertexBufferCommandBuilder::new(self._device, *command_buffer_ash, buffer);
        let command = Command::SetVertexBuffer(params);
        self._commands.push(command);
    }

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw_instanced(
            primitive_topology,
            vertex_count,
            vertex_offset,
            1, /*instance count*/
            0, /*base instance*/
        );
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let params = DrawCommandBuilder::new(
            self._device,
            *command_buffer_ash,
            vertex_count as u32,
            instance_count as u32,
            vertex_offset as u32,
            base_instance as u32,
        );
        let command = Command::Draw(params);
        self._commands.push(command);
    }

    fn draw_indexed(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: IndexFormat,
        _gpu_address: &GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
    ) {
        std::unimplemented!();
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
        std::unimplemented!();
    }

    fn draw_indirect(&mut self, _gpu_address: &GpuAddress) {
        std::unimplemented!();
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let descriptor_set = self._current_descriptor_set.unwrap();
        let pipeline_layout = self
            ._current_shader
            .unwrap()
            .to_data()
            .get_pipeline_layout();
        let params = DispatchParams::new(
            self._device,
            *command_buffer_ash,
            *pipeline_layout,
            descriptor_set,
            group_count_x,
            group_count_y,
            group_count_z,
        );
        let command = Command::Dispatch(params);
        self._commands.push(command);
    }

    fn set_texture_state_transition(
        &mut self,
        _texture: &Texture,
        _range: TextureSubresourceRange,
        _old_state: TextureState,
        _old_stage_bit: PipelineStageBit,
        _new_state: TextureState,
        _new_stage_bit: PipelineStageBit,
    ) {
    }

    fn copy_image_to_buffer(
        &mut self,
        dst_buffer: &mut Buffer,
        src_texture: &Texture,
        copy_region: &BufferTextureCopyRegion,
    ) {
        // イメージのコピーはレンダーパス外じゃないとできない
        // TODO: レンダーターゲットの設定を復元する？
        if self.is_render_pass_begining() {
            self.push_end_render_pass_command();
        }

        let command_buffer_ash = self._command_buffers.iter().next().unwrap();
        let builder = CopyImageToBufferCommandBuilder::new(
            self._device,
            *command_buffer_ash,
            dst_buffer,
            src_texture,
            copy_region,
        );
        let command = Command::CopyImageToBuffer(builder);
        self._commands.push(command);
    }
}

impl<'a> Drop for CommandBufferImpl<'a> {
    fn drop(&mut self) {
        // 各コマンド内で Vulkan オブジェクトを生成している場合もあるのでちゃんと破棄しておく
        self._commands.clear();

        let device_impl = self._device.to_data().get_device();
        unsafe {
            device_impl.destroy_descriptor_pool(self._descriptor_pool, None);
            device_impl.free_command_buffers(self._command_pool, &self._command_buffers);
            device_impl.destroy_command_pool(self._command_pool, None);
        }
    }
}
