use std::sync::Arc;

use sjgfx_interface::IShader;
use web_sys::WebGlRenderingContext as GL;
use web_sys::{WebGlProgram, WebGl2RenderingContext, WebGlShader};

use crate::DeviceWsys;

pub struct ShaderWsys {
    gl: Arc<WebGl2RenderingContext>,
    program: WebGlProgram,
}

impl ShaderWsys {
    pub fn clone_program(&self) -> WebGlProgram {
        self.program.clone()
    }
}

impl IShader for ShaderWsys {
    type DeviceType = DeviceWsys;

    fn new(device: &mut Self::DeviceType, info: &sjgfx_interface::ShaderInfo) -> Self {
        let gl = device.clone_context();

        let program = gl.create_program().unwrap();
        let shader = Shader::new_as_graphics(
            gl.clone(),
            info.get_vertex_shader_source().unwrap(),
            info.get_pixel_shader_source().unwrap(),
        );
        shader.setup_program(&program);

        Self { gl, program }
    }
}

impl Drop for ShaderWsys {
    fn drop(&mut self) {
        self.gl.delete_program(Some(&self.program));
    }
}

struct Shader {
    gl: Arc<WebGl2RenderingContext>,
    vertex_shader: Option<WebGlShader>,
    pixel_shader: Option<WebGlShader>,
}

impl Shader {
    pub fn new_as_graphics(
        gl: Arc<WebGl2RenderingContext>,
        vertex_shader_source: &str,
        pixel_shader_source: &str,
    ) -> Self {
        // 頂点シェーダ
        let vertex_shader = gl.create_shader(GL::VERTEX_SHADER).unwrap();
        gl.shader_source(&vertex_shader, vertex_shader_source);
        gl.compile_shader(&vertex_shader);

        // ピクセルシェーダ
        let pixel_shader = gl.create_shader(GL::FRAGMENT_SHADER).unwrap();
        gl.shader_source(&pixel_shader, pixel_shader_source);
        gl.compile_shader(&pixel_shader);

        Self {
            gl,
            vertex_shader: Some(vertex_shader),
            pixel_shader: Some(pixel_shader),
        }
    }

    pub fn setup_program(&self, program: &WebGlProgram) {
        self.gl
            .attach_shader(program, self.vertex_shader.as_ref().unwrap());
        self.gl
            .attach_shader(program, self.pixel_shader.as_ref().unwrap());
        self.gl.link_program(program);
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        self.gl.delete_shader(self.vertex_shader.as_ref());
        self.gl.delete_shader(self.pixel_shader.as_ref());
    }
}
