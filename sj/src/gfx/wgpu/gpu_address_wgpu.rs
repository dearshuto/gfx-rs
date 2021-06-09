use super::super::gpu_address_api::IGpuAddressImpl;
use super::super::Buffer;

pub struct GpuAddressWgpu {}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu {
    fn new<'buffer: 'a>(buffer: &'buffer Buffer<'buffer>) -> Self {
        todo!();
    }

    fn offset(&mut self, offset: i64) {
        todo!();
    }
}
