use super::super::device_api::DeviceInfo;
use futures::executor;

pub struct DeviceImpl {
    device_impl: wgpu::Device,
    queue_impl: wgpu::Queue,
	_adapter: wgpu::Adapter,
	_surface_opt: Option<wgpu::Surface>,
}

impl super::super::device_api::TDeviceImpl for DeviceImpl {
    fn new(info: &DeviceInfo) -> DeviceImpl {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);		
		let surface_opt =  if let Some(layer) = info.get_layer()
		{
			unsafe{ Some(instance.create_surface(layer.get_window())) }
		}
		else
		{
			None
		};
		let compatible_surface = if let Some(surface) = surface_opt.as_ref() {
			Some(surface)
		}
		else{ None };
		
        let adapter =
            executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface
            })).unwrap();

		println!("Name   : {}", adapter.get_info().name);
		println!("Backend: {:?}", adapter.get_info().backend);
		println!("Vendor : {}", adapter.get_info().vendor);
        let (device, queue) = executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                features: wgpu::Features::empty(),
                label: None,
            },
            None,
        ))
        .unwrap();

        DeviceImpl {
            device_impl: device,
            queue_impl: queue,
			_adapter: adapter,
			_surface_opt: surface_opt,
        }
    }
}

impl DeviceImpl {
    pub fn get_device(&self) -> &wgpu::Device {
        &self.device_impl
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue_impl
    }

	pub fn get_adapter(&self) -> &wgpu::Adapter {
		&self._adapter
	}

	pub fn get_surface(&self) -> &Option<wgpu::Surface> {
		&self._surface_opt
	}

	pub fn get_surface_mut(&mut self) -> &mut Option<wgpu::Surface> {
		&mut self._surface_opt
	}
}
