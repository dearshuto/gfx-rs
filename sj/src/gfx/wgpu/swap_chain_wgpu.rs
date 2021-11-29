use crate::gfx::ScanBufferView;

use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};
use super::scan_buffer_view_wgpu::ScanBufferViewWgpu;

pub struct SwapChainWgpu<'a> {
    _device: &'a Device,
}

impl<'a> SwapChainWgpu<'a> {}

impl<'a> ISwapChainImpl<'a> for SwapChainWgpu<'a> {
    fn new(device: &'a Device, _info: &SwapChainInfo) -> Self {
        Self { _device: device }
    }

    fn acquire_next_scan_buffer_view(&self) -> ScanBufferView {
        let surface = self._device.to_data().try_get_surface().unwrap();
        let format = surface
            .get_preferred_format(self._device.to_data().get_adapter())
            .unwrap();
        let frame = surface.get_current_frame().unwrap();
        let scan_buffer_view_wgpu = ScanBufferViewWgpu::new(frame, format);
        ScanBufferView::new(scan_buffer_view_wgpu)
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
        _semaphore: Option<&mut crate::gfx::Semaphore>,
        _fence: Option<&mut crate::gfx::Fence>,
    ) -> i32 {
        todo!()
    }
}
