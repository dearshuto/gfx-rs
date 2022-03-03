use crate::BufferWgpu;

pub struct GpuAddressWgpu<'a> {
    buffer: &'a BufferWgpu<'a>,
}

impl<'a> GpuAddressWgpu<'a> {
    pub(crate) fn new(buffer: &'a BufferWgpu) -> Self {
        Self { buffer }
    }

    pub fn get_binding_resource(&self) -> wgpu::BindingResource {
        self.buffer.get_buffer().as_entire_binding()
    }
}
