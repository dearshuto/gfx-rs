use crate::gfx::gpu_address_api::IGpuAddressImpl;

pub struct GpuAddressVk {}

impl<'a> IGpuAddressImpl<'a> for GpuAddressVk {
    fn new<'buffer: 'a>(_buffer: &'buffer crate::gfx::Buffer<'buffer>) -> Self {
        todo!()
    }

    fn offset(&mut self, _offset: i64) {
        todo!()
    }
}
