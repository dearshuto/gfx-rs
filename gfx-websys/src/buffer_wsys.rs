use std::sync::Arc;

use sjgfx_interface::{GpuAccess, IBuffer};
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlBuffer, WebGl2RenderingContext};

use crate::DeviceWsys;

pub struct BufferWsys {
    gl: Arc<WebGl2RenderingContext>,
    buffer: WebGlBuffer,
    target: u32,
}

impl BufferWsys {
    pub fn clone_buffer(&self) -> WebGlBuffer {
        self.buffer.clone()
    }

    fn convert_to_target(gpu_access: &GpuAccess) -> u32 {
        if gpu_access.contains(GpuAccess::VERTEX_BUFFER) {
            GL::ARRAY_BUFFER
        } else if gpu_access.contains(GpuAccess::CONSTANT_BUFFER) {
            todo!()
        } else if gpu_access.contains(GpuAccess::INDEX_BUFFER) {
            GL::ELEMENT_ARRAY_BUFFER
        } else {
            todo!()
        }
    }
}

impl IBuffer for BufferWsys {
    type DeviceType = DeviceWsys;

    fn new(device: &mut Self::DeviceType, info: &sjgfx_interface::BufferInfo) -> Self {
        let gl = device.clone_context();

        let target = Self::convert_to_target(&info.get_gpu_access_flags());
        let buffer = gl.create_buffer().unwrap();
        gl.bind_buffer(target, Some(&buffer));

        let mut data = Vec::new();
        data.resize(info.get_size(), 0);
        gl.buffer_data_with_u8_array(target, &data, GL::DYNAMIC_DRAW);
        gl.bind_buffer(target, None);

        Self { gl, buffer, target }
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

    fn map_as_slice_mut<T, F: Fn(&mut [T])>(&self, func: F) {
        self.gl.bind_buffer(self.target, Some(&self.buffer));

        let size = self
            .gl
            .get_buffer_parameter(self.target, GL::BUFFER_SIZE)
            .as_f64()
            .unwrap() as usize;
        let length = size / std::mem::size_of::<T>();
        let mut buffer = Vec::new();
        buffer.resize(size, 0);

        let casted_data =
            unsafe { std::slice::from_raw_parts_mut(buffer.as_ptr() as *mut T, length) };
        func(casted_data);

        self.gl
            .buffer_data_with_u8_array(self.target, &buffer, GL::STATIC_DRAW);
    }

    fn flush_mapped_range(&self, _offset: isize, _size: usize) {}

    fn invalidate_mapped_range(&self, _offset: isize, _size: usize) {}
}

impl Drop for BufferWsys {
    fn drop(&mut self) {
        self.gl.delete_buffer(Some(&self.buffer));
    }
}
