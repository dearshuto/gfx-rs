use crate::gfx::swap_chain_api::ISwapChainImpl;
use crate::gfx::{Device, SwapChainInfo};
use crate::vi::Layer;

pub struct SwapChainVk<'a> {
    _layer: &'a Layer,
}

impl<'a> ISwapChainImpl<'a> for SwapChainVk<'a> {
    fn new(_device: &'a Device, _info: &'a mut SwapChainInfo<'a>) -> Self {
        todo!()
    }

    fn get_scan_buffer_views_mut(&mut self) -> &mut [crate::gfx::ColorTargetView<'a>] {
        todo!()
    }

    fn get_scan_buffers_mut(&mut self) -> &mut [crate::gfx::Texture<'a>] {
        todo!()
    }

    fn get_scan_buffers_and_views(
        &mut self,
    ) -> (
        &mut [crate::gfx::Texture<'a>],
        &mut [crate::gfx::ColorTargetView<'a>],
    ) {
        todo!()
    }

    fn acquire_next_scan_buffer_index(
        &mut self,
        _semaphore: Option<&mut crate::gfx::Semaphore>,
        _fence: Option<&mut crate::gfx::Fence>,
    ) -> i32 {
        todo!()
    }

    fn update(&mut self) {
        todo!()
    }
}
