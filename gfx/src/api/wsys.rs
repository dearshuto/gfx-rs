use sjgfx_wsys::{
    BufferWsys, ColorTargetViewWsys, CommandBufferWsys, DepthStencilViewWsys, DeviceWsys,
    FenceWsys, QueueWsys, SamplerWsys, SemaphoreWsys, ShaderWsys, SwapChainWsys, TextureViewWsys,
    TextureWsys, VertexStateWsys,
};

use super::IApi;

pub struct Wsys;
impl IApi for Wsys {
    type Buffer = BufferWsys;
    type ColorTargetView = ColorTargetViewWsys;
    type DepthStencilView = DepthStencilViewWsys;
    type Device = DeviceWsys;
    type Queue = QueueWsys;
    type CommandBuffer = CommandBufferWsys;
    type Fence = FenceWsys;
    type Shader = ShaderWsys;
    type Texture = TextureWsys;
    type TextureView = TextureViewWsys;
    type Sampler = SamplerWsys;
    type Semaphore = SemaphoreWsys;
    type SwapChain = SwapChainWsys;
    type VertexState = VertexStateWsys;
    type Instance = sjvi::web_sys::Instance;
    type Display = sjvi::web_sys::Display;
}
