use winit::event_loop::EventLoop;
use winit::window::Window;

use super::super::swap_chain_api::{ISwapChainImpl, SwapChainInfo};
use super::super::{ColorTargetView, Device};

pub struct SwapChainWgpu<'a> {
    _event_loop: &'a mut EventLoop<()>,
    _window: &'a Window,
}

impl<'a> SwapChainWgpu<'a> {
    pub fn get_event_loop(&self) -> &EventLoop<()> {
        &self._event_loop
    }

    pub fn get_event_loop_mut(&mut self) -> &mut EventLoop<()> {
        self._event_loop
    }
}

impl<'a> ISwapChainImpl<'a> for SwapChainWgpu<'a> {
    fn new(_device: &Device, info: &'a mut SwapChainInfo<'a>) -> Self {
        let (event_loop, window) = info.get_layer().get_event_loop_and_window_mut();

        Self {
            _event_loop: event_loop,
            _window: window,
        }
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

    fn update(&mut self) {
        todo!();
    }
}
