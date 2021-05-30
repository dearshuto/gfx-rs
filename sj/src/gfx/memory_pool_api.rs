use super::super::gfx::MemoryPoolProperty;
use super::Device;
use std::marker::PhantomData;

pub struct MemoryPoolInfo {
    _memory_pool_property: MemoryPoolProperty,
    _size: u64,
}

impl MemoryPoolInfo {
    pub fn new() -> Self {
        Self {
            _memory_pool_property: MemoryPoolProperty::empty(),
            _size: 0,
        }
    }

    pub fn get_size(&self) -> u64 {
        self._size
    }

    pub fn set_size(mut self, size: u64) -> Self {
        self._size = size;
        self
    }

    pub fn get_memory_pool_property(&self) -> MemoryPoolProperty {
        self._memory_pool_property
    }

    pub fn set_memory_pool_property(mut self, memory_pool_property: MemoryPoolProperty) -> Self {
        self._memory_pool_property = memory_pool_property;
        self
    }
}

pub trait IMemoryPoolImpl<'a> {
    fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self;
}

pub struct TMemoryPoolInterface<'a, T>
where
    T: IMemoryPoolImpl<'a>,
{
    _memory_pool_impl: T,
    _marker: PhantomData<&'a u32>,
}

impl<'a, T: IMemoryPoolImpl<'a>> TMemoryPoolInterface<'a, T> {
    pub fn new(device: &'a Device, info: &MemoryPoolInfo) -> Self {
        Self {
            _memory_pool_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._memory_pool_impl
    }
}
