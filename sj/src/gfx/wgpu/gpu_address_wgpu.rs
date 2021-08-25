use crate::gfx::gpu_address_api::IGpuAddressImpl;
use crate::gfx::Buffer;
use std::sync::Arc;

pub struct GpuAddressWgpu {
    _buffer: Arc<wgpu::Buffer>,
}

impl<'a> IGpuAddressImpl<'a> for GpuAddressWgpu {
    fn new(buffer: &'a crate::gfx::Buffer<'a>) -> Self {
        Self {
			_buffer: buffer.to_data().clone_buffer(),
		}
    }

    fn offset(&mut self, _offset: i64) {
        todo!();
    }
}

impl GpuAddressWgpu {
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self._buffer
    }

	pub fn clone_buffer(&self) -> Arc<wgpu::Buffer> {
		self._buffer.clone()
	}
}
