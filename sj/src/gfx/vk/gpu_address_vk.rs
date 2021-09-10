use crate::gfx::gpu_address_api::IGpuAddressImpl;

pub struct GpuAddressVk {}

impl<'a> IGpuAddressImpl<'a> for GpuAddressVk {
    fn new<'buffer: 'a, TType: 'static>(
        _buffer: &'buffer crate::gfx::Buffer<'buffer, TType>,
    ) -> Self {
        todo!()
    }

    fn offset(&mut self, _offset: i64) {
        todo!()
    }
}
