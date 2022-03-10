use sjgfx_interface::{IQueue, QueueInfo};

use crate::{CommandBufferWgpu, DeviceWgpu, FenceWgpu, SwapChainWgpu};

pub struct QueueWgpu<'a> {
    device: &'a DeviceWgpu,
}

impl<'a> QueueWgpu<'a> {
    pub fn new(device: &'a DeviceWgpu, _info: &QueueInfo) -> Self {
        Self { device }
    }

    pub fn get_device(&self) -> &DeviceWgpu {
        self.device
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferWgpu) {
        let build_command = command_buffer.build_command();
        self.device.get_queue().submit(build_command);
        self.get_device().get_device().poll(wgpu::Maintain::Wait);
    }

    pub fn execute_with_fence(
        &mut self,
        _command_buffer: &CommandBufferWgpu,
        _fence: &mut FenceWgpu,
    ) {
        todo!()
    }

    pub fn present(&self, swap_chain: &mut SwapChainWgpu) {
        swap_chain.present();
    }

    pub fn flush(&self) {}

    pub fn sync(&self) {}
}

impl<'a> IQueue<'a> for QueueWgpu<'a> {
    type DeviceType = DeviceWgpu;
    type CommandBufferType = CommandBufferWgpu<'a>;
    type FenceType = FenceWgpu;
    type SwapChainType = SwapChainWgpu<'a>;

    fn new(device: &'a Self::DeviceType, info: &sjgfx_interface::QueueInfo) -> Self {
        QueueWgpu::new(device, info)
    }

    fn execute(&mut self, command_buffer: &Self::CommandBufferType) {
        self.execute(command_buffer);
    }

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        fence: &mut Self::FenceType,
    ) {
        self.execute_with_fence(command_buffer, fence);
    }

    fn present(&self, swap_chain: &mut Self::SwapChainType) {
        self.present(swap_chain);
    }

    fn flush(&self) {
        self.flush();
    }

    fn sync(&mut self) {
        QueueWgpu::sync(&self);
    }
}
