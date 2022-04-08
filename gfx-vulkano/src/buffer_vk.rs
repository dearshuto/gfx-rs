use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer},
    DeviceSize, pipeline::graphics::vertex_input::VertexBuffersCollection,
};

use crate::DeviceVk;

pub struct BufferVk {
    buffer: Arc<CpuAccessibleBuffer<[u8]>>,
}

impl BufferVk {
    pub fn new(device: &DeviceVk, info: &BufferInfo) -> Self {
        let length = info.get_size() / std::mem::size_of::<u8>();
        let buffer = unsafe {
            CpuAccessibleBuffer::<[u8]>::uninitialized_array(
                device.clone_device(),
                length as DeviceSize,
                Self::convert_usage(&info.get_gpu_access_flags()),
                true, /*host_cached*/
            )
                .unwrap()
        };

        Self {
            buffer
        }
    }

    pub fn map<T: 'static, F: Fn(&T)>(&self, func: F) {
        let mapped_data = self.buffer
            .read()
            .map(|x| {
                let ptr = x.as_ptr();
                let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
                casted
            })
            .unwrap();
        func(&mapped_data);
    }

    pub fn map_mut<T: 'static, F: Fn(&mut T)>(&mut self, func: F) {
        let mapped_data = self.buffer
            .write()
            .map(|mut x| {
                let ptr = x.as_mut_ptr();
                let casted = unsafe { (ptr as *mut T).as_mut().unwrap() };
                casted
            })
            .unwrap();
        func(mapped_data);
    }

    pub fn map_as_array<T: 'static, F: Fn(&[T])>(&self, func: F) {
        let mapped_data = self.buffer
            .read()
            .map(|x| {
                let ptr = x.as_ptr() as *const T;
                let size = x.len();
                let slice = unsafe { std::slice::from_raw_parts::<T>(ptr, size) };
                slice
            })
            .unwrap();
        func(&mapped_data);
    }

    pub fn map_as_array_mut<T: 'static, F: Fn(&mut [T])>(&self, func: F) {
        let mapped_data = self.buffer
            .write()
            .map(|mut x| {
                let ptr = x.as_mut_ptr() as *mut T;
                let size = x.len();
                let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, size) };
                slice
            })
            .unwrap();
        func(mapped_data);
    }

    pub fn view(&self) -> BufferView {
        BufferView::new(self)
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

    fn clone_buffer(&self) -> Arc<CpuAccessibleBuffer<[u8]>> {
        self.buffer.clone()
    }
}

impl IBuffer for BufferVk {
    type DeviceType = DeviceVk;

    fn new(_device: &Self::DeviceType, _info: &BufferInfo) -> Self {
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

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}

pub struct BufferView {
    buffer: Arc<CpuAccessibleBuffer<[u8]>>,
}

impl BufferView {
    fn new(buffer: &BufferVk) -> Self {
        Self {
            buffer: buffer.clone_buffer()
        }
    }

    pub fn clone_buffer(&self) -> Arc<dyn BufferAccess> {
        self.buffer.clone()
    }


    pub fn clone_buffer_as<T: Send + Sync + 'static>(&self) -> Arc<CpuAccessibleBuffer<T>> {
        todo!()
    }

    pub fn clone_buffer_view(&self) -> Arc<dyn BufferAccess> {
        self.buffer.clone()
    }

    pub fn clone_vertex_buffer_as<T: Send + Sync + 'static>(
        &self,
    ) -> Arc<CpuAccessibleBuffer<[T]>> {
        todo!()
    }

    pub fn clone(&self) -> Self {
        Self {buffer: self.buffer.clone()
        }
    }
}

unsafe impl VertexBuffersCollection for BufferView {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
        vec![self.buffer.clone()]
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{IDevice, DeviceInfo, BufferInfo, GpuAccess};

    use crate::{DeviceVk, BufferVk};

    #[test]
    fn new_as_constant_buffer() {
        new_impl(GpuAccess::CONSTANT_BUFFER);
    }

    #[test]
    fn new_as_vertex_buffer() {
        new_impl(GpuAccess::VERTEX_BUFFER);
    }

    #[test]
    fn new_as_index_buffer() {
        new_impl(GpuAccess::INDEX_BUFFER);
    }

    #[test]
    fn new_as_unordered_access_buffer() {
        new_impl(GpuAccess::UNORDERED_ACCESS_BUFFER);
    }

    fn new_impl(gpu_access: GpuAccess) {
        let device = DeviceVk::new(&DeviceInfo::new());
        let _ = BufferVk::new(&device, &BufferInfo::new().set_size(64).set_gpu_access_flags(gpu_access));
    }

    #[test]
    fn map_as_slice() {
        let device = DeviceVk::new(&DeviceInfo::new());
        let buffer = BufferVk::new(&device, &BufferInfo::new().set_size(std::mem::size_of::<f32>() * 4).set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER));
        buffer.map_as_array_mut(|x: &mut [f32]|{
            x[0] = 10.0;
            x[1] = 20.0;
            x[2] = 30.0;
            x[3] = 40.0;
        });
        buffer.map_as_array(|x: &[f32]| {
            assert!(x[0] == 10.0);
            assert!(x[1] == 20.0);
            assert!(x[2] == 30.0);
            assert!(x[3] == 40.0);
        });
    }
}
