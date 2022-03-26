pub mod api;
mod buffer_builder;
mod command_buffer_builder;
mod device_builder;
mod queue_builder;
mod shader_builder;
mod swap_chain_builder;
mod texture_builder;
mod texture_view_builder;
mod vertex_state_builder;

pub use buffer_builder::TBufferBuilder;
pub use command_buffer_builder::TCommandBufferBuilder;
pub use device_builder::TDeviceBuilder;
pub use queue_builder::TQueueBuilder;
pub use shader_builder::TShaderBuilder;
pub use swap_chain_builder::TSwapChainBuilder;
pub use texture_builder::TTextureBuilder;
pub use texture_view_builder::TTextureViewBuilder;
pub use vertex_state_builder::TVertexStateBuilder;

pub mod vulkano;
pub mod wgpu;

#[cfg(feature = "backend-ash")]
type BackendApi = api::Ash;

#[cfg(all(not(feature = "backend-ash"), feature = "backend-wgpu"))]
type BackendApi = api::Wgpu;

pub type BufferBuilder = TBufferBuilder<BackendApi>;
pub type CommandBufferBuilder = TCommandBufferBuilder<BackendApi>;
pub type DeviceBuilder = TDeviceBuilder<BackendApi>;
pub type QueueBuilder = TQueueBuilder<BackendApi>;
pub type ShaderBuilder = TShaderBuilder<BackendApi>;
pub type SwapChainBuilder = TSwapChainBuilder<BackendApi>;
pub type VertexStateBuilder = TVertexStateBuilder<BackendApi>;

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
