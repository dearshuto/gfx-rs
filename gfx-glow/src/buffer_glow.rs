use sjgfx_interface::IBuffer;
use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::BufferInfo;

use crate::DeviceGlow;

pub struct BufferGlow {
    gl: Arc<glow::Context>,
    buffer: glow::NativeBuffer,
}

impl BufferGlow {
    pub fn new(device: &mut DeviceGlow, info: &BufferInfo) -> Self {
        device.make_current();
        let gl = device.clone_context();
        let buffer = unsafe { gl.create_buffer() }.unwrap();
        let target = glow::ARRAY_BUFFER;
        unsafe { gl.bind_buffer(target, Some(buffer)) }
        unsafe { gl.buffer_data_size(target, info.get_size() as i32, glow::STATIC_DRAW) }
        unsafe { gl.bind_buffer(target, None) }
        Self { gl, buffer }
    }

    pub fn get_handle(&self) -> glow::NativeBuffer {
        self.buffer
    }
}

impl IBuffer for BufferGlow {
    type DeviceType = DeviceGlow;

    fn new(device: &Self::DeviceType, info: &BufferInfo) -> Self {
        BufferGlow::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        self.map(func);
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        self.map_mut(func);
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        let size = self.size / std::mem::size_of::<T>();
        self.map_as_slice(size + 1, func);
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        let size = self.size / std::mem::size_of::<T>();
        self.map_as_slice_mut(size, func);
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}
