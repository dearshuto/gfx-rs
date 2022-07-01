use sjgfx_interface::CommandBufferInfo;

use crate::{DeviceGlow, ShaderGlow};

pub struct CommandBufferGlow {
    shader: Option<glow::Program>,
}

impl CommandBufferGlow {
    pub fn new(_device: &DeviceGlow, _info: &CommandBufferInfo) -> Self {
        Self { shader: None }
    }

    pub fn begin(&mut self) {}
    pub fn end(&mut self) {}

    pub fn set_shader(&mut self, shader: &ShaderGlow) {
        self.shader = Some(shader.get_program());
    }

    pub fn try_get_program(&self) -> Option<glow::Program> {
        self.shader
    }
}
