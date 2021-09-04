use super::Device;
use std::marker::PhantomData;

pub struct ShaderInfo<'a> {
    _compute_shader_binary: Option<&'a [u8]>,
    _vertex_shader_binary: Option<&'a [u8]>,
    _pixel_shader_binary: Option<&'a [u8]>,
}

impl<'a> ShaderInfo<'a> {
    pub fn new() -> Self {
        Self {
            _compute_shader_binary: None,
            _vertex_shader_binary: None,
            _pixel_shader_binary: None,
        }
    }

    pub fn get_vertex_shader_binary(&self) -> &Option<&'a [u8]> {
        &self._vertex_shader_binary
    }

    pub fn set_vertex_shader_binary(mut self, shader_binary: &'a [u8]) -> Self {
        self._vertex_shader_binary = Some(shader_binary);
        self
    }

    pub fn get_pixel_shader_binary(&self) -> &Option<&'a [u8]> {
        &self._pixel_shader_binary
    }

    pub fn set_pixel_shader_binary(mut self, shader_binary: &'a [u8]) -> Self {
        self._pixel_shader_binary = Some(shader_binary);
        self
    }

    pub fn get_compute_shader_binary(&self) -> &Option<&[u8]> {
        &self._compute_shader_binary
    }

    pub fn set_compute_shader_binary(mut self, shader_binary: &'a [u8]) -> Self {
        self._compute_shader_binary = Some(shader_binary);
        self
    }

    pub fn get_shader_binary(&self) -> &'a [u8] {
        self._compute_shader_binary.unwrap()
    }

    pub fn set_shader_binary(mut self, shader_binary: &'a [u8]) -> Self {
        self._compute_shader_binary = Some(shader_binary);
        self
    }
}

pub trait IShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self;
}

pub struct TShaderInterface<'a, T: 'a>
where
    T: IShaderImpl<'a>,
{
    shader_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IShaderImpl<'a>> TShaderInterface<'a, T> {
    pub fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        Self {
            shader_impl: T::new(device, info),
            _marker: PhantomData,
        }
    }

    pub fn to_data(&'a self) -> &'a T {
        &self.shader_impl
    }
}

#[cfg(test)]
mod tests {
    use crate::gfx::{Device, DeviceInfo, Shader};

    use super::ShaderInfo;

    #[test]
    fn initialize_graphics_shader() {
        let vertex_shader_source = &include_str!("test.vs");
        let pixel_shader_source = &include_str!("test.fs");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();

        let vertex_shader_binary = compiler
            .compile_into_spirv(
                vertex_shader_source,
                shaderc::ShaderKind::Vertex,
                "shader.glsl",
                "main",
                Some(&options),
            )
            .unwrap();

        let pixel_shader_binary = compiler
            .compile_into_spirv(
                pixel_shader_source,
                shaderc::ShaderKind::Fragment,
                "shader.glsl",
                "main",
                Some(&options),
            )
            .unwrap();

        let device = Device::new(&DeviceInfo::new());
        let _shader = Shader::new(
            &device,
            &ShaderInfo::new()
                .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
                .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
        );
    }

    #[test]
    fn initialize_compute_shader() {
        let compute_shader_source = &include_str!("test.glsl");
        let mut compiler = shaderc::Compiler::new().unwrap();
        let options = shaderc::CompileOptions::new().unwrap();

        let compute_shader_binary = compiler
            .compile_into_spirv(
                compute_shader_source,
                shaderc::ShaderKind::Compute,
                "shader.glsl",
                "main",
                Some(&options),
            )
            .unwrap();

        let device = Device::new(&DeviceInfo::new());
        let _shader = Shader::new(
            &device,
            &ShaderInfo::new().set_compute_shader_binary(compute_shader_binary.as_binary_u8()),
        );
    }
}
