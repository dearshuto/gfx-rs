use super::super::device_info::DeviceInfo;
use futures::executor;


pub struct Device
{
	device_impl: wgpu::Device,
	queue_impl: wgpu::Queue,
}


impl Device
{
	pub fn new(_info: &DeviceInfo) -> Device
	{
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
		let adapter = executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default())).unwrap();

		let (device, queue) =  executor::block_on(
			adapter
				.request_device(
					&wgpu::DeviceDescriptor {
						label: None,
						features: wgpu::Features::empty(),
						limits: wgpu::Limits::default(),
					},
					None,
				)).unwrap();
			
		Device{
			device_impl: device,
			queue_impl: queue
		}
	}
	
	pub fn get_queue(&mut self) -> &mut wgpu::Queue
	{
		&mut self.queue_impl
	}
}
