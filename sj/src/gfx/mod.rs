mod buffer_api;
mod command_buffer_api;
mod device_api;
mod pipeline_api;
mod queue_api;
mod shader_api;
mod swap_chain;

use self::buffer_api::TBufferInterface;
use self::command_buffer_api::TCommandBufferInterface;
use self::device_api::TDeviceInterface;
use self::pipeline_api::TPipelineInterface;
use self::queue_api::TQueueInterface;
use self::shader_api::TShaderInterface;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;


// Buffer  -----------------------------------------------------------
pub use self::buffer_api::BufferInfo;

#[cfg(feature = "backend_vulkano")]
type BufferImpl<'a> = self::vk::buffer_vk::BufferImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type BufferImpl = self::wgpu::buffer_wgpu::BufferImpl<'a>;

pub type Buffer<'a> = TBufferInterface<'a, BufferImpl<'a>>;
// -------------------------------------------------------------------



// CommandBuffer -----------------------------------------------------
pub use self::command_buffer_api::CommandBufferInfo as CommandBufferInfo;

#[cfg(feature = "backend_vulkano")]
type CommandBufferImpl<'a> = self::vk::command_buffer_vk::CommandBufferImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type CommandBufferImpl = self::wgpu::command_buffer_wgpu::CommandBuffer<'a>;

pub type CommandBuffer<'a> = TCommandBufferInterface<'a, CommandBufferImpl<'a>>;
// -------------------------------------------------------------------



// Device --------------------------------------------------------------------
pub use self::device_api::DeviceInfo as DeviceInfo;

#[cfg(feature = "backend_vulkano")]
type DeviceImpl = self::vk::device_vk::DeviceImpl;

#[cfg(feature = "backend_wgpu")]
type DeviceImpl = self::wgpu::device_wgpu::DeviceImpl;

pub type Device = TDeviceInterface<DeviceImpl>;
//-----------------------------------------------------------------------------



//
//pub use self::pipeline_api::PipelineInfo;

//#[cfg(feature = "backend_wgpu")]
//pub type Pipeline<'a> = TPipelineInterface<'a, self::wgpu::pipeline_wgpu::Pipeline<'a>>;
//-----------------------------------------------------------------------------



// Queue
pub use self::queue_api::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
type QueueImpl<'a> =  self::vk::queue_vk::QueueImpl<'a>;
    
#[cfg(feature = "backend_wgpu")]
type QueueImpl<'a> = self::wgpu::queue_wgpu::QueueImpl<'a>;

pub type Queue<'a> = TQueueInterface<'a, QueueImpl<'a>>;
//--------------------------------------------------------------------



// Shader ------------------------------------------------------------
pub use self::shader_api::ShaderInfo as ShaderInfo;

#[cfg(feature = "backend_vulkano")]
type ShaderImpl<'a> = self::vk::shader_vk::ShaderImpl<'a>;

#[cfg(feature = "backend_wgpu")]
type ShaderImpl = self::wgpu::shader_wgpu::ShaderImpl<'a>;

pub type Shader<'a> = TShaderInterface<'a, ShaderImpl<'a>>;
//--------------------------------------------------------------------



// SwapChain
//pub use self::swap_chain::SwapChainInfo as SwapChainInfo;

//#[cfg(feature = "backend_vulkano")]
//pub use self::vk::swap_chain_vk::SwapChain as SwapChain;
