use crate::gfx::buffer_api::MappedData;
use crate::gfx::GpuAddress;

use super::super::buffer_api::{BufferInfo, IBufferImpl};
use super::super::{Device, GpuAccess, MemoryPool};
use super::gpu_address_wgpu::GpuAddressWgpu;
use std::marker::PhantomData;

pub struct BufferImpl<'a> {
    _device: &'a Device,
    _buffer_impl: wgpu::Buffer,
    _size: usize,
    _marker: PhantomData<&'a i32>,
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a> {
    fn new(
        device: &'a Device,
        info: &BufferInfo,
        _memory_pool: Option <&'a MemoryPool>,
        _offset: i64,
        _size: u64,
    ) -> Self {
        let slice_size = info.get_size();
        let size = slice_size as wgpu::BufferAddress;
        let buffer = device
            .to_data()
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size,
                usage: info.get_as_usage(),
                mapped_at_creation: false,
            });

        BufferImpl {
            _device: device,
            _buffer_impl: buffer,
            _size: info.get_size() as usize,
            _marker: PhantomData,
        }
    }

    fn get_required_alignment(_device: &Device, _info: &BufferInfo) -> u64 {
        0b1
    }

    fn map<T>(&self) -> &mut T {
        todo!();
    }

    fn map_as_slice<U>(&self, _count: usize) -> &[U] {
        todo!();
    }

    fn map_as_slice_mut<U>(&self, count: usize) -> MappedData<U> {
        let _result = self._buffer_impl.slice(..).map_async(wgpu::MapMode::Write);
        //let _result = self._buffer_impl.slice(..).map_async(wgpu::MapMode::Read);
        self._device
            .to_data()
            .get_device()
            .poll(wgpu::Maintain::Wait);

        let ptr = self._buffer_impl.slice(..).get_mapped_range_mut().as_ptr();
        MappedData::new(ptr as *mut std::ffi::c_void, count)
    }

    fn unmap(&self) {}

    fn flush_mapped_range(&self, _offset: i64, _size: u64) {}

    fn invalidate_mapped_range(&self, _offset: i64, _size: u64) {}

    fn get_gpu_address(&self) -> GpuAddress {
        let instance = GpuAddressWgpu::new(self);
        GpuAddress::new(instance)
    }
}

impl<'a> BufferImpl<'a> {
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self._buffer_impl
    }

    pub fn get_buffer_mut(&mut self) -> &mut wgpu::Buffer {
        &mut self._buffer_impl
    }
}

impl BufferInfo {
    pub fn get_as_usage(&self) -> wgpu::BufferUsages {
        let gpu_access = self.get_gpu_access_flags();

        let mut result = wgpu::BufferUsages::empty();
        if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
            result |= wgpu::BufferUsages::VERTEX;
        }
        if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
            result |= wgpu::BufferUsages::INDEX;
        }
        if gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
            result |= wgpu::BufferUsages::STORAGE;
			result |= wgpu::BufferUsages::MAP_READ;
			result |= wgpu::BufferUsages::MAP_WRITE;
        }
        if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
            result |= wgpu::BufferUsages::UNIFORM;
        }
        if gpu_access.contains(GpuAccess::READ) {
            result |= wgpu::BufferUsages::MAP_READ;
        }
        if gpu_access.contains(GpuAccess::WRITE) {
            result |= wgpu::BufferUsages::MAP_WRITE;
        }

        result |= wgpu::BufferUsages::COPY_SRC;
        result |= wgpu::BufferUsages::COPY_DST;

        result
    }
}
