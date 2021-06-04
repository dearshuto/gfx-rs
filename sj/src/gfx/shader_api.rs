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
