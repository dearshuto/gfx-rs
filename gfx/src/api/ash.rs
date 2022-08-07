use sjgfx_ash::{
    BufferAsh, ColorTargetViewAsh, CommandBufferAsh, DepthStencilViewAsh, DeviceAsh, FenceAsh,
    QueueAsh, SamplerAsh, SemaphoreAsh, ShaderAsh, SwapChainAsh, TextureAsh, TextureViewAsh,
    VertexStateAsh,
};

use super::IApi;

pub struct Ash;
impl IApi for Ash {
    type Buffer = BufferAsh;
    type ColorTargetView = ColorTargetViewAsh;
    type DepthStencilView = DepthStencilViewAsh;
    type Device = DeviceAsh;
    type Queue = QueueAsh;
    type CommandBuffer = CommandBufferAsh;
    type Fence = FenceAsh;
    type Shader = ShaderAsh;
    type Sampler = SamplerAsh;
    type Semaphore = SemaphoreAsh;
    type SwapChain = SwapChainAsh;
    type Texture = TextureAsh;
    type TextureView = TextureViewAsh;
    type VertexState = VertexStateAsh;

    type Instance = sjvi::winit::Instance;
    type Display = sjvi::winit::Display<()>;
}
