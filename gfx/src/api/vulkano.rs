use sjgfx_vulkano::{
    BufferVk, ColorTargetViewVk, CommandBufferVk, DepthStencilViewVk, DeviceVk, FenceVk, QueueVk,
    SamplerVk, SemaphoreVk, ShaderVk, SwapChainVk, TextureViewVk, TextureVk, VertexStateVk,
};

use super::IApi;

pub struct Vulkano;
impl IApi for Vulkano {
    type Buffer = BufferVk;
    type ColorTargetView = ColorTargetViewVk;
    type DepthStencilView = DepthStencilViewVk;
    type Device = DeviceVk;
    type Queue = QueueVk;
    type CommandBuffer = CommandBufferVk;
    type Fence = FenceVk;
    type Shader = ShaderVk;
    type Texture = TextureVk;
    type TextureView = TextureViewVk;
    type Sampler = SamplerVk;
    type Semaphore = SemaphoreVk;
    type SwapChain = SwapChainVk;
    type VertexState = VertexStateVk;

    type Instance = sjvi::winit::Instance;
    type Display = sjvi::winit::Display<()>;
}
