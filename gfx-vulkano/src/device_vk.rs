use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
};
use sjgfx_interface::{DeviceInfo, IDevice};
use std::sync::Arc;
use vulkano::{
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo,
    },
    instance::{Instance, InstanceCreateInfo},
    swapchain::Surface,
    Version, VulkanLibrary,
};

#[derive(Debug)]
struct Handler {
    raw_window_handle: RawWindowHandle,
    raw_display_handle: RawDisplayHandle,
}

unsafe impl Send for Handler {}
unsafe impl Sync for Handler {}

unsafe impl HasRawWindowHandle for Handler {
    fn raw_window_handle(&self) -> raw_window_handle::RawWindowHandle {
        self.raw_window_handle
    }
}

unsafe impl HasRawDisplayHandle for Handler {
    fn raw_display_handle(&self) -> raw_window_handle::RawDisplayHandle {
        self.raw_display_handle
    }
}

pub struct DeviceVk {
    device: Arc<vulkano::device::Device>,
    queue: Arc<vulkano::device::Queue>,
    surface: Option<Arc<Surface>>,
}

impl DeviceVk {
    pub fn new(_info: &DeviceInfo) -> Self {
        let (_instance, device, queue) = Self::create_device();
        Self {
            device,
            queue,
            surface: None,
        }
    }

    pub fn new_from_handle<T>(_info: &DeviceInfo, handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        let handler = Arc::new(Handler {
            raw_window_handle: handle.raw_window_handle(),
            raw_display_handle: handle.raw_display_handle(),
        });

        let (instance, device, queue) = Self::create_device();
        let surface = vulkano_win::create_surface_from_handle(handler, instance).unwrap();
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

    pub fn clone_surface(&self) -> Arc<Surface> {
        self.surface.as_ref().unwrap().clone()
    }

    pub fn get_physical_device(&self) -> &PhysicalDevice {
        self.device.physical_device()
    }

    fn create_device() -> (Arc<Instance>, Arc<Device>, Arc<Queue>) {
        let vulkan_library = VulkanLibrary::new().unwrap();

        let required_extensions = vulkano_win::required_extensions(&vulkan_library);
        let instance = Instance::new(
            vulkan_library,
            InstanceCreateInfo {
                enabled_extensions: required_extensions,
                engine_version: Version::V1_2,
                ..Default::default()
            },
        )
        .unwrap();

        // 物理デバイスの取得
        let device_ext = vulkano::device::DeviceExtensions {
            khr_swapchain: true,
            khr_maintenance1: true,
            ..vulkano::device::DeviceExtensions::empty()
        };
        let (physical_device, queue_family_index) = instance
            .enumerate_physical_devices()
            .unwrap()
            .filter(|p| p.supported_extensions().contains(&device_ext))
            .filter_map(|p| {
                p.queue_family_properties()
                    .iter()
                    .enumerate()
                    .position(|(_i, q)| {
                        q.queue_flags.graphics
                        //&& p.surface_support(i as u32, &surface).unwrap_or(false)
                    })
                    .map(|q| (p, q as u32))
            })
            .min_by_key(|(p, _)| match p.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                PhysicalDeviceType::Other => 4,
                _ => 5,
            })
            .unwrap();

        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..DeviceExtensions::empty()
        };
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                enabled_extensions: device_extensions,
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                ..Default::default()
            },
        )
        .unwrap();

        (instance, device, queues.next().unwrap())
    }
}

impl IDevice for DeviceVk {
    type Display = sjvi::winit::Display<()>;

    fn new(_info: &DeviceInfo) -> Self {
        let (_instance, device, queue) = Self::create_device();
        Self {
            device,
            queue,
            surface: None,
        }
    }

    fn new_with_surface(info: &DeviceInfo, display: &Self::Display) -> Self {
        Self::new_from_handle(info, &display.window)
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::DeviceInfo;

    use crate::DeviceVk;

    #[test]
    fn new() {
        let _ = DeviceVk::new(&DeviceInfo::new());
    }
}
