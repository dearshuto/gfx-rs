use std::sync::Arc;

use crate::BufferWgpu;

pub struct GpuAddressWgpu {
    buffer: Arc<wgpu::Buffer>,
}

impl GpuAddressWgpu {
    pub(crate) fn new(buffer: &BufferWgpu) -> Self {
        Self {
            buffer: buffer.close_buffer(),
        }
    }

    pub fn get_binding_resource(&self) -> wgpu::BindingResource {
        self.buffer.as_entire_binding()
    }
}
