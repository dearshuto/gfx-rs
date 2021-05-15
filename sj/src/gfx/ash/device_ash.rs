pub use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::vk;
use super::super::device_api::{DeviceInfo, TDeviceImpl};

pub struct DeviceImpl
{
	_instance: ash::Instance,
	pub _device: ash::Device,
	_queue: ash::vk::Queue,
	_queue_family_properties: Vec<ash::vk::QueueFamilyProperties>,
}

impl DeviceImpl
{
	pub fn get_device(&self) -> &ash::Device
	{
		&self._device
	}
}

impl TDeviceImpl for DeviceImpl
{
	fn new(_info: &DeviceInfo) -> Self
	{
		unsafe {
			let entry = ash::Entry::new().unwrap();
			let appinfo = vk::ApplicationInfo::builder()
//                .application_name(&app_name)
                .application_version(0)
//                .engine_name(&app_name)
                .engine_version(0)
                .api_version(vk::make_version(1, 0, 0));

			let layer_names = [std::ffi::CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
            let _layers_names_raw: Vec<*const i8> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();
			
			let surface_extensions = vec![
				ash::extensions::khr::Surface::name(),
//				ash::extensions::khr::XlibSurface::name(),
//				ash::extensions::khr::WaylandSurface::name(),
//				ash::extensions::khr::XcbSurface::name(),
				ash::extensions::ext::MetalSurface::name()];
			let mut extension_names_raw = surface_extensions
                .iter()
                .map(|ext| ext.as_ptr())
                .collect::<Vec<_>>();
            extension_names_raw.push(ash::extensions::ext::DebugUtils::name().as_ptr());
			
            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&appinfo)
//                .enabled_layer_names(&layers_names_raw)
				.enabled_extension_names(&extension_names_raw);

            let instance = entry
                .create_instance(&create_info, None)
                .expect("Instance creation error");

			let physical_devices = instance
                .enumerate_physical_devices()
                .expect("Physical device error");
			let (physical_device, queue_family_index) = physical_devices
				.iter()
				.map(|physical_device|
					 {
						 instance.get_physical_device_queue_family_properties(*physical_device)
							 .iter()
							 .enumerate()
							 .filter_map(|(index, ref info)|{
								 let supports_graphic_and_surface =
									 info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
								 if supports_graphic_and_surface {
									 Some((*physical_device, index))	 
								 }
								 else{
									 None
								 }
							 }).next()
					 })
				.filter_map(|x| x)
				.next()
				.unwrap();

			let queue_family_properties = instance.get_physical_device_queue_family_properties(physical_device);

			let queue_family_index = queue_family_index as u32;

			let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };
			let priorities = [1.0];
			let queue_info = [vk::DeviceQueueCreateInfo::builder()
							  .queue_family_index(queue_family_index)
							  .queue_priorities(&priorities)
							  .build()];
			let device_create_info = vk::DeviceCreateInfo::builder()
				.queue_create_infos(&queue_info)
				.enabled_features(&features);
			let device = instance.create_device(physical_device, &device_create_info, None).unwrap();
			let queue = device.get_device_queue(0, 0);
			
			Self{
				_instance: instance,
				_device: device,
				_queue: queue,
				_queue_family_properties: queue_family_properties,
			}
		}				
	}
}

impl Drop for DeviceImpl
{
	fn drop(&mut self)
	{
		unsafe{
			self._instance.destroy_instance(None);
		}
	}
}
