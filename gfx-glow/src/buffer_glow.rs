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

    fn new(device: &mut Self::DeviceType, info: &BufferInfo) -> Self {
        BufferGlow::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        let target = glow::ARRAY_BUFFER;
        let offset = 0;
        let length = std::mem::size_of::<T>();
        let access = glow::READ_BUFFER;
        unsafe { self.gl.bind_buffer(target, Some(self.buffer)) }

        let mapped_data = unsafe {
            self.gl
                .map_buffer_range(target, offset, length as i32, access)
        };
        let data = unsafe { (mapped_data as *const T).as_ref().unwrap() };
        func(data);
        unsafe { self.gl.unmap_buffer(target) }
    }

    fn map_mut<T, F: Fn(&mut T)>(&self, func: F) {
        let target = glow::ARRAY_BUFFER;
        let offset = 0;
        let length = std::mem::size_of::<T>();
        let access = glow::READ_BUFFER;
        unsafe { self.gl.bind_buffer(target, Some(self.buffer)) }

        let mapped_data = unsafe {
            self.gl
                .map_buffer_range(target, offset, length as i32, access)
        };
        let data = unsafe { (mapped_data as *mut T).as_mut().unwrap() };
        func(data);
        unsafe {
            self.gl
                .flush_mapped_buffer_range(glow::ARRAY_BUFFER, offset, length as i32)
        }
        unsafe { self.gl.unmap_buffer(target) }
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        let target = glow::ARRAY_BUFFER;
        let size = unsafe { self.gl.get_buffer_parameter_i32(target, glow::BUFFER_SIZE) };
        let length = (size as usize) / std::mem::size_of::<T>();
        let offset = 0;
        let access = glow::READ_WRITE;
        unsafe { self.gl.bind_buffer(target, Some(self.buffer)) }

        let mapped_data = unsafe {
            self.gl
                .map_buffer_range(target, offset, length as i32, access)
        };
        let data = unsafe { std::slice::from_raw_parts(mapped_data as *const T, length) };
        func(data);
        unsafe { self.gl.unmap_buffer(target) }
    }

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        let target = glow::ARRAY_BUFFER;
        unsafe { self.gl.bind_buffer(target, Some(self.buffer)) }
        let size = unsafe { self.gl.get_buffer_parameter_i32(target, glow::BUFFER_SIZE) };
        let length = (size as usize) / std::mem::size_of::<T>();
        let offset = 0;
        let access = glow::MAP_WRITE_BIT;

        let mapped_data = unsafe {
            self.gl
                .map_buffer_range(target, offset, length as i32, access)
        };
        let data = unsafe { std::slice::from_raw_parts_mut(mapped_data as *mut T, length) };
        func(data);
        unsafe { self.gl.unmap_buffer(target) }
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}
