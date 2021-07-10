use crate::gfx::queue_api::{IQueueImpl, QueueInfo};
use crate::gfx::{CommandBuffer, Device, Semaphore, SwapChain};

pub struct QueueVk {
    _device_vk: std::sync::Arc<vulkano::device::Device>,
    _queue_impl: std::sync::Arc<vulkano::device::Queue>,
}

impl<'a> IQueueImpl<'a> for QueueVk {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
        Self {
            _device_vk: device.to_data().get_device_impl(),
            _queue_impl: device.to_data().clone_queue(),
        }
    }

    fn execute(&mut self, _command_buffer: &'a CommandBuffer<'a>) {
        todo!()
    }

    fn present(&mut self, _swap_chain: &mut SwapChain, _present_interval: i32) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn sync(&self) {
        self._queue_impl.wait().unwrap();
    }

    fn sync_semaphore(&mut self, _semaphore: &mut Semaphore) {}
}
