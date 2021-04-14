mod device_info;
mod queue_info;
mod swap_chain;

#[cfg(feature = "backend_vulkano")]
mod vk;

#[cfg(feature = "backend_wgpu")]
mod wgpu;


// Device ---------------------------------------------------------------------
pub use self::device_info::DeviceInfo as DeviceInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::device_vk::Device as Device;

#[cfg(feature = "backend_wgpu")]
pub use self::wgpu::device_wgpu::Device as Device;
//-----------------------------------------------------------------------------


// Queue
pub use self::queue_info::QueueInfo as QueueInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::queue_vk::Queue as Queue;
    
#[cfg(feature = "backend_wgpu")]
pub use self::wgpu::queue_wgpu::Queue as Queue;
//--------------------------------------------------------------------

// SwapChain
pub use self::swap_chain::SwapChainInfo as SwapChainInfo;

#[cfg(feature = "backend_vulkano")]
pub use self::vk::swap_chain_vk::SwapChain as SwapChain;
