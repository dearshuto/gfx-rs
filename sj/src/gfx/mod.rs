mod device_info;
mod queue_info;
mod vk;

// Device
pub use self::device_info::DeviceInfo as DeviceInfo;
pub use self::vk::device_vk::Device as Device;

// Queue
pub use self::queue_info::QueueInfo as QueueInfo;
pub use self::vk::queue_vk::Queue as Queue;

    
