use crate::gfx::buffer_api::IBufferImpl;
use crate::gfx::{BufferInfo, Device};

pub struct BufferVk<'a> {
    _device: &'a Device,
    _buffer: std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer>,
}

impl<'a> IBufferImpl<'a> for BufferVk<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a crate::gfx::MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        todo!()
    }

    fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64 {
        todo!()
    }

    fn map<T>(&self) -> &mut T {
        todo!()
    }

    fn map_as_slice<U>(&self, count: usize) -> &[U] {
        todo!()
    }

    fn map_as_slice_mut<U>(&self, count: usize) -> crate::gfx::buffer_api::MappedData<U> {
        todo!()
    }

    fn unmap(&self) {
        todo!()
    }

    fn flush_mapped_range(&self, offset: i64, size: u64) {
        todo!()
    }

    fn invalidate_mapped_range(&self, offset: i64, size: u64) {
        todo!()
    }
}
