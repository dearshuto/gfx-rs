use crate::gfx::buffer_api::{BufferInfo, IBufferImpl};
use crate::gfx::Device;
use std::ops::{Deref, DerefMut};
use vulkano::buffer::CpuAccessibleBuffer;

pub struct BufferVk<'a, TType> {
    _device: &'a Device,
    _buffer: std::sync::Arc<CpuAccessibleBuffer<TType>>,
}

impl<'a, TType> IBufferImpl<'a, TType> for BufferVk<'a, TType>
where
    TType: 'static,
{
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        _memory_pool: &'a crate::gfx::MemoryPool,
        _offset: i64,
        size: u64,
    ) -> Self {
        assert!(info.get_size() <= size);

        let device_vk = device.to_data().get_device_impl();
        let buffer = unsafe {
            vulkano::buffer::CpuAccessibleBuffer::<TType>::uninitialized(
                device_vk.clone(),
                vulkano::buffer::BufferUsage::all(),
                true,
            )
            .unwrap()
        };

        Self {
            _device: device,
            _buffer: buffer,
        }
    }

    fn get_required_alignment(_device: &Device, _info: &BufferInfo) -> u64 {
        1
    }

    fn map(&self) {}

    fn read<F: FnMut(&TType)>(&self, mut action: F) {
        let mapped_data = self._buffer.read().unwrap();
        action(mapped_data.deref());
    }

    fn write<F: FnMut(&mut TType)>(&self, mut action: F) {
        let mut mapped_data = self._buffer.write().unwrap();
        action(mapped_data.deref_mut());
    }

    fn unmap(&self) {}

    fn flush_mapped_range(&self, _offset: i64, _size: u64) {}

    fn invalidate_mapped_range(&self, _offset: i64, _size: u64) {}
}
