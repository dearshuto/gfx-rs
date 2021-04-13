extern crate vulkano;

pub struct Queue
{
    queue_impl : std::sync::Arc<vulkano::device::Queue>,
}

impl Queue
{
    pub fn new(device: &super::device_vk::Device, _info : &super::super::QueueInfo) -> Queue
    {
	Queue{
	    queue_impl : device.get_queue().clone()
	}
    }

    pub fn sync(&mut self){}

    pub fn flush(&mut self){}

    pub fn present(&self, swap_chain: &mut super::swap_chain_vk::SwapChain)
    {
	let layer = swap_chain.get_layer();

    }
}
