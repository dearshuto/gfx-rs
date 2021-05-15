mod blend_state_api;
mod buffer_api;
mod command_buffer_api;
mod depth_stencil_state_api;
//mod descriptor_pool_api;
mod device_api;
mod memory_pool_api;
mod pipeline_api;
mod queue_api;
mod rasterizer_state_api;
mod shader_api;
mod swap_chain_api;

use self::blend_state_api::TBlendState;
use self::buffer_api::TBufferInterface;
use self::command_buffer_api::TCommandBufferInterface;
use self::depth_stencil_state_api::TDepthStencilState;
//use self::descriptor_pool_api::TDescriptorInterface;
use self::device_api::TDeviceInterface;
use self::memory_pool_api::TMemoryPoolInterface;
use self::pipeline_api::TPipelineInterface;
use self::rasterizer_state_api::TRasterizerStateInterface;
use self::queue_api::TQueueInterface;
use self::shader_api::TShaderInterface;
use self::swap_chain_api::TSwapChain;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;

#[cfg(feature = "backend_ash")]
mod ash;



// BlendState
pub use self::blend_state_api::BlendStateInfo as BlendStateInfo;

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



// CommandBuffer -----------------------------------------------------
pub use self::command_buffer_api::CommandBufferInfo as CommandBufferInfo;

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
pub use self::device_api::DeviceInfo as DeviceInfo;

#[cfg(feature = "backend_vulkano")]
type DeviceImpl = self::vk::device_vk::DeviceImpl;

#[cfg(feature = "backend_wgpu")]
type DeviceImpl = self::wgpu::device_wgpu::DeviceImpl;

#[cfg(feature = "backend_ash")]
type DeviceImpl = self::ash::device_ash::DeviceImpl;

pub type Device = TDeviceInterface<DeviceImpl>;
//-----------------------------------------------------------------------------



// DepthStencilState
pub use self::depth_stencil_state_api::DepthStencilStateInfo as DepthStencilStateInfo;

#[cfg(feature = "backend_ash")]
type DepthStencilStateImpl = self::ash::depth_stencil_state_ash::DepthStencilStateImpl;

pub type DepthStencilState = TDepthStencilState<DepthStencilStateImpl>;
//



// MemoryPool
pub use self::memory_pool_api::MemoryPoolInfo as MemoryPoolInfo;

#[cfg(feature = "backend_ash")]
type MemoryPoolImpl<'a> = self::ash::memory_pool_ash::MemoryPoolImpl<'a>;

pub type MemoryPool<'a> = TMemoryPoolInterface<'a, MemoryPoolImpl<'a>>;
// -------------------------------------------------------------------



//
pub use self::pipeline_api::ComputePipelineInfo as ComputePipelineInfo;

#[cfg(feature = "backend_wgpu")]
type PipelineImpl<'a> = self::wgpu::pipeline_wgpu::Pipeline<'a>;

#[cfg(feature = "backend_ash")]
type PipelineImpl<'a> = self::ash::pipeline_ash::PipelineImpl<'a>;

pub type Pipeline<'a> = TPipelineInterface<'a, PipelineImpl<'a>>;
//-----------------------------------------------------------------------------



// Queue
pub use self::queue_api::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
type QueueImpl<'a> =  self::vk::queue_vk::QueueImpl<'a>;
    
#[cfg(feature = "backend_wgpu")]
type QueueImpl<'a> = self::wgpu::queue_wgpu::QueueImpl<'a>;

#[cfg(feature = "backend_ash")]
type QueueImpl<'a> = self::ash::queue_ash::QueueImpl<'a>;

pub type Queue<'a> = TQueueInterface<'a, QueueImpl<'a>>;
//--------------------------------------------------------------------



// RasterizerState
pub use self::rasterizer_state_api::RasterizerStateInfo as RasterizerStateInfo;

#[cfg(feature = "backend_ash")]
type RasterizerStateImpl = self::ash::rasterizer_state_ash::RasterizerStateImpl;

pub type RasterizerState = TRasterizerStateInterface<RasterizerStateImpl>;
//



// Shader ------------------------------------------------------------
pub use self::shader_api::ShaderInfo as ShaderInfo;

#[cfg(feature = "backend_vulkano")]
type ShaderImpl<'a> = self::vk::shader_vk::ShaderImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type ShaderImpl<'a> = self::wgpu::shader_wgpu::ShaderImpl<'a>;

#[cfg(feature = "backend_ash")]
type ShaderImpl<'a> = self::ash::shader_ash::ShaderImpl<'a>;

pub type Shader<'a> = TShaderInterface<'a, ShaderImpl<'a>>;
//--------------------------------------------------------------------



// SwapChain
pub use self::swap_chain_api::SwapChainInfo as SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain as SwapChain;

#[cfg(feature = "backend_ash")]
type SwapChainImpl<'a> = self::ash::swap_chain_ash::SwapChainImpl<'a>;

pub type SwapChain<'a, 'b, 'c> = TSwapChain<'a, 'b, 'c, SwapChainImpl<'a>>;
//
