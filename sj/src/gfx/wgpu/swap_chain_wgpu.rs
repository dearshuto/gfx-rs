use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};

pub struct SwapChainWgpu {}

impl<'a> ISwapChainImpl<'a> for SwapChainWgpu {
    fn new(device: &Device, info: &'a mut SwapChainInfo<'a>) -> Self {
        todo!();
    }

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>] {
        todo!()
    }

    fn get_scan_buffers_mut(&mut self) -> &mut [crate::gfx::Texture<'a>] {
        todo!()
    }

    fn get_scan_buffers_and_views(
        &mut self,
    ) -> (&mut [crate::gfx::Texture<'a>], &mut [ColorTargetView<'a>]) {
        todo!()
    }

    fn acquire_next_scan_buffer_index(
        &mut self,
        semaphore: Option<&mut crate::gfx::Semaphore>,
        fence: Option<&mut crate::gfx::Fence>,
    ) -> i32 {
        todo!()
    }

    fn update(&mut self) {
        todo!();
    }
}
