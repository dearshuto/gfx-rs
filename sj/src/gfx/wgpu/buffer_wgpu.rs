use super::super::buffer_api::{BufferInfo, IBufferImpl};
use super::super::{Device, MemoryPool};
use std::marker::PhantomData;

pub struct BufferImpl<'a> {
    buffer_impl: wgpu::Buffer,
    _marker: PhantomData<&'a i32>,
}

impl<'a> BufferImpl<'a> {
    pub fn map(&self) {
        let buffer_slice = self.buffer_impl.slice(..);
    }
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        let slice_size = 1024 * std::mem::size_of::<u32>();
        let size = slice_size as wgpu::BufferAddress;
        let buffer = device
            .to_data()
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                size,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
            });

        BufferImpl {
            buffer_impl: buffer,
            _marker: PhantomData,
        }
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

    fn map_as_slice_mut<U>(&self, count: usize) -> &mut [U] {
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
