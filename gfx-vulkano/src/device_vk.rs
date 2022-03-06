use sjgfx_interface::{DeviceInfo, IDevice};
use std::sync::Arc;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, Features, Queue,
    },
    instance::Instance,
    swapchain::Surface,
};
use vulkano_win::VkSurfaceBuild;
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

pub struct DeviceVk {
    device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    surface: Option<Arc<Surface<Window>>>,
}

impl DeviceVk {
    pub fn new_as_graphics(_info: &DeviceInfo, event_loop: &EventLoop<()>) -> Self {
        let (instance, device, queue) = Self::create_device();

        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        Self {
            device,
            queue,
            surface: Some(surface),
        }
    }

    pub fn clone_device(&self) -> Arc<vulkano::device::Device> {
        self.device.clone()
    }

    pub fn get_queue(&self) -> &Queue {
        &self.queue
    }

    pub fn clone_queue(&self) -> Arc<vulkano::device::Queue> {
        self.queue.clone()
    }

    pub fn clone_surface(&self) -> Arc<Surface<Window>> {
        self.surface.as_ref().unwrap().clone()
    }

    pub fn get_physical_device(&self) -> PhysicalDevice {
        self.device.physical_device()
    }

    fn create_device() -> (Arc<Instance>, Arc<Device>, Arc<Queue>) {
        let required_extensions = vulkano_win::required_extensions();
        let instance = vulkano::instance::Instance::new(
            None,
            vulkano::Version::V1_0,
            &required_extensions,
            None,
        )
        .unwrap();

        // 物理デバイスの取得
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            ..vulkano::device::DeviceExtensions::none()
        };
        let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
            .filter(|&p| p.supported_extensions().is_superset_of(&device_ext))
            .filter_map(|p| {
                p.queue_families()
                    .find(|&q| q.supports_graphics())
                    .map(|q| (p, q))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
            })
            .unwrap();

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device,
            &Features::none(),
            &physical_device.required_extensions().union(&device_ext),
            [(queue_family, 0.5)].iter().cloned(),
        )
        .unwrap();

        (instance, device, queues.next().unwrap())
    }
}

impl IDevice for DeviceVk {
    fn new(_info: &DeviceInfo) -> Self {
        let (_instance, device, queue) = Self::create_device();
        Self {
            device,
            queue,
            surface: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{DeviceInfo, IDevice};

    use crate::DeviceVk;

    #[test]
    fn new() {
        let _ = DeviceVk::new(&DeviceInfo::new());
    }
}
