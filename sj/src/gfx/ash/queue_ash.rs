use super::super::queue_api::{IQueueImpl, QueueInfo};
use super::super::{CommandBuffer, Device, SwapChain};
use ash::version::DeviceV1_0;

pub struct QueueImpl<'a> {
    _device: &'a Device,
    _queue: ash::vk::Queue,
    _queue_submit_infos: Vec<ash::vk::SubmitInfo>,
    _queue_family_index: u32,
    _queue_index: u32,
    _dst_wait_mask: [ash::vk::PipelineStageFlags; 2],
}

impl<'a> IQueueImpl<'a> for QueueImpl<'a> {
    fn new(device: &'a Device, _info: &QueueInfo) -> Self {
        unsafe {
            let queue_family_index = 0;
            let queue_index = 0;
            let queue = device.to_data()._device.get_device_queue(0, 0);
            Self {
                _device: device,
                _queue: queue,
                _queue_submit_infos: Vec::new(),
                _queue_family_index: queue_family_index,
                _queue_index: queue_index,
                _dst_wait_mask: [
                    ash::vk::PipelineStageFlags::VERTEX_SHADER,
                    ash::vk::PipelineStageFlags::FRAGMENT_SHADER,
                ],
            }
        }
    }

    fn execute(&mut self, command_buffer: &CommandBuffer<'a>) {
        let command_buffer_impl = command_buffer.to_data();

        // コマンドが存在しない場合は処理をすかす
        if command_buffer_impl.get_command_count() == 0 {
            return;
        }

        let command_buffers = command_buffer_impl.get_command_buffers();
        let submit_info = ash::vk::SubmitInfo::builder()
            .command_buffers(&command_buffers)
            .build();
        self._queue_submit_infos.push(submit_info);

        self.flush();
    }

    fn present(&mut self, swap_chain: &mut SwapChain, _present_interval: i32) {
        let swap_chain_khr = swap_chain.to_data().get_swap_chain_khr();
        let swap_chains = [swap_chain_khr];
        let image_indices = [swap_chain.to_data().get_buffer_index() as u32];
        let present_info = ash::vk::PresentInfoKHR::builder()
            .swapchains(&swap_chains)
            .image_indices(&image_indices)
            .build();
        let swap_chain_loader = swap_chain.to_data().get_swap_chain();
        unsafe {
            swap_chain_loader
                .queue_present(self._queue, &present_info)
                .unwrap();
        }

        swap_chain.update();
    }

    fn flush(&mut self) {
        let device_impl = self._device.to_data().get_device();
        unsafe {
            device_impl
                .queue_submit(
                    self._queue,
                    &self._queue_submit_infos,
                    ash::vk::Fence::null(),
                )
                .expect("Fail to submit");
        }

        self._queue_submit_infos.clear();
    }

    fn sync(&self) {
        unsafe {
            self._device
                .to_data()
                .get_device()
                .device_wait_idle()
                .unwrap();
        }
    }

    fn sync_semaphore(&mut self, semaphore: &mut crate::gfx::Semaphore) {
        let semaphores = [semaphore.to_data_mut().get_semaphore()];
        let submit_info = ash::vk::SubmitInfo {
            p_wait_semaphores: semaphores.as_ptr(),
            wait_semaphore_count: 1,
            p_wait_dst_stage_mask: self._dst_wait_mask.as_ptr(),
            ..Default::default()
        };

        let device_impl = self._device.to_data().get_device();
        unsafe {
            device_impl
                .queue_submit(self._queue, &[submit_info], ash::vk::Fence::null())
                .expect("Fail to submit");
        }
    }
}

impl<'a> Drop for QueueImpl<'a> {
    fn drop(&mut self) {
        // とくにやることない
    }
}
