use super::{super::gpu_address_api::IGpuAddressImpl, buffer_wgpu::BufferImpl};

pub struct GpuAddressWgpu<'a> {
    _buffer: &'a BufferImpl<'a>,
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu<'a> {
    fn offset(&mut self, _offset: i64) {
        todo!();
    }
}

impl<'a> GpuAddressWgpu<'a> {
    pub fn new(buffer: &'a BufferImpl<'a>) -> Self {
        Self { _buffer: buffer }
    }

    pub fn get_buffer(&self) -> &BufferImpl<'a> {
        self._buffer
    }
}
