use sjgfx_interface::{BufferInfo, GpuAccess};

use crate::DeviceWgpu;

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

    pub fn map<T>(&self, func: fn(&T)) {
        let _result = self.buffer.slice(..).map_async(wgpu::MapMode::Write);

        self.device.get_device().poll(wgpu::Maintain::Wait);

        let ptr = self.buffer.slice(..).get_mapped_range().as_ptr();
        let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
        func(casted);
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
