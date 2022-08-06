use sjgfx_ash::{
    BufferAsh, ColorTargetViewAsh, CommandBufferAsh, DepthStencilViewAsh, DeviceAsh, FenceAsh,
    QueueAsh, SamplerAsh, SemaphoreAsh, ShaderAsh, SwapChainAsh, TextureAsh, TextureViewAsh,
    VertexStateAsh,
};
use sjgfx_glow::{
    BufferGlow, ColorTargetViewGlow, CommandBufferGlow, DepthStencilViewGlow, DeviceGlow,
    FenceGlow, QueueGlow, SamplerGlow, SemaphoerGlow, ShaderGlow, SwapChainGlow, TextureGlow,
    TextureViewGlow, VertexStateGlow,
};
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
use sjgfx_wsys::{
    BufferWsys, ColorTargetViewWsys, CommandBufferWsys, DepthStencilViewWsys, DeviceWsys,
    FenceWsys, QueueWsys, SamplerWsys, SemaphoreWsys, ShaderWsys, SwapChainWsys, TextureViewWsys,
    TextureWsys, VertexStateWsys,
};
use sjvi::{IDisplay, IInstance};

pub trait IApi {
    type Buffer: IBuffer<DeviceType = Self::Device>;
    type ColorTargetView: IColorTargetView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type DepthStencilView: IDepthStencilView<DeviceType = Self::Device, TextureType = Self::Texture>;
    type Device: IDevice<Display = Self::Display>;
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

    type Instance: IInstance<Display = Self::Display>;
    type Display: IDisplay;
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
    type Sampler = SamplerAsh;
    type Semaphore = SemaphoreAsh;
    type SwapChain = SwapChainAsh;
    type Texture = TextureAsh;
    type TextureView = TextureViewAsh;
    type VertexState = VertexStateAsh;

    type Instance = sjvi::winit::Instance;
    type Display = sjvi::winit::Display<()>;
}

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
