use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    DeviceSize,
};

use std::any::Any;

use crate::DeviceVk;

pub struct BufferVk {
    buffer: Arc<dyn BufferAccess>,
    any: Arc<dyn Any + Send + Sync>,
}

impl BufferVk {
    pub fn new<T: 'static>(device: &DeviceVk, info: &BufferInfo) -> Self
    where
        CpuAccessibleBuffer<T>: BufferAccess,
    {
        let buffer = unsafe {
            CpuAccessibleBuffer::<T>::uninitialized(
                device.clone_device(),
                Self::convert_usage(&info.get_gpu_access_flags()),
                true, /*host_cached*/
            )
            .unwrap()
        };
        Self {
            buffer: buffer.clone(),
            any: buffer.clone(),
        }
    }

    pub fn new_as_array<T: Send + Sync + 'static>(device: &DeviceVk, info: &BufferInfo) -> Self {
        let length = info.get_size() / std::mem::size_of::<T>();
        let buffer = unsafe {
            CpuAccessibleBuffer::<[T]>::uninitialized_array(
                device.clone_device(),
                length as DeviceSize,
                Self::convert_usage(&info.get_gpu_access_flags()),
                true, /*host_cached*/
            )
            .unwrap()
        };
        Self {
            buffer: buffer.clone(),
            any: buffer.clone(),
        }
    }

    pub fn clone_buffer(&self) -> Arc<dyn BufferAccess> {
        self.buffer.clone()
    }

    pub fn clone_buffer_as<T: Send + Sync + 'static>(&self) -> Arc<CpuAccessibleBuffer<T>> {
        let concrete_buffer = self
            .any
            .clone()
            .downcast::<CpuAccessibleBuffer<T>>()
            .unwrap();
        concrete_buffer
    }

    pub fn clone_vertex_buffer_as<T: Send + Sync + 'static>(
        &self,
    ) -> Arc<CpuAccessibleBuffer<[T]>> {
        let concrete_buffer = self
            .any
            .clone()
            .downcast::<CpuAccessibleBuffer<[T]>>()
            .unwrap();
        concrete_buffer
    }

    pub fn map<T: 'static, F: Fn(&T)>(&self, func: F) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<T>>().unwrap();
        let mapped_data = buffer.read().unwrap();
        func(&mapped_data);
    }

    pub fn map_mut<T: 'static, F: Fn(&mut T)>(&self, func: F) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<T>>().unwrap();
        let mut mapped_data = buffer.write().unwrap();
        func(&mut mapped_data);
    }

    pub fn map_as_array<T: 'static, F: Fn(&[T])>(&self, func: F) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<[T]>>().unwrap();
        let mapped_data = buffer.read().unwrap();
        func(&mapped_data);
    }

    pub fn map_as_array_mut<T: 'static, F: Fn(&mut [T])>(&self, func: F) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<[T]>>().unwrap();
        let mut mapped_data = buffer.write().unwrap();
        func(&mut mapped_data);
    }

    fn convert_usage(gpu_access: &GpuAccess) -> BufferUsage {
        let is_uniform_buffer = gpu_access.contains(GpuAccess::CONSTANT_BUFFER);
        let is_storage_buffer = gpu_access.contains(GpuAccess::UNORDERED_ACCESS_BUFFER);
        let is_index_buffer = gpu_access.contains(GpuAccess::INDEX_BUFFER);
        let is_vertex_buffer = gpu_access.contains(GpuAccess::VERTEX_BUFFER);
        let is_indirect_buffer = gpu_access.contains(GpuAccess::INDIRECT_BUFFER);

        let result = BufferUsage {
            transfer_source: false,
            transfer_destination: false,
            uniform_texel_buffer: false,
            storage_texel_buffer: false,
            uniform_buffer: is_uniform_buffer,
            storage_buffer: is_storage_buffer,
            index_buffer: is_index_buffer,
            vertex_buffer: is_vertex_buffer,
            indirect_buffer: is_indirect_buffer,
            device_address: true,
        };

        result
    }
}

impl<'a> IBuffer<'a> for BufferVk {
    type DeviceType = DeviceVk;

    fn new(_device: &'a Self::DeviceType, _info: &BufferInfo) -> Self {
        todo!()
    }

    fn map<T, F: Fn(&T)>(&self, _func: F) {
        todo!()
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, _func: F) {
        todo!()
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, _func: F) {
        todo!()
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, _func: F) {
        todo!()
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {
        
    }

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {

    }
}
