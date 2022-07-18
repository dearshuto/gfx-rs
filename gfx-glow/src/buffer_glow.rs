use sjgfx_interface::{GpuAccess, IBuffer};
use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::BufferInfo;

use crate::DeviceGlow;

pub struct BufferGlow {
    gl: Arc<glow::Context>,
    buffer: glow::NativeBuffer,
    target: u32,
}

impl BufferGlow {
    pub fn new(device: &mut DeviceGlow, info: &BufferInfo) -> Self {
        device.make_current();
        let gl = device.clone_context();
        let buffer = unsafe { gl.create_buffer() }.unwrap();
        let target = Self::convert_to_target(&info.get_gpu_access_flags());
        unsafe { gl.bind_buffer(target, Some(buffer)) }
        unsafe { gl.buffer_data_size(target, info.get_size() as i32, glow::DYNAMIC_DRAW) }
        unsafe { gl.bind_buffer(target, None) }

        let error = unsafe { gl.get_error() };
        if error != glow::NO_ERROR {
            println!("BUFFER ERROR: {}", error);
        }

        Self { gl, buffer, target }
    }

    pub fn get_handle(&self) -> glow::NativeBuffer {
        self.buffer
    }

    fn convert_to_target(gpu_access: &GpuAccess) -> u32 {
        if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
            glow::ARRAY_BUFFER
        } else if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
            glow::UNIFORM_BUFFER
        } else if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
            glow::ELEMENT_ARRAY_BUFFER
        } else {
            todo!()
        }
    }
}

impl IBuffer for BufferGlow {
    type DeviceType = DeviceGlow;

    fn new(device: &mut Self::DeviceType, info: &BufferInfo) -> Self {
        BufferGlow::new(device, info)
    }

    fn map<T, F: Fn(&T)>(&self, func: F) {
        let target = self.target;
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
        let target = self.target;
        let offset = 0;
        let length = std::mem::size_of::<T>();
        let access = glow::MAP_WRITE_BIT;
        unsafe { self.gl.bind_buffer(target, Some(self.buffer)) }
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("BIND ERROR: {}", error);
        }

        let mapped_data = unsafe {
            self.gl
                .map_buffer_range(target, offset, length as i32, access)
        };
        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("MAP_MUT BUFFER_RANGE ERROR: {}", error);
        }

        let data = unsafe { (mapped_data as *mut T).as_mut().unwrap() };
        func(data);
        unsafe { self.gl.unmap_buffer(target) }

        let error = unsafe { self.gl.get_error() };
        if error != glow::NO_ERROR {
            println!("MAP_MUT ERROR: {}", error);
        }
    }

    fn map_as_slice<T, F: Fn(&[T])>(&self, func: F) {
        let target = self.target;
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
        let target = self.target;
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
