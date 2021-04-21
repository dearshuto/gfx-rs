use super::super::buffer_api::{BufferInfo, IBufferImpl};
use super::super::Device;
use std::marker::PhantomData;

pub struct BufferImpl<'a> {
    buffer_impl: wgpu::Buffer,
    _marker: PhantomData<&'a i32>,
}

impl<'a> BufferImpl<'a> {
    pub fn map(&self) {
        let buffer_slice = self.buffer_impl.slice(..);
    }
}

impl<'a> IBufferImpl<'a> for BufferImpl<'a> {
    fn new(device: &'a Device, info: &BufferInfo) -> Self {
        let slice_size = 1024 * std::mem::size_of::<u32>();
        let size = slice_size as wgpu::BufferAddress;
        let buffer = device
            .to_data()
            .get_device()
            .create_buffer(&wgpu::BufferDescriptor {
                label: None,
                size,
                usage: wgpu::BufferUsage::MAP_READ | wgpu::BufferUsage::COPY_DST,
                mapped_at_creation: false,
            });

        BufferImpl {
            buffer_impl: buffer,
            _marker: PhantomData,
        }
    }
}
