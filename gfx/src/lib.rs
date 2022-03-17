use sjgfx_interface::{
    ICommandBuffer, IDevice, IFence, IQueue, ISemaphore, ISwapChain, ShaderInfo,
};
use sjgfx_wgpu::{
    CommandBufferWgpu, DeviceWgpu, FenceWgpu, QueueWgpu, SemaphoreWgpu, SwapChainWgpu,
};

pub mod vulkano;
pub mod wgpu;

pub trait IApi {
    type DeviceType: IDevice;
    type QueueType: IQueue;
    type CommandBufferType: ICommandBuffer<DeviceType = Self::DeviceType>;
    type FenceType: IFence<DeviceType = Self::DeviceType>;
    type SemaphoreType: ISemaphore<DeviceType = Self::DeviceType>;
    type SwapChainType: ISwapChain<DeviceType = Self::DeviceType>;
}

pub struct ApiWgpu;
impl IApi for ApiWgpu {
    type DeviceType = DeviceWgpu;
    type QueueType = QueueWgpu;
    type CommandBufferType = CommandBufferWgpu;
    type FenceType = FenceWgpu;
    type SemaphoreType = SemaphoreWgpu;
    type SwapChainType = SwapChainWgpu;
}

pub trait IDeviceBuilder<TDevice: IDevice> {
    fn build(&self) -> TDevice;
}
impl IDeviceBuilder<DeviceWgpu> for DeviceBuilder {
    fn build(&self) -> DeviceWgpu {
        todo!()
    }
}

pub struct DeviceBuilder;
impl DeviceBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

pub struct QueueBuilder;
impl QueueBuilder {
    pub fn new() -> Self {
        Self {}
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
