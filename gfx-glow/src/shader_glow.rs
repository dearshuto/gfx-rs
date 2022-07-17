use std::sync::Arc;

use glow::{Context, HasContext};
use sjgfx_interface::{IShader, ShaderInfo};

use crate::DeviceGlow;

pub struct ShaderGlow {
    context: Arc<glow::Context>,
    program: glow::Program,
}

impl ShaderGlow {
    pub fn new(device: &mut DeviceGlow, info: &ShaderInfo) -> Self {
        device.make_current();

        let gl = device.clone_context();
        let program = unsafe { gl.create_program() }.unwrap();

        if let Some(compute_shader_binary) = info.get_compute_shader_binary() {
            let shader = Shader::new_as_compute(gl.clone(), compute_shader_binary);
            shader.setup_program(program);
        } else {
            let vertex_shader_source = info.get_vertex_shader_source().unwrap();
            let pixel_shader_source = info.get_pixel_shader_source().unwrap();
            let shader =
                Shader::new_as_graphics(gl.clone(), vertex_shader_source, pixel_shader_source);
            shader.setup_program(program);
        }

        Self {
            context: gl,
            program,
        }
    }

    pub fn get_program(&self) -> glow::Program {
        self.program
    }
}

impl IShader for ShaderGlow {
    type DeviceType = DeviceGlow;

    fn new(device: &mut Self::DeviceType, info: &ShaderInfo) -> Self {
        Self::new(device, info)
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
    shader: Option<glow::Shader>,
    vertex_shader: Option<glow::Shader>,
    fragment_shader: Option<glow::Shader>,
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

        Self {
            context,
            shader: Some(shader),
            vertex_shader: None,
            fragment_shader: None,
        }
    }

    pub fn setup_program(self, program: glow::Program) {
        if let Some(compute_shader) = self.shader {
            unsafe { self.context.attach_shader(program, compute_shader) }
        } else {
            unsafe {
                self.context
                    .attach_shader(program, self.vertex_shader.unwrap())
            }
            unsafe {
                self.context
                    .attach_shader(program, self.fragment_shader.unwrap())
            }
        }

        unsafe { self.context.link_program(program) }
        if !unsafe { self.context.get_program_link_status(program) } {
            unsafe { panic!("{}", self.context.get_program_info_log(program)) }
        }

        if let Some(compute_shader) = self.shader {
            unsafe { self.context.detach_shader(program, compute_shader) }
        } else {
            unsafe {
                self.context
                    .detach_shader(program, self.vertex_shader.unwrap())
            }
            unsafe {
                self.context
                    .detach_shader(program, self.fragment_shader.unwrap())
            }
        }
    }

    pub fn new_as_graphics(
        context: Arc<Context>,
        vertex_shader_source: &str,
        fragment_shader_source: &str,
    ) -> Self {
        let vertex_shader = unsafe { context.create_shader(glow::VERTEX_SHADER) }.unwrap();
        unsafe { context.shader_source(vertex_shader, vertex_shader_source) };
        unsafe { context.compile_shader(vertex_shader) };
        unsafe {
            if !context.get_shader_compile_status(vertex_shader) {
                panic!("{}", context.get_shader_info_log(vertex_shader));
            }
        }

        let fragment_shader = unsafe { context.create_shader(glow::FRAGMENT_SHADER) }.unwrap();
        unsafe { context.shader_source(fragment_shader, fragment_shader_source) };
        unsafe { context.compile_shader(fragment_shader) };
        unsafe {
            if !context.get_shader_compile_status(fragment_shader) {
                panic!("{}", context.get_shader_info_log(fragment_shader));
            }
        }

        Self {
            context: context.clone(),
            shader: None,
            vertex_shader: Some(vertex_shader),
            fragment_shader: Some(fragment_shader),
        }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        if let Some(compute_shader) = self.shader {
            unsafe { self.context.delete_shader(compute_shader) }
        }

        if let Some(vertex_shader) = self.vertex_shader {
            unsafe { self.context.delete_shader(vertex_shader) }
        }

        if let Some(fragment_shader) = self.fragment_shader {
            unsafe { self.context.delete_shader(fragment_shader) }
        }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, ShaderInfo};

    use crate::{DeviceGlow, ShaderGlow};

    #[test]
    fn new_compute_shader() {
        let mut device = DeviceGlow::new(&DeviceInfo::new());
        let _shader = ShaderGlow::new(
            &mut device,
            &ShaderInfo::new().set_compute_shader_binary(&[]),
        );
    }
}
