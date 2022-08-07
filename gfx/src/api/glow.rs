use sjgfx_glow::{
    BufferGlow, ColorTargetViewGlow, CommandBufferGlow, DepthStencilViewGlow, DeviceGlow,
    FenceGlow, QueueGlow, SamplerGlow, SemaphoerGlow, ShaderGlow, SwapChainGlow, TextureGlow,
    TextureViewGlow, VertexStateGlow,
};

use super::IApi;

pub struct Glow;
impl IApi for Glow {
    type Buffer = BufferGlow;
    type ColorTargetView = ColorTargetViewGlow;
    type DepthStencilView = DepthStencilViewGlow;
    type Device = DeviceGlow;
    type Queue = QueueGlow;
    type CommandBuffer = CommandBufferGlow;
    type Fence = FenceGlow;
    type Shader = ShaderGlow;
    type Texture = TextureGlow;
    type TextureView = TextureViewGlow;
    type Sampler = SamplerGlow;
    type Semaphore = SemaphoerGlow;
    type SwapChain = SwapChainGlow;
    type VertexState = VertexStateGlow;
    type Instance = sjvi::glutin::Instance;
    type Display = sjvi::glutin::Display;
}
