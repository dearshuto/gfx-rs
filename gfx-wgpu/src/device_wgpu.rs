use std::sync::Arc;

use futures::executor;
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};
use sjgfx_interface::{DeviceInfo, IDevice};
use wgpu::{Adapter, Surface};

pub struct DeviceWgpu {
    device: Arc<wgpu::Device>,
    queue_impl: Arc<wgpu::Queue>,

    #[allow(dead_code)]
    adapter: Adapter,

    #[allow(dead_code)]
    surface_opt: Option<Arc<Surface>>,
}

impl DeviceWgpu {
    pub fn new_as_graphics<W>(_info: &DeviceInfo, window: &W) -> Self
    where
        W: HasRawWindowHandle + HasRawDisplayHandle,
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

        // Device の limits はウェブ版で分岐が必要
        let optional_features = wgpu::Features::empty();
        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        let (device, queue) = executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits())
                } else {
                    wgpu::Limits::downlevel_defaults().using_resolution(adapter.limits())
                },
                features: (optional_features & adapter_features) | required_features,
                label: None,
            },
            None,
        ))
        .unwrap();

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_supported_formats(&adapter)[0],
            width: 1600,
            height: 1200,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: surface.get_supported_alpha_modes(&adapter)[0],
        };
        surface.configure(&device, &config);

        DeviceWgpu {
            device: Arc::new(device),
            queue_impl: Arc::new(queue),
            adapter,
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
                format: surface.get_supported_formats(&self.adapter)[0],
                width,
                height,
                present_mode: wgpu::PresentMode::Fifo,
                alpha_mode: surface.get_supported_alpha_modes(&self.adapter)[0],
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
            adapter,
            surface_opt: None, //surface_opt,
        }
    }

    fn new_with_handle<T>(info: &DeviceInfo, raw_handle: &T) -> Self
    where
        T: HasRawWindowHandle + HasRawDisplayHandle,
    {
        DeviceWgpu::new_as_graphics(info, raw_handle)
    }
}
