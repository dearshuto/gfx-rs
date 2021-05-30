use ash::version::DeviceV1_0;
use super::super::{Buffer, ColorTargetView, Device, GpuAddress, Pipeline, ShaderStage};
use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::command_buffer_write_descriptor_set_builder::CommandBufferWriteDescriptorSetBuilder;

pub struct CommandBufferImpl<'a> {
    _device: &'a Device,
    _command_pool: ash::vk::CommandPool,
    _command_buffers: Vec<ash::vk::CommandBuffer>,
    _descriptor_sets: Vec<ash::vk::DescriptorSet>,
    _descriptor_pool: ash::vk::DescriptorPool,
    _pipeline_set_command: Option<PipelineSetCommand<'a>>,
    _descriptor_set_builder: CommandBufferWriteDescriptorSetBuilder,
}

impl<'a> CommandBufferImpl<'a> {
    pub fn get_command_buffers(&self) -> &Vec<ash::vk::CommandBuffer> {
        &self._command_buffers
    }

    pub fn get_command_count(&self) -> i32 {
        if self._pipeline_set_command.is_none() {
            0
        } else {
            1
        }
    }

    pub fn get_descriptor_pool(&self) -> &ash::vk::DescriptorPool {
        &self._descriptor_pool
    }

    pub fn get_descriptor_set(&self) -> &ash::vk::DescriptorSet {
        &self._descriptor_sets[0]
    }

    pub fn get_descriptor_set_mut(&mut self) -> &mut ash::vk::DescriptorSet {
        &mut self._descriptor_sets[0]
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

            let descriptor_sets = Vec::new();

            Self {
                _device: device,
                _command_pool: command_pool,
                _command_buffers: command_buffers,
                _descriptor_sets: descriptor_sets,
                _descriptor_pool: descriptor_pool,
                _pipeline_set_command: None,
                _descriptor_set_builder: CommandBufferWriteDescriptorSetBuilder::new(),
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
        let device_impl = self._device.to_data().get_device();
        let command_buffer = self._command_buffers.iter().next().unwrap();

        unsafe {
            device_impl.end_command_buffer(*command_buffer).expect("");
        }
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        self._pipeline_set_command = Some(PipelineSetCommand::new(pipeline));

        let layout = pipeline
            .to_data()
            .get_shader()
            .to_data()
            .get_descriptor_set_layout();
        let device_impl = self._device.to_data().get_device();
        unsafe {
            let descriptor_set = device_impl
                .allocate_descriptor_sets(
                    &ash::vk::DescriptorSetAllocateInfo::builder()
                        .set_layouts(&[*layout])
                        .descriptor_pool(self._descriptor_pool)
                        .build(),
                )
                .unwrap();
            self._descriptor_sets.push(descriptor_set[0]);
        }
    }

    // 使ってないコード
    fn set_buffer(&mut self, _buffer: &'a Buffer<'a>) {}

    fn set_unordered_access_buffer(
        &mut self,
        _slot: i32,
        _stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
        let buffer = gpu_address.to_data().get_buffer().get_buffer();
        let offset = gpu_address.to_data().get_offset();
        self._descriptor_set_builder.push(buffer, offset, size);
    }

	fn clear_color(&mut self, _color_target_view: &mut ColorTargetView, _red: f32, _green: f32, _blue: f32, _alpha: f32) {
	}

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let device_impl = self._device.to_data().get_device();
        let command_buffer_impl = self._command_buffers.iter().next().unwrap();

        // パイプラインをセット
        let c = self._pipeline_set_command.as_ref().unwrap();
        c.bind(device_impl, command_buffer_impl);

        // デスクリプタたちをセット
        let write_descriptor_sets = self
            ._descriptor_set_builder
            .build(self.get_descriptor_set());
        unsafe {
            device_impl.update_descriptor_sets(&write_descriptor_sets, &[]);

            device_impl.cmd_bind_descriptor_sets(
                *command_buffer_impl,
                ash::vk::PipelineBindPoint::COMPUTE,
                *c.get_pipeline(),
                0, /*first_point*/
                &[*self.get_descriptor_set()],
                &[], /*dynamic_offset*/
            );
        }

        // ディスパッチ
        unsafe {
            device_impl.cmd_dispatch(
                *command_buffer_impl,
                group_count_x,
                group_count_y,
                group_count_z,
            );
        }
    }
}

impl<'a> Drop for CommandBufferImpl<'a> {
    fn drop(&mut self) {
        let device_impl = self._device.to_data().get_device();
        unsafe {
            device_impl.destroy_descriptor_pool(self._descriptor_pool, None);
            device_impl.free_command_buffers(self._command_pool, &self._command_buffers);
            device_impl.destroy_command_pool(self._command_pool, None);
        }
    }
}

struct PipelineSetCommand<'a> {
    _pipeline: &'a Pipeline<'a>,
}

impl<'a> PipelineSetCommand<'a> {
    pub fn new(pipeline: &'a Pipeline) -> Self {
        Self {
            _pipeline: pipeline,
        }
    }

    pub fn bind(&self, device: &ash::Device, command_buffer: &ash::vk::CommandBuffer) {
        let pipeline = self._pipeline.to_data().get_pipeline();
        unsafe {
            device.cmd_bind_pipeline(
                *command_buffer,
                ash::vk::PipelineBindPoint::COMPUTE,
                *pipeline,
            );
        }
    }

    pub fn get_pipeline(&self) -> &ash::vk::PipelineLayout {
        self._pipeline
            .to_data()
            .get_shader()
            .to_data()
            .get_pipeline_layout()
    }
}
