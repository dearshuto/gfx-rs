use sjgfx_interface::{ShaderInfo, GpuAccess, BufferInfo, IDevice};

pub mod vulkano;
pub mod wgpu;

pub struct DeviceBuilder;
impl DeviceBuilder {
    pub fn new() -> DeviceBuilder {
        Self{}
    }
}
pub trait IDeviceBuilder<TDevice: IDevice> {
    fn build(&self) -> TDevice;
}

pub struct QueueBuilder;
impl QueueBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct BufferBuilder
{
    info: BufferInfo,
}

impl BufferBuilder{
    pub fn new() -> Self {
        Self{
            info: BufferInfo::new(),
        }
    }

    pub fn set_size(self, size: usize) -> Self {
        Self {
            info: self.info.set_size(size)
        }
    }

    pub fn set_gpu_access(self, gpu_access: GpuAccess) -> Self {
        Self {
            info: self.info.set_gpu_access_flags(gpu_access)
        }
    }

    pub fn get_buffer_info(&self) -> &BufferInfo {
        &self.info
    }
}

pub struct CommandBufferBuilder;
impl CommandBufferBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct SwapChainBuilder;
impl SwapChainBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct ShaderBuilder {
    compute_shader_binary: Option<Vec<u8>>,
    vertex_shader_binary: Option<Vec<u8>>,
    pixel_shader_binary: Option<Vec<u8>>,
}

impl ShaderBuilder {
    pub fn new() -> Self {
        Self {
            compute_shader_binary: None,
            vertex_shader_binary: None,
            pixel_shader_binary: None,
        }
    }

    pub fn set_compute_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: Some(shader_binary.to_vec()),
            vertex_shader_binary: self.vertex_shader_binary,
            pixel_shader_binary: self.pixel_shader_binary,
        }
    }

    pub fn set_vertex_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: self.compute_shader_binary,
            vertex_shader_binary: Some(shader_binary.to_vec()),
            pixel_shader_binary: self.pixel_shader_binary,
        }
    }

    pub fn set_pixel_shader_binary(self, shader_binary: &[u8]) -> Self {
        Self {
            compute_shader_binary: self.compute_shader_binary,
            vertex_shader_binary: self.vertex_shader_binary,
            pixel_shader_binary: Some(shader_binary.to_vec()),
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

pub struct FenceBuilder;
impl FenceBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct SemaphoreBuilder;
impl SemaphoreBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
