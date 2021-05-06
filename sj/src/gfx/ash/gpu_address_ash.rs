use super::super::gpu_address_api::IGpuAddressImpl;
use super::super::Buffer;
use super::buffer_ash::BufferImpl;

pub struct GpuAddressImpl<'a> {
    _buffer: &'a BufferImpl<'a>,
    _offset: i64,
}

impl<'a> GpuAddressImpl<'a> {
    pub fn get_buffer(&self) -> &'a BufferImpl<'a> {
        self._buffer
    }

    pub fn get_offset(&self) -> i64 {
        self._offset
    }
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressImpl<'a> {
    fn new<'buffer: 'a>(buffer: &'buffer Buffer<'buffer>) -> Self {
        Self {
            _buffer: buffer.to_data(),
            _offset: 0,
        }
    }

    fn offset(&mut self, _offset: i64) {}
}
