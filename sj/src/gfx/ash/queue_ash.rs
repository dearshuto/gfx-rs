use std::marker::PhantomData;
use ash::version::DeviceV1_0;
use super::super::{Device, CommandBuffer};
use super::super::queue_api::{QueueInfo, IQueueImpl};

pub struct QueueImpl<'a>
{
	_device: &'a Device,
	_queue: ash::vk::Queue,
	_queue_submit_infos: Vec<ash::vk::SubmitInfo>,
	_queue_family_index: u32,
	_queue_index: u32,
	_marker: PhantomData<&'a u32>,
}

impl<'a> IQueueImpl<'a> for QueueImpl<'a>
{
	fn new(device: &'a Device, _info: &QueueInfo) -> Self
	{
		unsafe{
			let queue_family_index = 0;
			let queue_index = 0;
			let queue =  device.to_data()._device.get_device_queue(0, 0);
			
			Self
			{
				_device: device,
				_queue: queue,
				_queue_submit_infos: Vec::new(),
				_queue_family_index: queue_family_index,
				_queue_index: queue_index,
				_marker: PhantomData,
			}			
		}
	}
	
	fn execute(&mut self, command_buffer: &CommandBuffer<'a>)
	{
		let command_buffer_impl = command_buffer.to_data();
		let command_buffers = command_buffer_impl.get_command_buffers();
		
		let submit_info = ash::vk::SubmitInfo::builder()
			.command_buffers(&command_buffers)
			.build();
		self._queue_submit_infos.push(submit_info);		

		// let device_impl = self._device.to_data().get_device();
		// for item in command_buffer_impl.get_internal_commands() {
		// 	item.execute(&device_impl, &self._queue);
		// }
		
		self.flush();
	}
	
    fn flush(&mut self)
	{
		let device_impl = self._device.to_data().get_device();
		unsafe {
			
			device_impl.queue_submit(self._queue, &self._queue_submit_infos, ash::vk::Fence::null()).expect("Fail to submit");
		}				

		self._queue_submit_infos.clear();
	}
    
    fn sync(&self)
	{
		unsafe {
			self._device.to_data().get_device().device_wait_idle().unwrap();
		}
	}
}

impl<'a> Drop for QueueImpl<'a>
{
	fn drop(&mut self) {
		// とくにやることない
	}
}
