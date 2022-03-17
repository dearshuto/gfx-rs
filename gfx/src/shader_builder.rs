use sjgfx_interface::{IShader, ShaderInfo};

pub struct TShaderBuilder<T: IShader> {
    compute_shader_binary: Option<Vec<u8>>,
    vertex_shader_binary: Option<Vec<u8>>,
    pixel_shader_binary: Option<Vec<u8>>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IShader> TShaderBuilder<T> {
    pub fn new() -> Self {
        Self {
            compute_shader_binary: None,
            vertex_shader_binary: None,
            pixel_shader_binary: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::DeviceType) -> T {
        let shader_info = self.create_info();
        T::new(device, &shader_info)
    }

    pub fn set_compute_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: Some(shader_binary.to_vec()),
            vertex_shader_binary: self.vertex_shader_binary,
            pixel_shader_binary: self.pixel_shader_binary,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set_vertex_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: self.compute_shader_binary,
            vertex_shader_binary: Some(shader_binary.to_vec()),
            pixel_shader_binary: self.pixel_shader_binary,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set_pixel_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: self.compute_shader_binary,
            vertex_shader_binary: self.vertex_shader_binary,
            pixel_shader_binary: Some(shader_binary.to_vec()),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn create_info(&self) -> ShaderInfo {
        let mut shader_info = ShaderInfo::new();

        // 演算シェーダ
        if let Some(compute_shader_binary) = &self.compute_shader_binary {
            shader_info = shader_info.set_compute_shader_binary(&compute_shader_binary);
        }

        // 頂点シェーダ
        if let Some(vertex_shader_binary) = &self.vertex_shader_binary {
            shader_info = shader_info.set_vertex_shader_binary(&vertex_shader_binary);
        }

        // ピクセルシェーダ
        if let Some(pixel_shader_binary) = &self.pixel_shader_binary {
            shader_info = shader_info.set_pixel_shader_binary(&pixel_shader_binary);
        }

        shader_info
    }
}
