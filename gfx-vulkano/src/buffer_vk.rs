use sjgfx_interface::BufferInfo;
use std::sync::Arc;
use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer};

use std::any::Any;

use crate::DeviceVk;

pub struct BufferVk {
    buffer: Arc<dyn BufferAccess>,
    any: Arc<dyn Any + Send + Sync>,
}

impl BufferVk {
    pub fn new<T: 'static>(device: &DeviceVk, _info: &BufferInfo) -> Self
    where
        CpuAccessibleBuffer<T>: BufferAccess,
    {
        let buffer = unsafe {
            CpuAccessibleBuffer::<T>::uninitialized(
                device.clone_device(),
                BufferUsage::all(),
                true, /*host_cached*/
            )
            .unwrap()
        };
        Self {
            buffer: buffer.clone(),
            any: buffer.clone(),
        }
    }

    pub fn new_as_array<T: Send + Sync + 'static>(device: &DeviceVk, _info: &BufferInfo) -> Self {
        let buffer = unsafe {
            CpuAccessibleBuffer::<[T]>::uninitialized_array(
                device.clone_device(),
                128, /*len*/
                BufferUsage::all(),
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

    pub fn map<T: 'static>(&self, func: fn(&T)) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<T>>().unwrap();
        let mapped_data = buffer.read().unwrap();
        func(&mapped_data);
    }

    pub fn map_as_array<T: 'static>(&self, func: fn(&[T])) {
        let buffer = self.any.downcast_ref::<CpuAccessibleBuffer<[T]>>().unwrap();
        let mapped_data = buffer.read().unwrap();
        func(&mapped_data);
    }
}
