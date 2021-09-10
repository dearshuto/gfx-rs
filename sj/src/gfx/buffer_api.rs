use super::{Device, GpuAccess, MemoryPool};
use std::marker::PhantomData;

pub struct BufferInfo {
    _size: u64,
    _gpu_access_flags: GpuAccess,
}

impl BufferInfo {
    pub fn new() -> Self {
        BufferInfo {
            _size: 0,
            _gpu_access_flags: GpuAccess::empty(),
        }
    }

    pub fn get_size(&self) -> u64 {
        self._size
    }

    pub fn set_size(mut self, size: u64) -> Self {
        self._size = size;
        self
    }

    pub fn get_gpu_access_flags(&self) -> GpuAccess {
        self._gpu_access_flags
    }

    pub fn set_gpu_access_flags(mut self, buffer_usage: GpuAccess) -> Self {
        self._gpu_access_flags = buffer_usage;
        self
    }
}

pub trait IBufferImpl<'a, TType> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self;

    fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64;

    fn map(&self);

    fn read<F: FnMut(&TType)>(&self, action: F);

    fn write<F: FnMut(&mut TType)>(&self, action: F);

    fn unmap(&self);

    fn flush_mapped_range(&self, offset: i64, size: u64);

    fn invalidate_mapped_range(&self, offset: i64, size: u64);
}

pub struct TBufferInterface<'a, T: 'a, TType>
where
    T: IBufferImpl<'a, TType>,
{
    buffer_impl: T,
    _marker: PhantomData<&'a (T, TType)>,
}

impl<'a, T, TType: Sized> TBufferInterface<'a, T, TType>
where
    T: IBufferImpl<'a, TType>,
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

    pub fn map(&self) {
        self.buffer_impl.map();
    }

    pub fn read<F: FnMut(&TType)>(&self, action: F) {
        self.buffer_impl.read(action);
    }

    pub fn write<F: FnMut(&mut TType)>(&self, action: F) {
        self.buffer_impl.write(action);
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
