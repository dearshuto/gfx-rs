use std::sync::Arc;

use sjgfx_interface::{IQueue, QueueInfo};

use crate::{CommandBufferWgpu, DeviceWgpu, FenceWgpu, SwapChainWgpu};

pub struct QueueWgpu {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
}

impl QueueWgpu {
    pub fn new(device: &DeviceWgpu, _info: &QueueInfo) -> Self {
        Self {
            device: device.close_device(),
            queue: device.clone_queue(),
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferWgpu) {
        let build_command = command_buffer.build_command();
        self.queue.submit(build_command);
        self.device.poll(wgpu::Maintain::Wait);
    }

    pub fn submit_command_buffer_direct(&self, command_buffer: wgpu::CommandBuffer) {
        self.queue.submit(Some(command_buffer));
    }

    pub fn execute_with_fence(
        &mut self,
        _command_buffer: &CommandBufferWgpu,
        _fence: &mut FenceWgpu,
    ) {
        todo!()
    }

    pub fn present(&mut self, swap_chain: &mut SwapChainWgpu) {
        swap_chain.present(self);
    }

    pub fn flush(&self) {}

    pub fn sync(&self) {}
}

impl IQueue for QueueWgpu {
    type DeviceType = DeviceWgpu;
    type CommandBufferType = CommandBufferWgpu;
    type FenceType = FenceWgpu;
    type SwapChainType = SwapChainWgpu;

    fn new(device: &mut Self::DeviceType, info: &QueueInfo) -> Self {
        Self::new(device, info)
    }

    fn execute(&mut self, command_buffer: &Self::CommandBufferType) {
        self.execute(command_buffer)
    }

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        fence: &mut Self::FenceType,
    ) {
        self.execute_with_fence(command_buffer, fence)
    }

    fn present(&mut self, swap_chain: &mut Self::SwapChainType) {
        QueueWgpu::present(self, swap_chain);
    }

    fn flush(&mut self) {
        QueueWgpu::flush(&self);
    }

    fn sync(&mut self) {
        QueueWgpu::sync(&self);
    }
}
