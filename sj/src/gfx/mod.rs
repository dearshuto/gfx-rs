mod buffer_api;
mod command_buffer_api;
mod device_api;
mod memory_pool_api;
mod pipeline_api;
mod queue_api;
mod shader_api;
mod swap_chain;

use self::buffer_api::TBufferInterface;
use self::command_buffer_api::TCommandBufferInterface;
use self::device_api::TDeviceInterface;
use self::memory_pool_api::TMemoryPoolInterface;
use self::pipeline_api::TPipelineInterface;
use self::queue_api::TQueueInterface;
use self::shader_api::TShaderInterface;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;

#[cfg(feature = "backend_ash")]
mod ash;

// Buffer  -----------------------------------------------------------
pub use self::buffer_api::BufferInfo;

#[cfg(feature = "backend_ash")]
type BufferImpl<'a> = self::ash::buffer_ash::BufferImpl<'a>;

pub type Buffer<'a> = TBufferInterface<'a, BufferImpl<'a>>;
// -------------------------------------------------------------------



// CommandBuffer -----------------------------------------------------
pub use self::command_buffer_api::CommandBufferInfo as CommandBufferInfo;

#[cfg(feature = "backend_wgpu")]
pub type CommandBuffer<'a> = TCommandBufferInterface<'a, self::wgpu::command_buffer_wgpu::CommandBuffer<'a>>;

#[cfg(feature = "backend_ash")]
type CommandBufferImpl<'a> = self::ash::command_buffer_ash::CommandBufferImpl<'a>;

pub type CommandBuffer<'a> = TCommandBufferInterface<'a, CommandBufferImpl<'a>>;
// -------------------------------------------------------------------

// Device --------------------------------------------------------------------
pub use self::device_api::DeviceInfo as DeviceInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::device_vk::Device as Device;

#[cfg(feature = "backend_wgpu")]
pub type Device = TDeviceInterface<self::wgpu::device_wgpu::DeviceImpl>;

#[cfg(feature = "backend_ash")]
type DeviceImpl = self::ash::device_ash::DeviceImpl;

pub type Device = TDeviceInterface<DeviceImpl>;
//-----------------------------------------------------------------------------



// MemoryPool
pub use self::memory_pool_api::MemoryPoolInfo as MemoryPoolInfo;

#[cfg(feature = "backend_ash")]
type MemoryPoolImpl<'a> = self::ash::memory_pool_ash::MemoryPoolImpl<'a>;

pub type MemoryPool<'a> = TMemoryPoolInterface<'a, MemoryPoolImpl<'a>>;
// -------------------------------------------------------------------



//
pub use self::pipeline_api::ComputePipelineInfo as ComputePipelineInfo;

#[cfg(feature = "backend_wgpu")]
pub type Pipeline<'a> = TPipelineInterface<'a, self::wgpu::pipeline_wgpu::Pipeline<'a>>;

#[cfg(feature = "backend_ash")]
type PipelineImpl<'a> = self::ash::pipeline_ash::PipelineImpl<'a>;

pub type Pipeline<'a> = TPipelineInterface<'a, PipelineImpl<'a>>;
//-----------------------------------------------------------------------------



// Queue
pub use self::queue_api::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::queue_vk::Queue as Queue;
    
#[cfg(feature = "backend_wgpu")]
pub type Queue<'a> = TQueueInterface<'a, self::wgpu::queue_wgpu::QueueImpl<'a>>;

#[cfg(feature = "backend_ash")]
type QueueImpl<'a> = self::ash::queue_ash::QueueImpl<'a>;

pub type Queue<'a> = TQueueInterface<'a, QueueImpl<'a>>;
//--------------------------------------------------------------------



// Shader ------------------------------------------------------------
pub use self::shader_api::ShaderInfo as ShaderInfo;

#[cfg(feature = "backend_wgpu")]
pub type Shader<'a> = TShaderInterface<'a, self::wgpu::shader_wgpu::ShaderImpl<'a>>;

#[cfg(feature = "backend_ash")]
type ShaderImpl<'a> = self::ash::shader_ash::ShaderImpl<'a>;

pub type Shader<'a> = TShaderInterface<'a, ShaderImpl<'a>>;
//--------------------------------------------------------------------



// SwapChain
pub use self::swap_chain::SwapChainInfo as SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain as SwapChain;
