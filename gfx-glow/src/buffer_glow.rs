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
