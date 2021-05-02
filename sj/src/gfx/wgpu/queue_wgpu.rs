use super::super::queue_api::QueueInfo;
use super::super::Device;
use super::super::CommandBuffer;

pub struct QueueImpl<'a> {
	device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> super::super::queue_api::IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
		let device_impl = device.to_data().get_device();
        let queue = device.to_data().get_queue();		
        QueueImpl { device: device_impl, queue }
    }

	fn execute(&mut self, command_buffer: &'a CommandBuffer<'a>)
	{
		let command_buffer_impl = command_buffer.to_data().get_command_buffer();
		self.queue.submit(Some(command_buffer_impl));
	}
	
	fn flush(&self)
	{
		
	}
    
    fn sync(&self)
	{
		self.device.poll(wgpu::Maintain::Wait);
	}
}
