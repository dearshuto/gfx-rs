use crate::{ICommandBuffer, IDevice, ISwapChain, IFence};

pub struct QueueInfo {}

impl QueueInfo {
    pub fn new() -> QueueInfo {
        QueueInfo {}
    }
}

pub trait IQueue {
    type DeviceType: IDevice;
    type CommandBufferType: ICommandBuffer;
    type FenceType: IFence<DeviceType = Self::DeviceType>;
    type SwapChainType: ISwapChain;

    fn new(device: &mut Self::DeviceType, info: &QueueInfo) -> Self;

    fn execute(&mut self, command_buffer: &Self::CommandBufferType);

    fn execute_with_fence(
        &mut self,
        command_buffer: &Self::CommandBufferType,
        fence: &mut Self::FenceType,
    );

    fn present(&mut self, swap_chain: &mut Self::SwapChainType);

    fn flush(&mut self);

    fn sync(&mut self);
}
