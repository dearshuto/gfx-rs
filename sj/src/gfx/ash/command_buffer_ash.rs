use std::marker::PhantomData;
use ash::version::DeviceV1_0;

use super::super::{Buffer, Device, Pipeline};
use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};

pub struct CommandBufferImpl<'a>
{
	_device: &'a Device,
	_command_pool: ash::vk::CommandPool,
	_command_buffers: Vec<ash::vk::CommandBuffer>,
	_pipeline_set_command: Option<PipelineSetCommand<'a>>,
	_write_descriptor_set_builders: Vec<WriteDescriptorSetBuilder<'a>>,
	_marker: PhantomData<&'a u32>,
}

impl<'a> CommandBufferImpl<'a>
{
	pub fn get_command_buffers(&self) -> &Vec<ash::vk::CommandBuffer>
	{
		&self._command_buffers
	}

	pub fn get_command_count(&self) -> i32 {
		0 // TOO
	}
}

impl<'a> ICommandBufferImpl<'a> for CommandBufferImpl<'a>
{
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self
	{
		let device_impl = device.to_data().get_device();
		unsafe {
			let command_pool = device_impl.create_command_pool(
				&ash::vk::CommandPoolCreateInfo::builder()
					.flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
					.queue_family_index(device.to_data().get_queue_family_index())
					.build()
					, None)
				.expect("command pool");
			let command_buffers = device_impl.allocate_command_buffers(&ash::vk::CommandBufferAllocateInfo::builder()
																	   .command_buffer_count(2)
																	   .command_pool(command_pool)
																	   .level(ash::vk::CommandBufferLevel::PRIMARY)
																	  .build())
				.unwrap();

			//let buffer_info =  ash::vk::DescriptorBufferInfo::builder()
			//.buffer()
			//	.build();
			//let image_info = ash::vk::DescriptorImageInfo::builder()
			//	.build();

			// ash::vk::WriteDescriptorSet::builder()
			// 	.buffer_info(&[buffer_info])
			// 	.image_info(&[image_info])
			// 	.build();
			
			Self{
				_device: device,
				_command_pool: command_pool,
				_command_buffers: command_buffers,
				_write_descriptor_set_builders: Vec::new(),
				_pipeline_set_command: None,
				_marker: PhantomData,
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
			device_impl.begin_command_buffer(*command_buffer, &command_buffer_begin_info).expect("");
		}
	}

	fn end(&mut self) {
		let device_impl = self._device.to_data().get_device();
		let command_buffer = self._command_buffers.iter().next().unwrap();
		
		unsafe {
			device_impl.end_command_buffer(*command_buffer).expect("");
		}
	}
	
	fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>)
	{
		self._pipeline_set_command = Some(PipelineSetCommand::new(pipeline));
	}

	fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32)
	{			
		let device_impl = self._device.to_data().get_device();
		let command_buffer_impl = self._command_buffers.iter().next().unwrap();
//		let pipeline = self._pipeline_set_command.

		// パイプラインをセット
		self._pipeline_set_command.as_ref().unwrap().bind(device_impl, command_buffer_impl);

		// デスクリプタたちをセット
		let mut write_descriptor_sets = Vec::new();
		for builder in &self._write_descriptor_set_builders {
			write_descriptor_sets.push(builder.build());
		 }
		unsafe {
			device_impl.update_descriptor_sets(&write_descriptor_sets, &[]);
		}
		
		// ディスパッチ
		unsafe {
			device_impl.cmd_dispatch(*command_buffer_impl, group_count_x, group_count_y, group_count_z);
		}
	}	
}

impl<'a> Drop for CommandBufferImpl<'a>
{
	fn drop(&mut self) {
		let device_impl = self._device.to_data().get_device();
		unsafe {
			device_impl.free_command_buffers(self._command_pool, &self._command_buffers);
			device_impl.destroy_command_pool(self._command_pool, None);
		}
	}
}

struct PipelineSetCommand<'a>
{
	_pipeline: &'a Pipeline<'a>,
}

impl<'a> PipelineSetCommand<'a>
{
	pub fn new(pipeline:  &'a Pipeline) -> Self{
		Self{
			_pipeline: pipeline,
		}
	}

	pub fn bind(&self, device: &ash::Device, command_buffer: &ash::vk::CommandBuffer) {
		let pipeline = self._pipeline.to_data().get_pipeline();
		unsafe {
			device.cmd_bind_pipeline(*command_buffer, ash::vk::PipelineBindPoint::COMPUTE, *pipeline);
		}
	}
}

struct WriteDescriptorSetBuilder<'a>
{
	_buffer: Option<&'a Buffer<'a>>,
}

impl<'a> WriteDescriptorSetBuilder<'a>
{
	pub fn build(&self) -> ash::vk::WriteDescriptorSet
	{
		let descriptor_buffer_info = ash::vk::DescriptorBufferInfo::builder()
			//.buffer()
			.build();
		
		ash::vk::WriteDescriptorSet::builder()
			.descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
			.buffer_info(&[descriptor_buffer_info])
			.build()
	}
}
