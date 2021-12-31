use super::super::device_api::DeviceInfo;
use futures::executor;
use wgpu::{Adapter, Surface};

pub struct DeviceImpl {
    device_impl: wgpu::Device,
    queue_impl: wgpu::Queue,
    _adapter: Adapter,
    _surface_opt: Option<Surface>,
}

impl super::super::device_api::TDeviceImpl for DeviceImpl {
    fn new(info: &DeviceInfo) -> DeviceImpl {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface_opt = match info.get_layer() {
            Some(layer) => unsafe { Some(instance.create_surface(layer.get_window())) },
            None => None,
        };
        let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: if surface_opt.is_some() {
                surface_opt.as_ref()
            } else {
                None
            },
        }))
        .unwrap();

        let (device, queue) = executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits()),
                features: wgpu::Features::empty(),
                label: None,
            },
            None,
        ))
        .unwrap();

        if surface_opt.is_some() {
            let surface = surface_opt.as_ref().unwrap();
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&adapter).unwrap(),
                width: 640,
                height: 480,
                present_mode: wgpu::PresentMode::Mailbox,
            };
            surface.configure(&device, &config);
        }

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

    pub fn get_adapter(&self) -> &Adapter {
        &self._adapter
    }

    pub fn try_get_surface(&self) -> Option<&Surface> {
        if self._surface_opt.is_some() {
            Some(self._surface_opt.as_ref().unwrap())
        } else {
            None
        }
    }
}
