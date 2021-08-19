use super::super::queue_api::QueueInfo;
use super::super::CommandBuffer;
use super::super::Device;

pub struct QueueImpl<'a>
{
    _device: &'a Device,
}

impl<'a> super::super::queue_api::IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
        let device_impl = device.to_data().get_device();
        QueueImpl {
            _device: device,
        }
    }

    fn execute(&mut self, command_buffer: &CommandBuffer<'a>) {
        let command_encoder = command_buffer.to_data().create_command_encoder();
        self._device
            .to_data()
            .get_queue()
            .submit(Some(command_encoder.finish()));
    }

    fn present(&mut self, _swap_chain: &mut crate::gfx::SwapChain, _present_interval: i32) {}

    fn flush(&mut self) {}

    fn sync(&self) {
        self._device
            .to_data()
            .get_device()
            .poll(wgpu::Maintain::Wait);
    }

    fn sync_semaphore(&mut self, _semaphore: &mut crate::gfx::Semaphore) {}
}
