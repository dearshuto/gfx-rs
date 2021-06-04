use ash::version::DeviceV1_0;
use super::super::super::{Device, CommandBuffer, Pipeline};
use super::set_pipeline_command::SetPipelineParams;
use super::dispatch_command::DispatchParams;

pub struct CommandBuilder<'a> {
	_device: &'a Device,
	_commands: Vec<Command<'a>>,
	_marker: std::marker::PhantomData<&'a ()>,
	_current_descriptor_set: Option<ash::vk::DescriptorSet>,
}

impl<'a> CommandBuilder<'a> {
	pub fn new(device: &'a Device) -> Self {
		Self{
			_device: device,
			_commands: Vec::new(),
			_marker: std::marker::PhantomData,
			_current_descriptor_set: None,
		}
	}

	pub fn push_set_pipeline(&mut self, command_buffer: &'a CommandBuffer, pipeline: &'a Pipeline<'a>){
		let set_pipelie_params = SetPipelineParams::new(self._device, pipeline, command_buffer.to_data().get_descriptor_pool());
		self._current_descriptor_set = Some(*set_pipelie_params.get_descriptor_set());
		self._commands.push(Command::SetPipeline(set_pipelie_params));
	}
	
    fn push_dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
	}
}

enum Command<'a> {
	SetPipeline(SetPipelineParams<'a>),
	Dispatch(DispatchParams)
}
