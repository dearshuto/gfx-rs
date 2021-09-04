use super::super::device_api::{DeviceInfo, TDeviceImpl};

pub struct DeviceVk {
    device_impl: std::sync::Arc<vulkano::device::Device>,
    queue_impl: std::sync::Arc<vulkano::device::Queue>,
}

impl TDeviceImpl for DeviceVk {
    fn new(_info: &DeviceInfo) -> Self {
        let required_extensions = vulkano_win::required_extensions();
        let instance = vulkano::instance::Instance::new(
            None,
            vulkano::Version::V1_1,
            &required_extensions,
            None,
        )
        .unwrap();
        let physical_device = vulkano::device::physical::PhysicalDevice::enumerate(&instance)
            .next()
            .unwrap();

        let queue_family = physical_device
            .queue_families()
            .find(|&q| {
                // We take the first queue that supports drawing to our window.
                q.supports_graphics()
            })
            .unwrap();

        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };
        let (device, mut queues) = vulkano::device::Device::new(
            physical_device,
            &vulkano::device::Features::none(),
            &physical_device.required_extensions().union(&device_ext),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        Self {
            device_impl: device,
            queue_impl: queues.next().unwrap(),
        }
    }
}

impl DeviceVk {
    pub fn get_device_impl(&self) -> std::sync::Arc<vulkano::device::Device> {
        self.device_impl.clone()
    }

    pub fn get_queue(&self) -> std::sync::Arc<vulkano::device::Queue> {
        self.queue_impl.clone()
    }

    pub fn clone_queue(&self) -> std::sync::Arc<vulkano::device::Queue> {
        self.queue_impl.clone()
    }
}
