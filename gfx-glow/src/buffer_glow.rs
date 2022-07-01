use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::BufferInfo;

use crate::DeviceGlow;

pub struct BufferGlow {
    gl: Arc<glow::Context>,
    buffer: glow::NativeBuffer,
}

impl BufferGlow {
    pub fn new(device: &DeviceGlow, _info: &BufferInfo) -> Self {
        let gl = device.clone_context();
        let buffer = unsafe{ gl.create_buffer() }.unwrap();
        Self { gl ,buffer }
    }

    pub fn map(&self, _offset: i32, _size: u32) {
        unsafe{ self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.buffer)) };
        unsafe{ self.gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, &[], glow::STATIC_DRAW) };
    }

    pub fn get_buffer(&self) -> glow::NativeBuffer {
        self.buffer
    }
}
