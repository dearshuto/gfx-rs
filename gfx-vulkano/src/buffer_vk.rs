use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferCreateInfo, BufferUsage, Subbuffer},
    device::Device,
    memory::allocator::{AllocationCreateInfo, MemoryUsage, StandardMemoryAllocator},
    pipeline::graphics::vertex_input::VertexBuffersCollection,
};

use crate::{interop, DeviceVk};

pub struct BufferVk {
    device: Arc<Device>,
    buffer: Subbuffer<[u8]>,
}

impl BufferVk {
    pub fn new(device: &DeviceVk, info: &BufferInfo) -> Self {
        let memory_allocator =
            Arc::new(StandardMemoryAllocator::new_default(device.clone_device()));
        let buffer_usage = Self::convert_usage(&info.get_gpu_access_flags());
        let buffer = Buffer::from_iter(
            &memory_allocator,
            BufferCreateInfo {
                usage: buffer_usage,
                ..Default::default()
            },
            AllocationCreateInfo {
                usage: MemoryUsage::Upload,
                ..Default::default()
            },
            vec![0u8; info.get_size()],
        )
        .unwrap();

        Self {
            device: device.clone_device(),
            buffer,
        }
    }

    pub fn map<T, F: Fn(&T)>(&self, func: F) {
        let data = self.buffer.read().unwrap();
        let ptr = data.as_ptr();
        let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
        func(casted);
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let ptr = self.buffer.mapped_ptr().unwrap().as_ptr();
        let casted = unsafe { (ptr as *mut T).as_mut().unwrap() };
        func(casted);
    }

    pub fn map_as_array<T, F: Fn(&[T])>(&self, func: F) {
        let ptr = self.buffer.mapped_ptr().unwrap().as_ptr() as *mut T;
        let size = (self.buffer.len() as usize) / std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts::<T>(ptr, size) };
        func(slice);
    }

    pub fn map_as_array_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        let ptr = self.buffer.mapped_ptr().unwrap().as_ptr() as *mut T;
        let size = (self.buffer.len() as usize) / std::mem::size_of::<T>();
        let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, size) };
        func(slice);
    }

    pub fn view(&self) -> BufferView {
        BufferView::new(self)
    }

    fn convert_usage(gpu_access: &GpuAccess) -> BufferUsage {
        interop::convert_usage(gpu_access)
    }

    fn clone_buffer(&self) -> Subbuffer<[u8]> {
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
    pub buffer: Subbuffer<[u8]>,
}

impl BufferView {
    fn new(buffer: &BufferVk) -> Self {
        Self {
            device: buffer.device.clone(),
            buffer: buffer.clone_buffer(),
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            device: self.device.clone(),
            buffer: self.buffer.clone(),
        }
    }
}

impl VertexBuffersCollection for BufferView {
    fn into_vec(self) -> Vec<Subbuffer<[u8]>> {
        vec![]
    }
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
