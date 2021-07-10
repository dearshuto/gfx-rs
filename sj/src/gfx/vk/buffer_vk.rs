use crate::gfx::buffer_api::{BufferInfo, IBufferImpl, MappedData};
use crate::gfx::Device;

use super::common::{Data1024, Data128, Data2048, Data256, Data4096, Data512, Data64};

pub struct BufferVk<'a> {
    _device: &'a Device,
    _buffer64: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data64>>>,
    _buffer128: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data128>>>,
    _buffer256: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data256>>>,
    _buffer512: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data512>>>,
    _buffer1024: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data1024>>>,
    _buffer2048: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data2048>>>,
    _buffer4096: Option<std::sync::Arc<vulkano::buffer::CpuAccessibleBuffer<Data4096>>>,
}

impl<'a> IBufferImpl<'a> for BufferVk<'a> {
    fn new(
        device: &'a Device,
        _info: &BufferInfo,
        _memory_pool: &'a crate::gfx::MemoryPool,
        _offset: i64,
        _size: u64,
    ) -> Self {
        let device_vk = device.to_data().get_device_impl();
        let buffer = unsafe {
            vulkano::buffer::CpuAccessibleBuffer::<Data128>::uninitialized(
                device_vk.clone(),
                vulkano::buffer::BufferUsage::all(),
                true,
            )
            .unwrap()
        };

        Self {
            _device: device,
            _buffer64: None,
            _buffer128: Some(buffer),
            _buffer256: None,
            _buffer512: None,
            _buffer1024: None,
            _buffer2048: None,
            _buffer4096: None,
        }
    }

    fn get_required_alignment(_device: &Device, _info: &BufferInfo) -> u64 {
        1
    }

    fn map<T>(&self) -> &mut T {
        todo!()
    }

    fn map_as_slice<U>(&self, _count: usize) -> &[U] {
        todo!()
    }

    fn map_as_slice_mut<U>(&self, count: usize) -> MappedData<U> {
        let buffer = self._buffer128.as_ref().unwrap().read().unwrap();
        let raw_ptr = buffer.buffer.as_ptr() as *mut std::ffi::c_void;
        MappedData::new(raw_ptr, count)
    }

    fn unmap(&self) {}

    fn flush_mapped_range(&self, _offset: i64, _size: u64) {}

    fn invalidate_mapped_range(&self, _offset: i64, _size: u64) {}
}
