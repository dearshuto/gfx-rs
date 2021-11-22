use super::{super::gpu_address_api::IGpuAddressImpl, buffer_wgpu::BufferImpl};
use crate::gfx::Buffer;

pub struct GpuAddressWgpu<'a> {
    _buffer: &'a Buffer<'a>,
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu<'a> {
    fn new(buffer: &'a Buffer<'a>) -> Self {
        Self { _buffer: buffer }
    }

    fn offset(&mut self, _offset: i64) {
        todo!();
    }
}

impl<'a> GpuAddressWgpu<'a> {
    pub fn get_buffer(&self) -> &BufferImpl<'a> {
        &self._buffer.to_data()
    }
}
