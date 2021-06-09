use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};

pub struct SwapChainWgpu {}

impl<'a> ISwapChainImpl<'a> for SwapChainWgpu {
    fn new(device: &Device, info: &'a mut SwapChainInfo<'a>) -> Self {
        todo!();
    }

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView] {
        todo!();
    }

    fn update(&mut self) {
        todo!();
    }
}
