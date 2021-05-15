fn main() {
	let mut display = sj::vi::create_display();
	let layer = sj::vi::create_layer(&mut display);
	
	let device = sj::gfx::Device::new(&sj::gfx::DeviceInfo::new());
	let mut queue = sj::gfx::Queue::new(&device, &sj::gfx::QueueInfo::new());
	let swap_chain_info = sj::gfx::SwapChainInfo::new(layer);
	let mut swap_chain = sj::gfx::SwapChain::new(&device, swap_chain_info);

	loop {
		queue.present(&mut swap_chain, 1);
		std::thread::sleep(std::time::Duration::from_millis(16));
	}
}
