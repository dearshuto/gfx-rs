use super::{Device, CommandBuffer, SwapChain};
use std::marker::PhantomData;

pub struct QueueInfo {}

impl QueueInfo {
    pub fn new() -> QueueInfo {
        QueueInfo {}
    }
}

pub trait IQueueImpl<'a> {
    fn new(device: &'a Device, info: &QueueInfo) -> Self;
	
//	fn present(&self, swap_chain: &impl super::swap_chain::TSwapChain);

	fn execute(&mut self, command_buffer: &'a CommandBuffer<'a>);

	fn present(&self, _swap_chain: &mut SwapChain, _present_interval: i32);
	
    fn flush(&mut self);
    
    fn sync(&self);
}

pub struct TQueueInterface<'a, T: 'a>
where
    T: IQueueImpl<'a>
{
    queue_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IQueueImpl<'a>> TQueueInterface<'a, T> {
    pub fn new(device: &'a Device, info: &QueueInfo) -> Self {
        TQueueInterface {
            queue_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

	pub fn execute(&mut self, command_buffer: &'a CommandBuffer<'a>)
	{
		self.queue_impl.execute(command_buffer);
	}
	
	pub fn flush(&mut self)
	{
		self.queue_impl.flush();
	}

	pub fn sync(&mut self)
	{
		self.queue_impl.sync();
	}

	pub fn present(&mut self, swap_chain: &mut SwapChain, _present_interval: i32) {
		swap_chain.update();
	}

	pub fn to_data(&self) -> &T {
        &self.queue_impl
    }
	
    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self.queue_impl
    }
}
