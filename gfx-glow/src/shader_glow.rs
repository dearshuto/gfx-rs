use std::sync::Arc;

use glow::{Context, HasContext};
use sjgfx_interface::ShaderInfo;

use crate::DeviceGlow;

pub struct ShaderGlow {
    context: Arc<glow::Context>,
    program: glow::Program,
}

impl ShaderGlow {
    pub fn new(device: &DeviceGlow, info: &ShaderInfo) -> Self {
        let gl = device.clone_context();

        let program = unsafe { gl.create_program() }.unwrap();

        if let Some(compute_shader_binary) = info.get_compute_shader_binary() {
            let shader = Shader::new_as_compute(gl.clone(), compute_shader_binary);
            shader.setup_program(program);
        }

        Self {
            context: gl,
            program,
        }
    }
}

impl Drop for ShaderGlow {
    fn drop(&mut self) {
        // プログラムの破棄
        unsafe { self.context.delete_program(self.program) }
    }
}

struct Shader {
    context: Arc<Context>,
    shader: glow::Shader,
}

impl Shader {
    pub fn new_as_compute(context: Arc<Context>, _shader_binary: &[u8]) -> Self {
        let shader = unsafe { context.create_shader(glow::VERTEX_SHADER) }.unwrap();

        let source = r"
#version 450

layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

void main()
{
}
";

        unsafe { context.shader_source(shader, source) }
        unsafe { context.compile_shader(shader) }
        unsafe {
            if !context.get_shader_compile_status(shader) {
                panic!("{}", context.get_shader_info_log(shader));
            }
        }

        Self { context, shader }
    }

    pub fn setup_program(self, program: glow::Program) {
        unsafe { self.context.attach_shader(program, self.shader) }
        unsafe { self.context.link_program(program) }
        if !unsafe { self.context.get_program_link_status(program) } {
            unsafe { panic!("{}", self.context.get_program_info_log(program)) }
        }

        unsafe { self.context.detach_shader(program, self.shader) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { self.context.delete_shader(self.shader) }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, ShaderInfo};

    use crate::{DeviceGlow, ShaderGlow};

    #[test]
    fn new_compute_shader() {
        let device = DeviceGlow::new(&DeviceInfo::new());
        let _shader = ShaderGlow::new(&device, &ShaderInfo::new().set_compute_shader_binary(&[]));
    }
}
