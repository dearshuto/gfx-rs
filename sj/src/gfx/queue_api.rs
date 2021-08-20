use super::{CommandBuffer, Device, SwapChain};
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

    fn present(&mut self, _swap_chain: &mut SwapChain, _present_interval: i32);

    fn flush(&mut self);

    fn sync(&self);

    fn sync_semaphore(&mut self, semaphore: &mut crate::gfx::Semaphore);
}

pub struct TQueueInterface<'a, T: 'a>
where
    T: IQueueImpl<'a>,
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

    pub fn execute(&mut self, command_buffer: &'a CommandBuffer<'a>) {
        self.queue_impl.execute(command_buffer);
    }

    pub fn flush(&mut self) {
        self.queue_impl.flush();
    }

    pub fn sync(&mut self) {
        self.queue_impl.sync();
    }

    pub fn sync_semaphore(&mut self, semaphore: &mut crate::gfx::Semaphore) {
        self.queue_impl.sync_semaphore(semaphore);
    }

    pub fn present(&mut self, swap_chain: &mut SwapChain, present_interval: i32) {
        self.queue_impl.present(swap_chain, present_interval);
    }

    pub fn to_data(&self) -> &T {
        &self.queue_impl
    }

    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self.queue_impl
    }
}

#[cfg(test)]
mod tests {
    #[cfg(test)]
    use crate::gfx::{CommandBuffer, CommandBufferInfo, Device, DeviceInfo, Queue, QueueInfo};

    #[test]
    fn new() {
        let device = Device::new(&DeviceInfo::new());
        let _queue = Queue::new(&device, &QueueInfo::new());
    }

    #[test]
    fn sync_test() {
        let device = Device::new(&DeviceInfo::new());
        let mut queue = Queue::new(&device, &QueueInfo::new());
        queue.sync();
    }

    #[test]
    fn execure_test() {
        let device = Device::new(&DeviceInfo::new());
        let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
        let mut queue = Queue::new(&device, &QueueInfo::new());

        command_buffer.begin();
        command_buffer.end();
        queue.execute(&command_buffer);
        queue.sync();
    }
}
