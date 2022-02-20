use sjgfx_interface::{IDevice, IQueue, QueueInfo};

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

impl<'a> IQueue for QueueWgpu<'a> {
    fn new<TDevice: IDevice>(_device: &TDevice, _info: &sjgfx_interface::QueueInfo) -> Self {
        todo!()
    }
}
