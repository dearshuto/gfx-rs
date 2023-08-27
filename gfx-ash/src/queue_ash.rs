use crate::DeviceAsh;
use sjgfx_interface::QueueInfo;

pub struct QueueAsh {
    handle: ash::vk::Queue,
}

impl QueueAsh {
    pub fn new(device: &DeviceAsh, _info: &QueueInfo) -> Self {
        let handle = unsafe { device.handle().get_device_queue(0, 0) };
        Self { handle }
    }

    pub fn handle(&self) -> ash::vk::Queue {
        self.handle.clone()
    }
}
