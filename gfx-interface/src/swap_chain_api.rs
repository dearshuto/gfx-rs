pub struct SwapChainInfo {}

impl SwapChainInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISwapChain {
    type ColorTargetViewType;
    type DeviceType;
    type SemaphoreType;
    type FenceType;

    fn new(device: &Self::DeviceType, info: &SwapChainInfo) -> Self;

    fn acquire_next_scan_buffer_view(
        &mut self,
        semaphore: Option<&mut Self::SemaphoreType>,
        fence: Option<&mut Self::FenceType>,
    ) -> Self::ColorTargetViewType;
}
