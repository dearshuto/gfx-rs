fn main()
{    
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);
    
    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    let display = sj::vi::create_display();
    let mut layer = sj::vi::create_layer(&display);
    
    let mut swap_chain_info = sj::gfx::SwapChainInfo::new().set_layer(&mut layer);
    let mut swap_chain = sj::gfx::SwapChain::new(&device, &mut swap_chain_info);

    println!("begin");
    for _ in 0..100 {
	let tt = &mut swap_chain;
	queue.present(tt);
	queue.flush();
	queue.sync();
    }
    println!("end");
}
