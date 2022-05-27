use std::sync::Arc;

use futures::executor;
use raw_window_handle::HasRawWindowHandle;
use sjgfx_interface::{DeviceInfo, IDevice};
use wgpu::{Adapter, Surface};
use winit::event_loop::EventLoop;

pub struct DeviceWgpu {
    device: Arc<wgpu::Device>,
    queue_impl: Arc<wgpu::Queue>,

    adapter: Arc<Adapter>,

    #[allow(dead_code)]
    surface_opt: Option<Arc<Surface>>,
}

impl DeviceWgpu {
    pub fn new_as_graphics<W>(_info: &DeviceInfo, window: &W) -> Self
    where
        W: HasRawWindowHandle,
    {
        let backend = Self::get_primary_backend_type();
        let instance = wgpu::Instance::new(backend);
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
                features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                label: None,
            },
            None,
        ))
        .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: 1280,
            height: 960,
            present_mode: wgpu::PresentMode::Mailbox,
        };
        surface.configure(&device, &config);

        DeviceWgpu {
            device: Arc::new(device),
            queue_impl: Arc::new(queue),
            adapter: Arc::new(adapter),
            surface_opt: Some(Arc::new(surface)),
        }
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.device
    }

    pub fn close_device(&self) -> Arc<wgpu::Device> {
        self.device.clone()
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.queue_impl
    }

    pub fn clone_queue(&self) -> Arc<wgpu::Queue> {
        self.queue_impl.clone()
    }

    pub fn get_adapter(&self) -> &wgpu::Adapter {
        &self.adapter
    }

    pub fn clone_adapter(&self) -> Arc<Adapter> {
        self.adapter.clone()
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        self.surface_opt.as_ref().unwrap()
    }

    pub fn clone_surface(&self) -> Arc<wgpu::Surface> {
        self.surface_opt.as_ref().unwrap().clone()
    }

    pub fn update_surface_size(&mut self, width: u32, height: u32) {
        if let Some(surface) = &self.surface_opt {
            let config = wgpu::SurfaceConfiguration {
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                format: surface.get_preferred_format(&self.adapter).unwrap(),
                width: width,
                height: height,
                present_mode: wgpu::PresentMode::Mailbox,
            };
            surface.configure(&self.device, &config);
        }
    }

    fn get_primary_backend_type() -> wgpu::Backends {
        if cfg!(target_os = "windows") {
            wgpu::Backends::DX12
        } else {
            wgpu::Backends::all()
        }
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
                features: wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES,
                label: None,
            },
            None,
        ))
        .unwrap();

        DeviceWgpu {
            device: Arc::new(device),
            queue_impl: Arc::new(queue),
            adapter: Arc::new(adapter),
            surface_opt: None, //surface_opt,
        }
    }

    fn new_with_surface<TWindow>(
        info: &DeviceInfo,
        window: &TWindow,
        _event_loop: &EventLoop<()>,
    ) -> Self
    where
        TWindow: raw_window_handle::HasRawWindowHandle,
    {
        Self::new_as_graphics(info, window)
    }
}
