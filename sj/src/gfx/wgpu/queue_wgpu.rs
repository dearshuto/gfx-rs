use super::device_wgpu::Device;
use super::command_buffer_wgpu::CommandBuffer;
use super::super::queue_info::QueueInfo;

pub struct Queue<'a>
{
	queue: &'a wgpu::Queue,
}

impl<'a> Queue<'a>
{
	pub fn new(device: &'a mut Device, _info : &QueueInfo) -> Queue<'a>
	{		
		let queue = device.get_queue();
		Queue{ queue }
	}

	pub fn execute(&self, command_buffer: &mut CommandBuffer)
	{
		let command_buffer = command_buffer.get_command_buffer();
		self.queue.submit(Some(command_buffer));
	}
}
