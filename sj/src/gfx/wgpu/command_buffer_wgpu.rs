use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::Device;
use super::super::Pipeline;

pub struct CommandBuffer<'a>
{
    device: &'a wgpu::Device,
	commands: Vec<Box<ICommand>>,
}

impl<'a> ICommandBufferImpl<'a> for CommandBuffer<'a> {
    fn new(device: &'a Device, _info: &CommandBufferInfo) -> Self {
        CommandBuffer {
            device: device.to_data().get_device(),
			commands: Vec::new(),
        }
    }
}

impl<'a> CommandBuffer<'a> {
    pub fn begin(&mut self) {
    }	

    pub fn end(&mut self) {
    }

	pub fn set_pipeline(&self, pipeline: &'a Pipeline)
	{
		let a : &super::pipeline_wgpu::Pipeline = pipeline.to_data();
	}
	
    pub fn get_command_buffer(&self) -> wgpu::CommandBuffer {
		let mut command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
		{
			let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{label: None});
			// for item in &self.commands {
			// 	item.push(&mut command_encoder);
			// }
		}
		
		command_encoder.finish()
    }
}

trait ICommand
{
	fn push(&self, command_encoder: &mut wgpu::CommandEncoder);
}

struct SetPipelineCommand<'a>
{
	compute_pipeline: &'a wgpu::ComputePipeline,
}

impl<'a> SetPipelineCommand<'a> {
	fn push(&self, command_encoder: &mut wgpu::RenderPipeline, compute_pass: &'a mut wgpu::ComputePass<'a>)
	{
		compute_pass.set_pipeline(self.compute_pipeline);
	}
}
