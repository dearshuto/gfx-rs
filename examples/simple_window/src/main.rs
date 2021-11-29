fn main() {
    let mut display = sj::vi::create_display();
    let mut layer = sj::vi::create_layer(&mut display);

    let device = sj::gfx::Device::new(&sj::gfx::DeviceInfo::new().set_layer(Some(&mut layer)));
    let mut queue = sj::gfx::Queue::new(&device, &sj::gfx::QueueInfo::new());
    let mut swap_chain = sj::gfx::SwapChain::new(&device, &sj::gfx::SwapChainInfo::new());

    loop {
        queue.present(&mut swap_chain, 1);
        std::thread::sleep(std::time::Duration::from_millis(16));
    }
}
