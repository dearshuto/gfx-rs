use sjgfx_interface::{ISwapChain, SwapChainInfo};

use crate::api::IApi;

pub struct TSwapChainBuilder<T: IApi> {
    info: SwapChainInfo,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TSwapChainBuilder<T> {
    pub fn new() -> Self {
        Self {
            info: SwapChainInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &mut T::Device) -> T::SwapChain {
        T::SwapChain::new(device, &self.info)
    }

    pub fn with_width(self, width: u32) -> Self {
        Self {
            info: self.info.with_width(width),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_height(self, height: u32) -> Self {
        Self {
            info: self.info.with_height(height),
            _marker: std::marker::PhantomData,
        }
    }
}
