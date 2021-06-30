use super::super::gpu_address_api::IGpuAddressImpl;
use super::super::Buffer;

pub struct GpuAddressWgpu {}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu {
    fn new<'buffer: 'a>(_buffer: &'buffer Buffer<'buffer>) -> Self {
        todo!();
    }

    fn offset(&mut self, _offset: i64) {
        todo!();
    }
}

impl GpuAddressWgpu {
    pub fn get_buffer(&self) -> &Buffer {
        todo!();
    }
}
