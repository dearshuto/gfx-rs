use super::super::gpu_address_api::IGpuAddressImpl;
use super::super::Buffer;

pub struct GpuAddressWgpu<'a> {
    _buffer: &'a Buffer<'a>,
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu<'a> {
    fn new(buffer: &'a Buffer) -> Self {
        Self { _buffer: buffer }
    }

    fn offset(&mut self, _offset: i64) {
        todo!();
    }
}

impl<'a> GpuAddressWgpu<'a> {
    pub fn get_buffer(&self) -> &Buffer {
        self._buffer
    }
}
