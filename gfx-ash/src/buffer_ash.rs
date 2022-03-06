pub struct BufferAsh {
    buffer: ash::vk::Buffer,
}

impl BufferAsh {
    pub fn get_buffer(&self) -> ash::vk::Buffer {
        self.buffer
    }
}
