use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu,
    FenceWgpu, QueueWgpu, SamplerWgpu, SemaphoreWgpu, ShaderWgpu, SwapChainWgpu, TextureViewWgpu,
    TextureWgpu, VertexStateWgpu,
};

use super::IApi;

pub struct Wgpu;
impl IApi for Wgpu {
    type Buffer = BufferWgpu;
    type ColorTargetView = ColorTargetViewWgpu;
    type DepthStencilView = DepthStencilViewWgpu;
    type Device = DeviceWgpu;
    type Queue = QueueWgpu;
    type CommandBuffer = CommandBufferWgpu;
    type Fence = FenceWgpu;
    type Sampler = SamplerWgpu;
    type Shader = ShaderWgpu;
    type Semaphore = SemaphoreWgpu;
    type SwapChain = SwapChainWgpu;
    type Texture = TextureWgpu;
    type TextureView = TextureViewWgpu;
    type VertexState = VertexStateWgpu;

    type Instance = sjvi::winit::Instance;
    type Display = sjvi::winit::Display<()>;
}
