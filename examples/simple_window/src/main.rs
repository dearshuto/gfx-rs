fn main() {
	let device = sj::gfx::Device::new(&sj::gfx::DeviceInfo::new());
	let _swap_chain = sj::gfx::SwapChain::new(&device, &sj::gfx::SwapChainInfo::new());
}
