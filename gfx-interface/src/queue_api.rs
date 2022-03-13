use crate::{ICommandBuffer, IDevice, ISwapChain};

pub struct QueueInfo {}

impl QueueInfo {
    pub fn new() -> QueueInfo {
        QueueInfo {}
    }
}

pub trait IQueue {
    type DeviceType: IDevice;
    type CommandBufferType: ICommandBuffer;
    type FenceType;
    type SwapChainType: ISwapChain;

    fn new(device: &Self::DeviceType, info: &QueueInfo) -> Self;

    fn execute(&mut self, command_buffer: &Self::CommandBufferType);

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        fence: &mut Self::FenceType,
    );

    fn present(&self, swap_chain: &mut Self::SwapChainType);

    fn flush(&self);

    fn sync(&mut self);
}
