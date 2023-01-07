use std::sync::Arc;

use sjgfx_interface::{BufferInfo, GpuAccess, IBuffer};
use uuid::Uuid;
use wgpu::util::DeviceExt;

use crate::{DeviceWgpu, GpuAddressWgpu};

pub struct BufferWgpu {
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    buffer: Arc<wgpu::Buffer>,
    size: usize,
    id: Uuid,
}

impl BufferWgpu {
    pub fn get_buffer(&self) -> &wgpu::Buffer {
        &self.buffer
    }

    pub fn close_buffer(&self) -> Arc<wgpu::Buffer> {
        self.buffer.clone()
    }

    pub fn new(device: &DeviceWgpu, info: &BufferInfo) -> Self {
        let queue = device.clone_queue();
        let device = device.close_device();

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: info.get_size() as u64,
            usage: Self::convert(&info.get_gpu_access_flags()),
            mapped_at_creation: false,
        });

        Self {
            device,
            queue,
            buffer: Arc::new(buffer),
            size: info.get_size(),
            id: Uuid::new_v4(),
        }
    }

    pub fn new_init(device: &DeviceWgpu, info: &BufferInfo, data: &[u8]) -> Self {
        let queue = device.clone_queue();
        let device = device.close_device();
        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: data,
            usage: Self::convert(&info.get_gpu_access_flags()),
        });

        Self {
            device,
            queue,
            buffer: Arc::new(buffer),
            size: info.get_size(),
            id: Uuid::new_v4(),
        }
    }

    pub fn get_id(&self) -> &Uuid {
        &self.id
    }

    pub fn view(&self) -> BufferView {
        BufferView {
            buffer: self.buffer.clone(),
            id: self.id,
        }
    }

    pub fn map<T, F: Fn(&T)>(&self, func: F) {
        let (sender, _receiver) = futures_intrusive::channel::shared::oneshot_channel();
        let _result = self
            .buffer
            .slice(..)
            .map_async(wgpu::MapMode::Write, move |v| sender.send(v).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

        let ptr = self.buffer.slice(..).get_mapped_range().as_ptr();
        let casted = unsafe { (ptr as *const T).as_ref().unwrap() };
        func(casted);
        self.buffer.unmap();
    }

    pub fn map_mut<T, F: Fn(&mut T)>(&self, func: F)
    where
        T: Default + Clone,
    {
        let mut temp_buffer = Default::default();
        func(&mut temp_buffer);
        let data = unsafe {
            std::slice::from_raw_parts(
                (&temp_buffer as *const T) as *const u8,
                std::mem::size_of::<T>(),
            )
        };

        self.queue.write_buffer(&self.buffer, 0, &data);
    }

    pub fn map_as_slice<T, F: Fn(&[T])>(&self, size: usize, func: F) {
        let (sender, _receiver) = futures_intrusive::channel::shared::oneshot_channel();
        let _result = self
            .buffer
            .slice(..)
            .map_async(wgpu::MapMode::Write, move |v| sender.send(v).unwrap());

        self.device.poll(wgpu::Maintain::Wait);

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

    pub fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, size: usize, func: F)
    where
        T: Default + Clone,
    {
        let mut buffer = vec![Default::default(); size];
        func(&mut buffer);
        let data = unsafe {
            std::slice::from_raw_parts(
                (buffer.as_mut_ptr() as *const T) as *const u8,
                size * std::mem::size_of::<T>(),
            )
        };

        self.queue.write_buffer(&self.buffer, 0, &data);
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

        result |= wgpu::BufferUsages::COPY_DST;

        // バッファを CPU に map するためのフラグ
        // 現状の実装の web 版だと map できないので分岐
        #[cfg(not(target_arch = "wasm32"))]
        {
            result |= wgpu::BufferUsages::MAP_READ;
            result |= wgpu::BufferUsages::MAP_WRITE;
            result |= wgpu::BufferUsages::COPY_SRC;
        }

        result
    }
}

pub struct BufferView {
    pub buffer: Arc<wgpu::Buffer>,
    pub id: Uuid,
}

impl IBuffer for BufferWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &mut Self::DeviceType, info: &BufferInfo) -> Self {
        Self::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        self.map(func);
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, _func: F) {
        todo!()
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        let size = self.size / std::mem::size_of::<T>();
        self.map_as_slice(size + 1, func);
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, _func: F) {
        todo!()
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}
