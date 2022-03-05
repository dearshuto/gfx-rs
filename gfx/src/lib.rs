pub mod vulkano;
pub mod wgpu;

pub struct DeviceBuilder;
impl DeviceBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct QueueBuilder;
impl QueueBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct CommandBufferBuilder;
impl CommandBufferBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
