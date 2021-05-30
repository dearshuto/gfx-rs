fn main() {
	let mut display = sj::vi::create_display();
	let mut layer = sj::vi::create_layer(&mut display);
	
	let device = sj::gfx::Device::new(&sj::gfx::DeviceInfo::new());
	let mut queue = sj::gfx::Queue::new(&device, &sj::gfx::QueueInfo::new());
	let mut swap_chain_info = sj::gfx::SwapChainInfo::new(&mut layer);
	let mut swap_chain = sj::gfx::SwapChain::new(&device, &mut swap_chain_info);

	let _scan_buffer_views = swap_chain.get_scan_buffer_views_mut();
	
	let command_buffer_info = sj::gfx::CommandBufferInfo::new();
	let mut _command_buffer = sj::gfx::CommandBuffer::new(&device, &command_buffer_info);
	
	loop {
		{
			//			let next_scan_buffer = &mut scan_buffer_views[0];
			//command_buffer.clear_color(&mut scan_buffer_views[0], 0.0, 0.0, 0.0, 0.0);			
		}
		
		queue.present(&mut swap_chain, 1);
		std::thread::sleep(std::time::Duration::from_millis(16));
	}
}
