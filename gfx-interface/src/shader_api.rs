use crate::IDevice;

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

pub trait IShader {
    type DeviceType: IDevice;

    fn new(device: &Self::DeviceType, info: &ShaderInfo) -> Self;
}
