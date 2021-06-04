use super::{BufferUsage, Device, MemoryPool};
use std::marker::PhantomData;

pub struct BufferInfo {
    _size: u64,
    _buffer_usage: BufferUsage,
}

impl BufferInfo {
    pub fn new() -> Self {
        BufferInfo {
            _size: 0,
            _buffer_usage: BufferUsage::empty(),
        }
    }

    pub fn get_size(&self) -> u64 {
        self._size
    }

    pub fn set_size(mut self, size: u64) -> Self {
        self._size = size;
        self
    }

    pub fn get_buffer_usage(&self) -> BufferUsage {
        self._buffer_usage
    }

    pub fn set_buffer_usage(mut self, buffer_usage: BufferUsage) -> Self {
        self._buffer_usage = buffer_usage;
        self
    }
}

pub trait IBufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self;

    fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64;

    fn map<T>(&self) -> &mut T;

    fn map_as_slice<U>(&self, count: usize) -> &[U];

    fn map_as_slice_mut<U>(&self, count: usize) -> &mut [U];

    fn unmap(&self);

    fn flush_mapped_range(&self, offset: i64, size: u64);

    fn invalidate_mapped_range(&self, offset: i64, size: u64);
}

pub struct TBufferInterface<'a, T: 'a>
where
    T: IBufferImpl<'a>,
{
    buffer_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T> TBufferInterface<'a, T>
where
    T: IBufferImpl<'a>,
{
    pub fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        Self {
            buffer_impl: T::new(device, info, memory_pool, offset, size),
            _marker: PhantomData,
        }
    }

    pub fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64 {
        T::get_required_alignment(device, info)
    }

    pub fn map<U>(&self) -> &mut U {
        self.buffer_impl.map()
    }

    pub fn map_as_slice<U>(&self, count: usize) -> &[U] {
        self.buffer_impl.map_as_slice(count)
    }

    pub fn map_as_slice_mut<U>(&self, count: usize) -> &mut [U] {
        self.buffer_impl.map_as_slice_mut(count)
    }

    pub fn unmap(&self) {
        self.buffer_impl.unmap();
    }

    pub fn flush_mapped_range(&self, offset: i64, size: u64) {
        self.buffer_impl.flush_mapped_range(offset, size);
    }

    pub fn invalidate_mapped_range(&self, offset: i64, size: u64) {
        self.buffer_impl.invalidate_mapped_range(offset, size);
    }

    pub fn to_data(&'a self) -> &'a T {
        &self.buffer_impl
    }
}
