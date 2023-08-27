pub struct CommandBufferAsh {
    handle: ash::vk::CommandBuffer,
}

impl CommandBufferAsh {
    pub fn handle(&self) -> ash::vk::CommandBuffer {
        self.handle
    }
}
