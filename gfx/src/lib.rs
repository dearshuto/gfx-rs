use sjgfx_interface::{ICommandBuffer, IDevice, IFence, IQueue, ISemaphore, ISwapChain};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, FenceWgpu, QueueWgpu, SemaphoreWgpu, ShaderWgpu,
    SwapChainWgpu, VertexStateWgpu,
};

mod buffer_builder;
mod command_buffer_builder;
mod device_builder;
mod queue_builder;
mod shader_builder;
mod swap_chain_builder;
mod vertex_state_builder;

pub use buffer_builder::TBufferBuilder;
pub use command_buffer_builder::TCommandBufferBuilder;
pub use device_builder::TDeviceBuilder;
pub use queue_builder::TQueueBuilder;
pub use shader_builder::TShaderBuilder;
pub use swap_chain_builder::TSwapChainBuilder;
pub use vertex_state_builder::TVertexStateBuilder;

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

pub type BufferBuilder = TBufferBuilder<BufferWgpu>;
pub type CommandBufferBuilder = TCommandBufferBuilder<CommandBufferWgpu>;
pub type DeviceBuilder = TDeviceBuilder<DeviceWgpu>;
pub type QueueBuilder = TQueueBuilder<QueueWgpu>;
pub type ShaderBuilder = TShaderBuilder<ShaderWgpu>;
pub type SwapChainBuilder = TSwapChainBuilder<SwapChainWgpu>;
pub type VertexStateBuilder = TVertexStateBuilder<VertexStateWgpu>;

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
