use crate::{CommandBufferAsh, DeviceAsh, FenceAsh};
use sjgfx_interface::QueueInfo;

pub struct QueueAsh {
    device: ash::Device,
}

impl QueueAsh {
    pub fn new(device: &DeviceAsh, _info: &QueueInfo) -> Self {
        let device = device.handle();
        Self { device }
    }

    pub fn execute_command(&mut self, command_buffer: &CommandBufferAsh, fence: &FenceAsh) {
        // サブミット
        let submit_info = ash::vk::SubmitInfo::builder()
            .command_buffers(&[command_buffer.handle()])
            .build();
        unsafe {
            self.device
                .queue_submit(
                    self.device.get_device_queue(0, 0),
                    &[submit_info],
                    fence.handle(),
                )
                .unwrap();
        }
    }
}
