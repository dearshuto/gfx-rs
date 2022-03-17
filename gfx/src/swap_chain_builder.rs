use sjgfx_interface::{ISwapChain, SwapChainInfo};

pub struct TSwapChainBuilder<T: ISwapChain> {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ISwapChain> TSwapChainBuilder<T> {
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 480,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::DeviceType) -> T {
        T::new(device, &SwapChainInfo::new())
    }
}
