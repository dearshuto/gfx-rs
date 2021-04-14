pub struct CommandBuffer<'a>
{
	device: &'a mut wgpu::Device,
	command_encoder: Option<wgpu::CommandEncoder>,
}

impl<'a> CommandBuffer<'a>
{
	pub fn begin(&mut self)
	{
		let command_encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
		self.command_encoder = Some(command_encoder);
	}

	pub fn end(&mut self)
	{
		self.command_encoder = None;
	}	

	pub fn get_command_buffer(&mut self) -> wgpu::CommandBuffer
	{
		let command_encoder = std::mem::replace(&mut self.command_encoder, None);
		command_encoder.unwrap().finish()
	}
}
