use std::sync::Arc;

use glow::HasContext;
use sjgfx_interface::QueueInfo;

use crate::{CommandBufferGlow, DeviceGlow};

pub struct QueueGlow {
    gl: Arc<glow::Context>,
}

impl QueueGlow {
    pub fn new(device: &DeviceGlow, _info: &QueueInfo) -> Self {
        Self {
            gl: device.clone_context(),
        }
    }

    pub fn execute(&mut self, command_buffer: &CommandBufferGlow) {
        unsafe { self.gl.clear(glow::COLOR_BUFFER_BIT) }

        // シェーダ
        unsafe { self.gl.use_program(command_buffer.try_get_program()) }
    }
}
