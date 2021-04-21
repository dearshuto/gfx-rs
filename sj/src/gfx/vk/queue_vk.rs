use super::super::Device;
use super::super::CommandBuffer;
use super::super::queue_api::{QueueInfo, IQueueImpl};
use std::marker::PhantomData;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState, SubpassContents};

pub struct QueueImpl<'a>
{
	
	queue_impl : std::sync::Arc<vulkano::device::Queue>,
	_marker: PhantomData<&'a u32>,
}

impl<'a> IQueueImpl<'a> for QueueImpl<'a>
{
	fn new(device: &'a Device, info: &QueueInfo) -> Self
	{
		let queue_impl = device.to_data().get_queue_impl();
		
		Self{
			queue_impl,
			_marker: PhantomData,
		}
	}

	fn execute(&mut self, command_buffer: &'a mut CommandBuffer<'a>)
	{
		let mut builder = AutoCommandBufferBuilder::primary_one_time_submit(
            device.clone(),
            self.queue_impl.as_ref().family(),
        ).unwrap();
		
		let a = builder.build().unwrap();
		let b = builder.build().unwrap();


	}
	
    fn flush(&self)
	{
		
	}
    
    fn sync(&self)
	{
		
	}
	
}

//impl QueueImpl<'a>
//{
    // pub fn new(device: &super::device_vk::Device, _info : &super::super::QueueInfo) -> Queue
    // {
	// Queue{
	//     queue_impl : device.get_queue().clone()
	// }
    // }

    // pub fn sync(&mut self){}

    // pub fn flush(&mut self){}

    // pub fn present(&self, swap_chain: &mut super::swap_chain_vk::SwapChain)
    // {
	// let layer = swap_chain.get_layer();

    // }
//}
