use bitflags;

mod blend_state_api;
mod buffer_api;
mod color_target_view_api;
mod command_buffer_api;
mod depth_stencil_state_api;
mod depth_stencil_view_api;
//mod descriptor_pool_api;
mod device_api;
mod gpu_address_api;
mod memory_pool_api;
mod pipeline_api;
mod queue_api;
mod rasterizer_state_api;
mod shader_api;
mod swap_chain_api;
mod texture_api;
mod vertex_state_api;
mod viewport_scissor_state_api;

use self::blend_state_api::TBlendState;
use self::buffer_api::TBufferInterface;
use self::color_target_view_api::TColorTargetView;
use self::command_buffer_api::TCommandBufferInterface;
use self::depth_stencil_state_api::TDepthStencilState;
use self::depth_stencil_view_api::TDepthStencilView;
//use self::descriptor_pool_api::TDescriptorInterface;
use self::device_api::TDeviceInterface;
use self::gpu_address_api::TGpuAddressInterface;
use self::memory_pool_api::TMemoryPoolInterface;
use self::pipeline_api::TPipelineInterface;
use self::queue_api::TQueueInterface;
use self::rasterizer_state_api::TRasterizerStateInterface;
use self::shader_api::TShaderInterface;
use self::swap_chain_api::TSwapChain;
use self::texture_api::TTexture;
use self::vertex_state_api::TVertexState;
use self::viewport_scissor_state_api::TViewportScissorState;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;

#[cfg(feature = "backend_ash")]
mod ash;

// BlendState
pub use self::blend_state_api::BlendStateInfo;
pub use self::blend_state_api::BlendTargetStateInfo;

#[cfg(feature = "backend_ash")]
type BlendStateImpl = self::ash::blend_state_ash::BlendStateImpl;

pub type BlendState = TBlendState<BlendStateImpl>;
//

// Buffer  -----------------------------------------------------------
pub use self::buffer_api::BufferInfo;

#[cfg(feature = "backend_vulkano")]
type BufferImpl<'a> = self::vk::buffer_vk::BufferImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type BufferImpl<'a> = self::wgpu::buffer_wgpu::BufferImpl<'a>;

#[cfg(feature = "backend_ash")]
type BufferImpl<'a> = self::ash::buffer_ash::BufferImpl<'a>;

pub type Buffer<'a> = TBufferInterface<'a, BufferImpl<'a>>;
// -------------------------------------------------------------------

// ColorTargetView
pub use self::color_target_view_api::ColorTargetViewInfo;

#[cfg(feature = "backend_ash")]
type ColorTargetViewImpl<'a> = self::ash::color_target_view_ash::ColorTargetViewImpl<'a>;

pub type ColorTargetView<'a> = TColorTargetView<'a, ColorTargetViewImpl<'a>>;
//

// CommandBuffer -----------------------------------------------------
pub use self::command_buffer_api::CommandBufferInfo;

#[cfg(feature = "backend_vulkano")]
type CommandBufferImpl<'a> = self::vk::command_buffer_vk::CommandBufferImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type CommandBufferImpl<'a> = self::wgpu::command_buffer_wgpu::CommandBuffer<'a>;

#[cfg(feature = "backend_ash")]
type CommandBufferImpl<'a> = self::ash::command_buffer_ash::CommandBufferImpl<'a>;

pub type CommandBuffer<'a> = TCommandBufferInterface<'a, CommandBufferImpl<'a>>;
// -------------------------------------------------------------------

//
//type DescriptorPoolImpl = self::wgpu::descriptor_pool_wgpu::DescriptorPoolImpl;
//pub type DescriptorPool = TDescriptorInterface<DescriptorPoolImpl>;
//

// Device --------------------------------------------------------------------
pub use self::device_api::DeviceInfo;

#[cfg(feature = "backend_vulkano")]
type DeviceImpl = self::vk::device_vk::DeviceImpl;

#[cfg(feature = "backend_wgpu")]
type DeviceImpl = self::wgpu::device_wgpu::DeviceImpl;

#[cfg(feature = "backend_ash")]
type DeviceImpl = self::ash::device_ash::DeviceImpl;

pub type Device = TDeviceInterface<DeviceImpl>;
//-----------------------------------------------------------------------------

// DepthStencilState
pub use self::depth_stencil_state_api::DepthStencilStateInfo;

#[cfg(feature = "backend_ash")]
type DepthStencilStateImpl = self::ash::depth_stencil_state_ash::DepthStencilStateImpl;

pub type DepthStencilState = TDepthStencilState<DepthStencilStateImpl>;
//

// DepthStencilView
pub use self::depth_stencil_view_api::DepthStencilViewInfo;

#[cfg(feature = "backend_ash")]
type DepthStencilViewImpl = self::ash::depth_stencil_view_ash::DepthStencilViewImpl;

pub type DepthStencilView = TDepthStencilView<DepthStencilViewImpl>;
//

//
#[cfg(feature = "backend_ash")]
type GpuAddressImpl<'a> = self::ash::gpu_address_ash::GpuAddressImpl<'a>;

pub type GpuAddress<'a> = TGpuAddressInterface<'a, GpuAddressImpl<'a>>;
//

// MemoryPool
pub use self::memory_pool_api::MemoryPoolInfo;

#[cfg(feature = "backend_ash")]
type MemoryPoolImpl<'a> = self::ash::memory_pool_ash::MemoryPoolImpl<'a>;

pub type MemoryPool<'a> = TMemoryPoolInterface<'a, MemoryPoolImpl<'a>>;
// -------------------------------------------------------------------

//
pub use self::pipeline_api::ComputePipelineInfo;
pub use self::pipeline_api::GraphicsPipelineInfo;

#[cfg(feature = "backend_wgpu")]
type PipelineImpl<'a> = self::wgpu::pipeline_wgpu::Pipeline<'a>;

#[cfg(feature = "backend_ash")]
type PipelineImpl<'a> = self::ash::pipeline_ash::PipelineImpl<'a>;

pub type Pipeline<'a> = TPipelineInterface<'a, PipelineImpl<'a>>;
//-----------------------------------------------------------------------------

// Queue
pub use self::queue_api::QueueInfo;

#[cfg(feature = "backend_vulkano")]
type QueueImpl<'a> = self::vk::queue_vk::QueueImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type QueueImpl<'a> = self::wgpu::queue_wgpu::QueueImpl<'a>;

#[cfg(feature = "backend_ash")]
type QueueImpl<'a> = self::ash::queue_ash::QueueImpl<'a>;

pub type Queue<'a> = TQueueInterface<'a, QueueImpl<'a>>;
//--------------------------------------------------------------------

// RasterizerState
pub use self::rasterizer_state_api::RasterizerStateInfo;

#[cfg(feature = "backend_ash")]
type RasterizerStateImpl = self::ash::rasterizer_state_ash::RasterizerStateImpl;

pub type RasterizerState = TRasterizerStateInterface<RasterizerStateImpl>;
//

// Shader ------------------------------------------------------------
pub use self::shader_api::ShaderInfo;

#[cfg(feature = "backend_vulkano")]
type ShaderImpl<'a> = self::vk::shader_vk::ShaderImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type ShaderImpl<'a> = self::wgpu::shader_wgpu::ShaderImpl<'a>;

#[cfg(feature = "backend_ash")]
type ShaderImpl<'a> = self::ash::shader_ash::ShaderImpl<'a>;

pub type Shader<'a> = TShaderInterface<'a, ShaderImpl<'a>>;
//--------------------------------------------------------------------

// SwapChain
pub use self::swap_chain_api::SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain;

#[cfg(feature = "backend_ash")]
type SwapChainImpl<'a> = self::ash::swap_chain_ash::SwapChainImpl<'a>;

pub type SwapChain<'a> = TSwapChain<'a, SwapChainImpl<'a>>;
//

// Texture
pub use self::texture_api::BufferTextureCopyRegion;
pub use self::texture_api::TextureCopyRegion;
pub use self::texture_api::TextureInfo;
pub use self::texture_api::TextureSubResource;
pub use self::texture_api::TextureSubresourceRange;

#[cfg(feature = "backend_ash")]
type TextureImpl<'a> = self::ash::texture_ash::TextureImpl<'a>;

pub type Texture<'a> = TTexture<'a, TextureImpl<'a>>;
//

// VertexState
pub use self::vertex_state_api::VertexAttributeStateInfo;
pub use self::vertex_state_api::VertexBufferStateInfo;
pub use self::vertex_state_api::VertexStateInfo;

#[cfg(feature = "backend_ash")]
type VertexStateImpl = self::ash::vertex_state_ash::VertexStateImpl;

pub type VertexState = TVertexState<VertexStateImpl>;
//

//
pub use self::viewport_scissor_state_api::ScissorStateInfo;
pub use self::viewport_scissor_state_api::ViewportScissorStateInfo;
pub use self::viewport_scissor_state_api::ViewportStateInfo;

#[cfg(feature = "backend_ash")]
type ViewportScissorStateImpl = self::ash::viewport_scissor_state_ash::ViewportScissorStateImpl;

pub type ViewportScissorState<'a> = TViewportScissorState<'a, ViewportScissorStateImpl>;
//

bitflags! {
    pub struct MemoryPoolProperty: u32 {
        const CPU_CACHED = 0x01;
        const CPU_UNCACHED = 0x02;
        const GPU_CACHED = 0x04;
        const GPU_UNCACHED = 0x08;
        const CPU_INVISIBLE = 0x16;
    }
}

bitflags! {
    pub struct GpuAccess: u32 {
        const VERTEX_BUFFER = 0x01;
        const INDEX_BUFFER = 0x02;
        const CONSTANT_BUFFER = 0x04;
        const TEXTURE = 0x08;
        const UNORDERED_ACCESS_BUFFER = 0x16;
        const COLOR_BUFFER = 0x32;
        const DEPTH_STENCIL = 0x64;
        const READ = 0x128;
        const WRITE = 0x256;
        const INDIRECT_BUFFER = 0x128;
        const IMAGE = 0x4000;
    }
}

bitflags! {
    pub struct TextureState: u32 {
        const UNDEFINED = 0x01;
        const DATA_TRANSFER = 0x02;
        const COPY_SOURCE = 0x04;
        const COPY_DESTINATION = 0x08;
        const SHADER_READ = 0x16;
        const SHADER_WRITE = 0x32;
        const COLOR_TARGET = 0x64;
        const DEPTH_READ = 0x128;
        const DEPTH_WRITE = 0x256;
        const CLEAR = 0x512;
        const RESOLVE_SOURCE = 0x1024;
        const RESOLVE_DESTINATION = 0x2048;
        const PRESENT = 0x4096;
    }
}

pub enum ImageFormat {
    R8G8B8A8Unorm,
}

pub enum ShaderStage {
    Vertex,
    Pixel,
    Compute,
}

bitflags! {
    pub struct PipelineStageBit: u32 {
        const VERTEX_INPUT = 0x01;
        const VERTEX_SHADER = 0x02;
        const HULL_SHADER = 0x04;
        const DOMAIN_SHADER = 0x08;
        const GEOMETRY_SHADER = 0x16;
        const PIXEL_SHADER = 0x32;
        const RENDER_TARGET = 0x64;
        const COMPUTE_SHDER = 0x128;
    }
}

pub enum AttributeFormat {
    Float32_32,
    Float32_32_32,
}

pub enum PrimitiveTopology {
    PointList,
    TriangleList,
}

pub enum IndexFormat {
    Uint32,
}
