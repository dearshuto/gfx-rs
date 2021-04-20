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
pub type Buffer<'a> = TBufferInterface<'a, self::wgpu::buffer_wgpu::BufferImpl<'a>>;
// -------------------------------------------------------------------



// CommandBuffer -----------------------------------------------------
pub use self::command_buffer_api::CommandBufferInfo as CommandBufferInfo;

#[cfg(feature = "backend_wgpu")]
pub type CommandBuffer<'a> = TCommandBufferInterface<'a, self::wgpu::command_buffer_wgpu::CommandBuffer<'a>>;
// -------------------------------------------------------------------

// Device --------------------------------------------------------------------
pub use self::device_api::DeviceInfo as DeviceInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::device_vk::Device as Device;

#[cfg(feature = "backend_wgpu")]
pub type Device = TDeviceInterface<self::wgpu::device_wgpu::DeviceImpl>;
//-----------------------------------------------------------------------------



//
pub use self::pipeline_api::PipelineInfo;

#[cfg(feature = "backend_wgpu")]
pub type Pipeline<'a> = TPipelineInterface<'a, self::wgpu::pipeline_wgpu::Pipeline<'a>>;
//-----------------------------------------------------------------------------



// Queue
pub use self::queue_api::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::queue_vk::Queue as Queue;
    
#[cfg(feature = "backend_wgpu")]
pub type Queue<'a> = TQueueInterface<'a, self::wgpu::queue_wgpu::QueueImpl<'a>>;
//--------------------------------------------------------------------



// Shader ------------------------------------------------------------
pub use self::shader_api::ShaderInfo as ShaderInfo;

#[cfg(feature = "backend_wgpu")]
pub type Shader<'a> = TShaderInterface<'a, self::wgpu::shader_wgpu::ShaderImpl<'a>>;
//--------------------------------------------------------------------



// SwapChain
pub use self::swap_chain::SwapChainInfo as SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain as SwapChain;
