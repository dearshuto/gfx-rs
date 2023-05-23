use sjgfx_interface::{IQueue, QueueInfo};

use crate::{CommandBufferAsh, DeviceAsh, FenceAsh, SwapChainAsh};

pub struct QueueAsh<'a> {
    device: ash::Device,
    queue: ash::vk::Queue,
    queue_submit_infos: Vec<ash::vk::SubmitInfo<'a>>,
}

impl<'a> QueueAsh<'a> {
    pub fn new(device: &DeviceAsh, _info: &QueueInfo) -> Self {
        Self {
            device: device.get_device(),
            queue: device.get_queue_handle(),
            queue_submit_infos: Vec::new(),
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferAsh) {
        // let submit_info =
        //     ash::vk::SubmitInfo::default().command_buffers(&[command_buffer.get_command_buffer()]);

        // self.queue_submit_infos.push(submit_info);

        // flush() で execute() と present() の内容を一気に実行しようかと思ったけど、
        // present() を遅延評価するのがめんどくさそうだったから即時実行しちゃう
        self.flush();
    }

    pub fn present(&mut self, swap_chain: &mut SwapChainAsh) {
        let swap_chain_khr = swap_chain.get_swap_chain_khr();
        let swap_chains = [swap_chain_khr];
        let image_indices = [swap_chain.get_current_view_index()];
        let present_info = ash::vk::PresentInfoKHR::default()
            .swapchains(&swap_chains)
            .image_indices(&image_indices);
        let swap_chain_loader = swap_chain.get_swap_chain();
        unsafe { swap_chain_loader.queue_present(self.queue, &present_info) }.unwrap();
    }

    pub fn flush(&mut self) {
        unsafe {
            self.device
                .queue_submit(self.queue, &self.queue_submit_infos, ash::vk::Fence::null())
        }
        .unwrap();
        self.queue_submit_infos.clear();
    }

    pub fn sync(&self) {
        unsafe { self.device.device_wait_idle() }.unwrap();
    }
}

impl<'a> IQueue for QueueAsh<'a> {
    type DeviceType = DeviceAsh;
    type CommandBufferType = CommandBufferAsh<'a>;
    type FenceType = FenceAsh;
    type SwapChainType = SwapChainAsh;

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
        QueueAsh::sync(&self);
    }
}

impl<'a> Drop for QueueAsh<'a> {
    fn drop(&mut self) {
        // とくにやることない
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DebugMode, DeviceInfo, QueueInfo};

    use crate::{DeviceAsh, QueueAsh};

    #[test]
    fn new() {
        let device = DeviceAsh::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let _queue = QueueAsh::new(&device, &QueueInfo::new());
    }

    #[test]
    fn flush_empty() {
        let device = DeviceAsh::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let mut queue = QueueAsh::new(&device, &QueueInfo::new());
        queue.flush();
    }

    #[test]
    fn sync_empty() {
        let device = DeviceAsh::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let queue = QueueAsh::new(&device, &QueueInfo::new());
        queue.sync();
    }
}
