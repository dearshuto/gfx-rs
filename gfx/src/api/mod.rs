use sjgfx_ash::{
    BufferAsh, ColorTargetViewAsh, CommandBufferAsh, DepthStencilViewAsh, DeviceAsh, FenceAsh,
    QueueAsh, SemaphoreAsh, ShaderAsh, SwapChainAsh, TextureAsh, TextureViewAsh, VertexStateAsh,
};
use sjgfx_interface::{
    IBuffer, IColorTargetView, ICommandBuffer, IDepthStencilView, IDevice, IFence, IQueue,
    ISemaphore, IShader, ISwapChain, ITexture, ITextureView, IVertexState,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DepthStencilViewWgpu, DeviceWgpu,
    FenceWgpu, QueueWgpu, SemaphoreWgpu, ShaderWgpu, SwapChainWgpu, TextureViewWgpu, TextureWgpu,
    VertexStateWgpu,
};

pub trait IApi {
    type Buffer: IBuffer<DeviceType = Self::Device>;
    type ColorTargetView: IColorTargetView<DeviceType = Self::Device>;
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
        TextureType = Self::Texture,
        TextureViewType = Self::TextureView,
        VertexStateType = Self::VertexState,
    >;
    type Fence: IFence<DeviceType = Self::Device>;
    type Shader: IShader<DeviceType = Self::Device>;
    type Texture: ITexture<DeviceType = Self::Device>;
    type TextureView: ITextureView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Semaphore: ISemaphore<DeviceType = Self::Device>;
    type SwapChain: ISwapChain<
        DeviceType = Self::Device,
        SemaphoreType = Self::Semaphore,
        ColorTargetViewType = Self::ColorTargetView,
    >;
    type VertexState: IVertexState<DeviceType = Self::Device>;
}

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
    type Semaphore = SemaphoreAsh;
    type SwapChain = SwapChainAsh;
    type Texture = TextureAsh;
    type TextureView = TextureViewAsh;
    type VertexState = VertexStateAsh;
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
    type Shader = ShaderWgpu;
    type Semaphore = SemaphoreWgpu;
    type SwapChain = SwapChainWgpu;
    type Texture = TextureWgpu;
    type TextureView = TextureViewWgpu;
    type VertexState = VertexStateWgpu;
}
