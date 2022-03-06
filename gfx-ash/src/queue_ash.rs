use sjgfx_interface::QueueInfo;

use crate::{CommandBufferAsh, DeviceAsh, SwapChainAsh};

pub struct QueueAsh {
    device: ash::Device,
    queue: ash::vk::Queue,
    queue_submit_infos: Vec<ash::vk::SubmitInfo>,
}

impl QueueAsh {
    pub fn new(device: &DeviceAsh, _info: &QueueInfo) -> Self {
        Self {
            device: device.get_device(),
            queue: device.get_queue_handle(),
            queue_submit_infos: Vec::new(),
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferAsh) {
        let submit_info = ash::vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer.get_command_buffer()])
            .build();

        self.queue_submit_infos.push(submit_info);

        // flush() で execute() と present() の内容を一気に実行しようかと思ったけど、
        // present() を遅延評価するのがめんどくさそうだったから即時実行しちゃう
        self.flush();
    }

    pub fn present(&mut self, swap_chain: &mut SwapChainAsh) {
        let swap_chain_khr = swap_chain.get_swap_chain_khr();
        let swap_chains = [swap_chain_khr];
        let image_indices = [swap_chain.get_current_view_index()];
        let present_info = ash::vk::PresentInfoKHR::builder()
            .swapchains(&swap_chains)
            .image_indices(&image_indices)
            .build();
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

impl Drop for QueueAsh {
    fn drop(&mut self) {
        // とくにやることない
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, QueueInfo};

    use crate::{DeviceAsh, QueueAsh};

    #[test]
    fn new() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _queue = QueueAsh::new(&device, &QueueInfo::new());
    }

    #[test]
    fn flush_empty() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let mut queue = QueueAsh::new(&device, &QueueInfo::new());
        queue.flush();
    }

    #[test]
    fn sync_empty() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let queue = QueueAsh::new(&device, &QueueInfo::new());
        queue.sync();
    }
}
