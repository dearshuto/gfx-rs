use sjgfx_interface::ISwapChain;
use sjvi::IDisplayEventListener;

use crate::{ColorTargetViewWsys, DeviceWsys, FenceWsys, SemaphoreWsys};

pub struct SwapChainWsys {
    scan_buffer: ColorTargetViewWsys,
}

impl ISwapChain for SwapChainWsys {
    type ColorTargetViewType = ColorTargetViewWsys;
    type DeviceType = DeviceWsys;
    type SemaphoreType = SemaphoreWsys;
    type FenceType = FenceWsys;

    fn new(_device: &mut Self::DeviceType, _info: &sjgfx_interface::SwapChainInfo) -> Self {
        Self {
            scan_buffer: ColorTargetViewWsys::new_direct()
        }
    }

    fn acquire_next_scan_buffer_view(
        &mut self,
        _semaphore: Option<&mut Self::SemaphoreType>,
        _fence: Option<&mut Self::FenceType>,
    ) -> &mut Self::ColorTargetViewType {
        &mut self.scan_buffer
    }
}

impl IDisplayEventListener for SwapChainWsys {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}
