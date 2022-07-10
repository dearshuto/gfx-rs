use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};
use std::sync::Arc;
use vulkano::{
    buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    device::{Device, DeviceOwned},
    pipeline::graphics::vertex_input::VertexBuffersCollection,
    DeviceSize,
};

use crate::DeviceVk;

pub struct BufferVk {
    device: Arc<Device>,
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
            device: device.clone_device(),
            buffer,
        }
    }

    pub fn map<T, F: Fn(&T)>(&self, func: F) {
        let mapped_data = self
            .buffer
            .read()
            .map(|x| {
                let ptr = x.as_ptr();
                let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
                casted
            })
            .unwrap();
        func(&mapped_data);
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let mapped_data = self
            .buffer
            .write()
            .map(|mut x| {
                let ptr = x.as_mut_ptr();
                let casted = unsafe { (ptr as *mut T).as_mut().unwrap() };
                casted
            })
            .unwrap();
        func(mapped_data);
    }

    pub fn map_as_array<T, F: Fn(&[T])>(&self, func: F) {
        let mapped_data = self
            .buffer
            .read()
            .map(|x| {
                let ptr = x.as_ptr() as *const T;
                let size = x.len() / std::mem::size_of::<T>();
                let slice = unsafe { std::slice::from_raw_parts::<T>(ptr, size) };
                slice
            })
            .unwrap();
        func(&mapped_data);
    }

    pub fn map_as_array_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        let mapped_data = self
            .buffer
            .write()
            .map(|mut x| {
                let ptr = x.as_mut_ptr() as *mut T;
                let size = x.len() / std::mem::size_of::<T>();
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
            ..Default::default()
        };

        result
    }

    fn clone_buffer(&self) -> Arc<CpuAccessibleBuffer<[u8]>> {
        self.buffer.clone()
    }
}

impl IBuffer for BufferVk {
    type DeviceType = DeviceVk;

    fn new(device: &mut Self::DeviceType, info: &BufferInfo) -> Self {
        Self::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        self.map(func);
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        self.map_mut(func);
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        self.map_as_array(func);
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        self.map_as_array_mut(func);
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}

pub struct BufferView {
    device: Arc<Device>,
    buffer: Arc<CpuAccessibleBuffer<[u8]>>,
}

impl BufferView {
    fn new(buffer: &BufferVk) -> Self {
        Self {
            device: buffer.device.clone(),
            buffer: buffer.clone_buffer(),
        }
    }

    pub fn clone_buffer(&self) -> Arc<dyn BufferAccess> {
        self.buffer.clone()
    }

    pub fn clone_buffer_view(&self) -> Arc<dyn BufferAccess> {
        self.buffer.clone()
    }

    pub fn clone_index_buffer(&self) -> Arc<CpuAccessibleBuffer<[u32]>> {
        todo!()
    }

    pub fn clone(&self) -> Self {
        Self {
            device: self.device.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl VertexBuffersCollection for BufferView {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
        vec![self.buffer.clone()]
    }
}

unsafe impl DeviceOwned for BufferView {
    fn device(&self) -> &Arc<vulkano::device::Device> {
        &self.device
    }
}

unsafe impl BufferAccess for BufferView {
    fn inner(&self) -> vulkano::buffer::BufferInner {
        self.buffer.inner()
    }

    fn size(&self) -> DeviceSize {
        self.buffer.size()
    }

    fn conflict_key(&self) -> (u64, u64) {
        self.buffer.conflict_key()
    }

    fn try_gpu_lock(
        &self,
        exclusive_access: bool,
        queue: &vulkano::device::Queue,
    ) -> Result<(), vulkano::sync::AccessError> {
        self.buffer.try_gpu_lock(exclusive_access, queue)
    }

    unsafe fn increase_gpu_lock(&self) {
        self.buffer.increase_gpu_lock()
    }

    unsafe fn unlock(&self) {
        self.buffer.unlock()
    }
}

unsafe impl TypedBufferAccess for BufferView {
    type Content = [u32];
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{BufferInfo, DeviceInfo, GpuAccess};

    use crate::{BufferVk, DeviceVk};

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
        let _ = BufferVk::new(
            &device,
            &BufferInfo::new()
                .set_size(64)
                .set_gpu_access_flags(gpu_access),
        );
    }

    #[test]
    fn map_as_slice() {
        let device = DeviceVk::new(&DeviceInfo::new());
        let buffer = BufferVk::new(
            &device,
            &BufferInfo::new()
                .set_size(std::mem::size_of::<f32>() * 4)
                .set_gpu_access_flags(GpuAccess::CONSTANT_BUFFER),
        );
        buffer.map_as_array_mut(|x: &mut [f32]| {
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
