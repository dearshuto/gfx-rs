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
        memory_pool: &'a MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self;

    fn get_required_alignment(device: &Device, info: &BufferInfo) -> u64;

    fn map(&self);

    fn read<TType: 'static>(&self, action: fn(&mut TType));

    fn read_with_user_data<TType: 'static, TUserData>(
        &self,
        action: fn(&mut TType, Option<&mut TUserData>),
        user_data: Option<&mut TUserData>,
    );

    fn write<TType: 'static>(&self, action: fn(&mut TType));

    fn write_with_user_data<TType: 'static, TUserData>(
        &self,
        action: fn(&mut TType, Option<&mut TUserData>),
        user_data: Option<&mut TUserData>,
    );

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
    _marker: PhantomData<&'a ()>,
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

    pub fn map(&self) {
        self.buffer_impl.map();
    }

    pub fn read<TType: 'static>(&self, action: fn(&mut TType)) {
        self.buffer_impl.read(action);
    }

    pub fn read_with_user_data<TType: 'static, TUserData>(
        &self,
        action: fn(&mut TType, Option<&mut TUserData>),
        user_data: Option<&mut TUserData>,
    ) {
        self.buffer_impl.read_with_user_data(action, user_data);
    }

    pub fn write<TType: 'static>(&self, action: fn(&mut TType)) {
        self.buffer_impl.write(action);
    }

    pub fn write_with_user_data<TType: 'static, TUserData>(
        &self,
        action: fn(&mut TType, Option<&mut TUserData>),
        user_data: Option<&mut TUserData>,
    ) {
        self.buffer_impl.write_with_user_data(action, user_data);
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
