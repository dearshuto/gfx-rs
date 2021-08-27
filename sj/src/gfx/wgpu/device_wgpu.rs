use super::super::device_api::DeviceInfo;
use futures::executor;

pub struct DeviceImpl {
    device_impl: wgpu::Device,
    queue_impl: wgpu::Queue,
}

impl super::super::device_api::TDeviceImpl for DeviceImpl {
    fn new(_info: &DeviceInfo) -> DeviceImpl {
        let instance = wgpu::Instance::new(wgpu::Backends::PRIMARY);
        let adapter =
            executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .unwrap();

        let (device, queue) = executor::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                limits: wgpu::Limits::default(),
                features: wgpu::Features::default(),
                label: None,
            },
            None,
        ))
        .unwrap();

        DeviceImpl {
            device_impl: device,
            queue_impl: queue,
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
}
