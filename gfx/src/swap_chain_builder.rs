use sjgfx_interface::{ISwapChain, SwapChainInfo};

use crate::api::IApi;

pub struct TSwapChainBuilder<T: IApi> {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TSwapChainBuilder<T> {
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 480,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &mut T::Device) -> T::SwapChain {
        T::SwapChain::new(device, &SwapChainInfo::new())
    }
}
