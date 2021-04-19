mod command_buffer_api;
mod device_api;
mod queue_api;
mod swap_chain;

use self::command_buffer_api::TCommandBufferInterface;
use self::device_api::TDeviceInterface;
use self::queue_api::TQueueInterface;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;


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


// Queue
pub use self::queue_api::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::queue_vk::Queue as Queue;
    
#[cfg(feature = "backend_wgpu")]
pub type Queue<'a> = TQueueInterface<'a, self::wgpu::queue_wgpu::QueueImpl<'a>>;
//--------------------------------------------------------------------

// SwapChain
pub use self::swap_chain::SwapChainInfo as SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain as SwapChain;
