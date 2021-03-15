extern crate vulkano;
use super::device_vk::DeviceAccessor;

pub struct Queue
{
    queue_impl : std::sync::Arc<vulkano::device::Queue>,
}

impl Queue
{
    pub fn new(device: &super::device_vk::Device, info : &super::super::QueueInfo) -> Queue
    {
	Queue{
	    queue_impl : device.get_queue().clone()
	}
    }
}
 
