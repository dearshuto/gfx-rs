use crate::{CommandBufferVk, DeviceVk, FenceVk, SwapChainVk};
use sjgfx_interface::{IQueue, QueueInfo};
use std::sync::Arc;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::device::Queue;
use vulkano::{
    command_buffer::PrimaryAutoCommandBuffer,
    device::Device,
    swapchain::{Swapchain, SwapchainAcquireFuture},
    sync,
    sync::{FlushError, GpuFuture},
};
use winit::window::Window;

pub struct QueueVk {
    device: Arc<Device>,
    queue: Arc<Queue>,
    previous_frame_end: Option<Box<dyn GpuFuture>>,
    command_builder: Option<AutoCommandBufferBuilder<PrimaryAutoCommandBuffer>>,
    swap_chain: Option<Arc<Swapchain<Window>>>,
    swap_chain_acquire_future: Option<SwapchainAcquireFuture<Window>>,
    image_index: Option<usize>,
}

impl QueueVk {
    pub fn new(device: &DeviceVk, _info: &QueueInfo) -> Self {
        Self {
            device: device.clone_device(),
            queue: device.clone_queue(),
            previous_frame_end: Some(sync::now(device.clone_device()).boxed()),
            command_builder: None,
            swap_chain: None,
            swap_chain_acquire_future: None,
            image_index: None,
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferVk) {
        let command_builder = command_buffer.build_command_builder();
        self.command_builder = Some(command_builder);
    }

    pub fn flush(&mut self) {
        self.previous_frame_end.as_mut().unwrap().cleanup_finished();

        if self.command_builder.is_none() {
            return;
        }

        // CommandBuilder
        let mut command_builder = None;
        std::mem::swap(&mut command_builder, &mut self.command_builder);

        // SwapChain
        let mut swap_chain = None;
        std::mem::swap(&mut swap_chain, &mut self.swap_chain);

        // SwapChain Image Index
        let mut image_index = None;
        std::mem::swap(&mut image_index, &mut self.image_index);

        // SwapChain Acquire Future
        let mut swap_chain_acquire_future = None;
        std::mem::swap(
            &mut swap_chain_acquire_future,
            &mut self.swap_chain_acquire_future,
        );

        if swap_chain_acquire_future.is_some() {
            let future = self
                .previous_frame_end
                .take()
                .unwrap()
                .join(swap_chain_acquire_future.unwrap())
                .then_execute(
                    self.queue.clone(),
                    command_builder.unwrap().build().unwrap(),
                )
                .unwrap()
                .then_swapchain_present(
                    self.queue.clone(),
                    swap_chain.unwrap(),
                    image_index.unwrap(),
                )
                .then_signal_fence_and_flush();

            let next_frame = match future {
                Ok(future) => {
                    // TODO
                    //future.wait(None).unwrap();
                    Some(future.boxed())
                }
                Err(FlushError::OutOfDate) => {
                    //recreate_swapchain = true;
                    Some(sync::now(self.device.clone()).boxed())
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    Some(sync::now(self.device.clone()).boxed())
                }
            };
            self.previous_frame_end = next_frame;
        } else {
            let future = self
                .previous_frame_end
                .take()
                .unwrap()
                .then_execute(
                    self.queue.clone(),
                    command_builder.unwrap().build().unwrap(),
                )
                .unwrap()
                .then_signal_fence_and_flush();

            let next_frame = match future {
                Ok(future) => {
                    // TODO
                    //future.wait(None).unwrap();
                    Some(future.boxed())
                }
                Err(FlushError::OutOfDate) => {
                    //recreate_swapchain = true;
                    Some(sync::now(self.device.clone()).boxed())
                }
                Err(e) => {
                    println!("Failed to flush future: {:?}", e);
                    Some(sync::now(self.device.clone()).boxed())
                }
            };
            self.previous_frame_end = next_frame;
        }
    }

    pub fn present(&mut self, swap_chain: &mut SwapChainVk) {
        self.swap_chain_acquire_future = Some(swap_chain.unwrap_acquire_future());
        self.image_index = Some(swap_chain.get_current_index() as usize);
        self.swap_chain = Some(swap_chain.clone_swap_chain());
    }

    pub fn sync(&mut self) {
        self.queue.wait().unwrap();
        if let Some(future) = self.previous_frame_end.as_mut() {
            future.cleanup_finished();
        }
    }
}

impl IQueue for QueueVk {
    type DeviceType = DeviceVk;
    type CommandBufferType = CommandBufferVk;
    type FenceType = FenceVk;
    type SwapChainType = SwapChainVk;

    fn new(device: &mut Self::DeviceType, info: &QueueInfo) -> Self {
        Self::new(device, info)
    }

    fn execute(&mut self, command_buffer: &Self::CommandBufferType) {
        self.execute(command_buffer);
    }

    fn execute_with_fence(
        &mut self,
        _command_buffer: &Self::CommandBufferType,
        _fence: &mut Self::FenceType,
    ) {
        todo!()
    }

    fn present(&mut self, swap_chain: &mut Self::SwapChainType) {
        self.present(swap_chain);
    }

    fn flush(&mut self) {
        self.flush();
    }

    fn sync(&mut self) {
        self.sync();
    }
}
