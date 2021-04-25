use super::{Device, MemoryPool};
use std::marker::PhantomData;

pub struct BufferInfo {}

impl BufferInfo {
    pub fn new() -> Self {
        BufferInfo {}
    }
}

pub trait IBufferImpl<'a> {
    fn new(device: &'a Device, info: &BufferInfo, memory_pool: &'a MemoryPool, offset: i64, size: u64) -> Self;

	fn map<T>(&self) -> &mut T;

	fn unmap(&self);
}

pub struct TBufferInterface<'a, T: 'a>
where
    T: IBufferImpl<'a>,
{
    buffer_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IBufferImpl<'a>> TBufferInterface<'a, T> {
    pub fn new(device: &'a Device, info: &BufferInfo, memory_pool: &'a MemoryPool, offset: i64, size: u64) -> Self {
        Self {
            buffer_impl: T::new(device, info, memory_pool, offset, size),
            _marker: PhantomData,
        }
    }

	pub fn map<U>(&self) -> &mut U
	{
		self.buffer_impl.map()
	}

	pub fn unmap(&self)
	{
		self.buffer_impl.unmap();
	}
}
