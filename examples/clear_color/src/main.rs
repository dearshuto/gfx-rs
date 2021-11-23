fn main()
{
    let mut display = sj::vi::create_display();
    let mut layer = sj::vi::create_layer(&mut display);

    let device = sj::gfx::Device::new(&sj::gfx::DeviceInfo::new().set_layer(Some(&layer)));
    let mut queue = sj::gfx::Queue::new(&device, &sj::gfx::QueueInfo::new());
    let mut swap_chain_info = sj::gfx::SwapChainInfo::new(&mut layer);
    let mut swap_chain = sj::gfx::SwapChain::new(&device, &mut swap_chain_info);

	let device_wgpu = device.to_data().get_device();
	let surface = device.to_data().try_get_surface().unwrap();
	let adapter = device.to_data().get_adapter();
	let swapchain_format = surface.get_preferred_format(&adapter).unwrap();
	
	let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: swapchain_format,
        width: 640,
        height: 480,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    surface.configure(&device_wgpu, &config);
	
    loop {
		let frame = surface
            .get_current_frame()
            .expect("Failed to acquire next swap chain texture")
            .output;
		let mut encoder =
            device.to_data().get_device().create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
		encoder.clear_texture(&frame.texture, &wgpu::ImageSubresourceRange{
			aspect: wgpu::TextureAspect::All,
			base_mip_level: 0,
			mip_level_count: None,
			base_array_layer: 0,
			array_layer_count: None });
		queue.to_data().get_queue().submit(Some(encoder.finish()));
		
        queue.present(&mut swap_chain, 1);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
