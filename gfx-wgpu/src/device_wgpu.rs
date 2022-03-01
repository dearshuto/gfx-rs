use futures::executor;
use raw_window_handle::HasRawWindowHandle;
use sjgfx_interface::{DeviceInfo, IDevice};
use wgpu::{Adapter, Surface};

pub struct DeviceWgpu {
    device_impl: wgpu::Device,
    queue_impl: wgpu::Queue,

    #[allow(dead_code)]
    adapter: Adapter,

    #[allow(dead_code)]
    surface_opt: Option<Surface>,
}

impl DeviceWgpu {
    pub fn new_as_graphics<W>(_info: &DeviceInfo, window: &W) -> Self
    where
        W: HasRawWindowHandle,
    {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
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

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: 640,
            height: 480,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);

        DeviceWgpu {
            device_impl: device,
            queue_impl: queue,
            adapter: adapter,
            surface_opt: Some(surface),
        }
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device_impl
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue_impl
    }

    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        self.surface_opt.as_ref().unwrap()
    }
}

impl IDevice for DeviceWgpu {
    fn new(_: &DeviceInfo) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None,
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

        DeviceWgpu {
            device_impl: device,
            queue_impl: queue,
            adapter: adapter,
            surface_opt: None, //surface_opt,
        }
    }
}
