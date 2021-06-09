use super::super::queue_api::QueueInfo;
use super::super::CommandBuffer;
use super::super::Device;

pub struct QueueImpl<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
}

impl<'a> super::super::queue_api::IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
        let device_impl = device.to_data().get_device();
        let queue = device.to_data().get_queue();
        QueueImpl {
            device: device_impl,
            queue,
        }
    }

    fn execute(&mut self, _command_buffer: &'a CommandBuffer<'a>) {
        // let command_buffer_impl = command_buffer.to_data().get_command_buffer();
        // self.queue.submit(Some(command_buffer_impl));
    }

    fn present(&mut self, _swap_chain: &mut crate::gfx::SwapChain, _present_interval: i32) {
        todo!()
    }

    fn flush(&mut self) {
        todo!()
    }

    fn sync(&self) {
        self.device.poll(wgpu::Maintain::Wait);
    }

    fn sync_semaphore(&mut self, semaphore: &mut crate::gfx::Semaphore) {
        todo!()
    }
}
