use sjgfx_interface::{
    IBuffer, IColorTargetView, ICommandBuffer, IDepthStencilView, IDevice, IFence, IQueue,
    ISampler, ISemaphore, IShader, ISwapChain, ITexture, ITextureView, IVertexState,
};
use sjgfx_vulkano::{
    BufferVk, ColorTargetViewVk, CommandBufferVk, DepthStencilViewVk, DeviceVk, FenceVk, QueueVk,
    SamplerVk, SemaphoreVk, ShaderVk, SwapChainVk, TextureViewVk, TextureVk, VertexStateVk,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu,
    FenceWgpu, QueueWgpu, SamplerWgpu, SemaphoreWgpu, ShaderWgpu, SwapChainWgpu, TextureViewWgpu,
    TextureWgpu, VertexStateWgpu,
};

pub trait IApi {
    type Buffer: IBuffer<DeviceType = Self::Device>;
    type ColorTargetView: IColorTargetView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type DepthStencilView: IDepthStencilView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Device: IDevice;
    type Queue: IQueue<
        DeviceType = Self::Device,
        CommandBufferType = Self::CommandBuffer,
        SwapChainType = Self::SwapChain,
    >;
    type CommandBuffer: ICommandBuffer<
        DeviceType = Self::Device,
        DepthStencilViewType = Self::DepthStencilView,
        ShaderType = Self::Shader,
        BufferType = Self::Buffer,
        ColorTargetViewType = Self::ColorTargetView,
        SamplerType = Self::Sampler,
        TextureType = Self::Texture,
        TextureViewType = Self::TextureView,
        VertexStateType = Self::VertexState,
    >;
    type Fence: IFence<DeviceType = Self::Device>;
    type Shader: IShader<DeviceType = Self::Device>;
    type Texture: ITexture<DeviceType = Self::Device>;
    type TextureView: ITextureView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Sampler: ISampler<DeviceType = Self::Device>;
    type Semaphore: ISemaphore<DeviceType = Self::Device>;
    type SwapChain: ISwapChain<
        DeviceType = Self::Device,
        SemaphoreType = Self::Semaphore,
        ColorTargetViewType = Self::ColorTargetView,
    >;
    type VertexState: IVertexState<DeviceType = Self::Device>;
}

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
}

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
}
