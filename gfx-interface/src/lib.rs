mod buffer_api;
mod color_target_view_api;
mod command_buffer_api;
mod enums;
mod fence_api;
mod queue_api;
mod shader_api;
mod swap_chain_api;
mod texture_api;
mod viewport_scissor_state_api;
mod vertex_state_api;

pub use buffer_api::{BufferInfo, IBuffer};
pub use color_target_view_api::ColorTargetViewInfo;
pub use command_buffer_api::CommandBufferInfo;
pub use enums::{GpuAccess, ImageFormat, PrimitiveTopology};
pub use fence_api::FenceInfo;
pub use queue_api::{QueueInfo, IQueue};
pub use shader_api::ShaderInfo;
pub use swap_chain_api::SwapChainInfo;
pub use texture_api::{TextureArrayRange, TextureCopyRegion, TextureInfo, TextureSubresource, TextureSubresourceRange, BufferTextureCopyRegion};
pub use viewport_scissor_state_api::ViewportScissorStateInfo;
pub use vertex_state_api::{VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo};

pub struct DeviceInfo{}

impl DeviceInfo {
    pub fn new() -> Self {
        Self{}
    }
}

pub trait IDevice {
    fn new(info: &DeviceInfo) -> Self;
}
