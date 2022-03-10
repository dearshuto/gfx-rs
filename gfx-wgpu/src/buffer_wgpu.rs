use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};

use crate::{DeviceWgpu, GpuAddressWgpu};

pub struct BufferWgpu<'a> {
    device: &'a DeviceWgpu,
    buffer: wgpu::Buffer,
}

impl<'a> BufferWgpu<'a> {
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn new(device: &'a DeviceWgpu, info: &BufferInfo) -> Self {
        let buffer = device.get_device().create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: info.get_size() as u64,
            usage: Self::convert(&info.get_gpu_access_flags()),
            mapped_at_creation: false,
        });

        Self { device, buffer }
    }

    pub fn map<T, F: Fn(&T)>(&self, func: F) {
        let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

        self.device.get_device().poll(wgpu::Maintain::Wait);

        let ptr = self.buffer.slice(..).get_mapped_range().as_ptr();
        let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
        func(casted);
        self.buffer.unmap();
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

        self.device.get_device().poll(wgpu::Maintain::Wait);

        let ptr = self.buffer.slice(..).get_mapped_range_mut().as_mut_ptr();
        let casted = unsafe { (ptr as *mut T).as_mut().unwrap() };
        func(casted);
        self.buffer.unmap();
    }

    pub fn map_as_slice<T, F: Fn(&[T])>(&self, size: usize, func: F) {
        let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

        self.device.get_device().poll(wgpu::Maintain::Wait);

        let ptr = self
            .buffer
            .slice(..)
            .get_mapped_range()
            .as_ptr()
            .cast::<T>();
        let slice = unsafe { std::slice::from_raw_parts::<T>(ptr, size) };
        func(slice);
        self.buffer.unmap();
    }

    pub fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, size: usize, func: F) {
        let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

        self.device.get_device().poll(wgpu::Maintain::Wait);

        let ptr = self
            .buffer
            .slice(..)
            .get_mapped_range_mut()
            .as_mut_ptr()
            .cast::<T>();
        let slice = unsafe { std::slice::from_raw_parts_mut::<T>(ptr, size) };
        func(slice);
        self.buffer.unmap();
    }

    pub fn get_gpu_address(&self) -> GpuAddressWgpu {
        GpuAddressWgpu::new(self)
    }

    fn convert(gpu_access: &GpuAccess) -> wgpu::BufferUsages {
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

        result |= wgpu::BufferUsages::MAP_READ;
        result |= wgpu::BufferUsages::MAP_WRITE;
        result |= wgpu::BufferUsages::COPY_SRC;
        result |= wgpu::BufferUsages::COPY_DST;

        result
    }
}

impl<'a> IBuffer<'a> for BufferWgpu<'a> {
    type DeviceType = DeviceWgpu;

    fn new(device: &'a Self::DeviceType, info: &BufferInfo) -> Self {
        BufferWgpu::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        self.map(func);
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        self.map_mut(func);
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        self.map_as_slice(64, func);
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        self.map_as_slice_mut(64, func);
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}
