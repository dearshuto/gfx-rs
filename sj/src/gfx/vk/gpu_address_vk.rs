use vulkano::buffer::BufferAccess;

use crate::gfx::gpu_address_api::IGpuAddressImpl;

pub struct GpuAddressVk {
	_buffer_acess: std::sync::Arc<dyn BufferAccess>,
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressVk {
    fn new<'buffer: 'a, TType: 'static + Send + Sync>(
        buffer: &'buffer crate::gfx::Buffer<'buffer, TType>,
    ) -> Self {

		Self {
			_buffer_acess:  buffer.to_data().clone_buffer_access()
		}
    }

    fn offset(&mut self, _offset: i64) {
        todo!()
    }
}

impl GpuAddressVk {
	pub fn clone_buffer_access(&self) -> std::sync::Arc<dyn BufferAccess> {
		self._buffer_acess.clone()
	}
}
