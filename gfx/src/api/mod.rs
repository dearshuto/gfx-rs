use sjgfx_ash::{
    BufferAsh, ColorTargetViewAsh, CommandBufferAsh, DeviceAsh, FenceAsh, QueueAsh, SemaphoreAsh,
    ShaderAsh, SwapChainAsh, VertexStateAsh,
};
use sjgfx_interface::{
    IBuffer, IColorTargetView, ICommandBuffer, IDevice, IFence, IQueue, ISemaphore, IShader,
    ISwapChain, IVertexState,
};
use sjgfx_wgpu::{
    BufferWgpu, ColorTargetViewWgpu, CommandBufferWgpu, DeviceWgpu, FenceWgpu, QueueWgpu,
    SemaphoreWgpu, ShaderWgpu, SwapChainWgpu, VertexStateWgpu,
};

pub trait IApi {
    type Buffer: IBuffer<DeviceType = Self::Device>;
    type ColorTargetView: IColorTargetView<DeviceType = Self::Device>;
    type Device: IDevice;
    type Queue: IQueue<
        DeviceType = Self::Device,
        CommandBufferType = Self::CommandBuffer,
        SwapChainType = Self::SwapChain,
    >;
    type CommandBuffer: ICommandBuffer<
        DeviceType = Self::Device,
        ShaderType = Self::Shader,
        BufferType = Self::Buffer,
        ColorTargetViewType = Self::ColorTargetView,
        VertexStateType = Self::VertexState,
    >;
    type Fence: IFence<DeviceType = Self::Device>;
    type Shader: IShader<DeviceType = Self::Device>;
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
    type Device = DeviceAsh;
    type Queue = QueueAsh;
    type CommandBuffer = CommandBufferAsh;
    type Fence = FenceAsh;
    type Shader = ShaderAsh;
    type Semaphore = SemaphoreAsh;
    type SwapChain = SwapChainAsh;
    type VertexState = VertexStateAsh;
}

pub struct Wgpu;
impl IApi for Wgpu {
    type Buffer = BufferWgpu;
    type ColorTargetView = ColorTargetViewWgpu;
    type Device = DeviceWgpu;
    type Queue = QueueWgpu;
    type CommandBuffer = CommandBufferWgpu;
    type Fence = FenceWgpu;
    type Shader = ShaderWgpu;
    type Semaphore = SemaphoreWgpu;
    type SwapChain = SwapChainWgpu;
    type VertexState = VertexStateWgpu;
}
