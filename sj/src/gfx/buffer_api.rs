use super::{Device, GpuAccess, GpuAddress, MemoryPool};
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

pub trait IBufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        memory_pool: Option<&'a MemoryPool>,
        offset: i64,
        size: u64,
    ) -> Self;

    fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64;

    fn map<T>(&self) -> &mut T;

    fn map_as_slice<U>(&self, count: usize) -> &[U];

    fn map_as_slice_mut<U>(&self, count: usize) -> MappedData<U>;

    fn unmap(&self);

    fn flush_mapped_range(&self, offset: i64, size: u64);

    fn invalidate_mapped_range(&self, offset: i64, size: u64);

    fn get_gpu_address(&self) -> GpuAddress;
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
        memory_pool: Option<&'a MemoryPool>,
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

    pub fn map_as_slice_mut<U>(&self, count: usize) -> MappedData<U> {
        self.buffer_impl.map_as_slice_mut::<U>(count)
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

    pub fn get_gpu_address(&self) -> GpuAddress {
        self.buffer_impl.get_gpu_address()
    }

    pub fn to_data(&self) -> &T {
        &self.buffer_impl
    }
}

pub struct MappedData<'a, T> {
    _raw_ptr: *mut std::ffi::c_void,
    _aligned_data: &'a mut [T],
}

impl<'a, T> MappedData<'a, T> {
    pub fn new(raw_ptr: *mut std::ffi::c_void, count: usize) -> Self {
        unsafe {
            Self {
                _raw_ptr: raw_ptr,
                _aligned_data: std::slice::from_raw_parts_mut(raw_ptr as *mut T, count),
            }
        }
    }
}

impl<'a, T> std::ops::Index<usize> for MappedData<'a, T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        &self._aligned_data[index]
    }
}

impl<'a, T> std::ops::IndexMut<usize> for MappedData<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self._aligned_data[index]
    }
}

impl<'a, T> Drop for MappedData<'a, T> {
    fn drop(&mut self) {
        unsafe {
            std::ptr::copy(
                self._aligned_data.as_ptr(),
                self._raw_ptr as *mut T,
                self._aligned_data.len(),
            );
        }
    }
}
