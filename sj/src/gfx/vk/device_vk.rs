use super::super::device_api::{DeviceInfo, TDeviceImpl};

pub struct DeviceImpl {
    device_impl: std::sync::Arc<vulkano::device::Device>,
    queue_impl: std::sync::Arc<vulkano::device::Queue>,
}

impl TDeviceImpl for DeviceImpl {
    fn new(info: &DeviceInfo) -> Self {
        let required_extensions = vulkano_win::required_extensions();
        let instance = vulkano::instance::Instance::new(None, &required_extensions, None).unwrap();
        let physical_device = vulkano::instance::PhysicalDevice::enumerate(&instance)
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
            physical_device.supported_features(),
            &device_ext,
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        DeviceImpl{
            device_impl: device.clone(),
            queue_impl: queues.next().unwrap(),
        }
    }
}

impl DeviceImpl {
    pub fn get_device_impl(&self) -> std::sync::Arc<vulkano::device::Device>
    {
    	self.device_impl.clone()
    }

    pub fn get_queue_impl(&self) -> std::sync::Arc<vulkano::device::Queue>
    {
    	self.queue_impl.clone()
    }
}

impl Drop for DeviceImpl {
    fn drop(&mut self) {}
}
