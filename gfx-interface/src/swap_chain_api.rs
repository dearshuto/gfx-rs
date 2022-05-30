use sjvi::IDisplayEventListener;

pub struct SwapChainInfo {
    width: u32,
    height: u32,
}

impl SwapChainInfo {
    pub fn new() -> Self {
        Self {
            width: 640,
            height: 480,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn with_width(mut self, width: u32) -> Self {
        self.width = width;
        self
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn with_height(mut self, height: u32) -> Self {
        self.height = height;
        self
    }
}

pub trait ISwapChain: IDisplayEventListener {
    type ColorTargetViewType;
    type DeviceType;
    type SemaphoreType;
    type FenceType;

    fn new(device: &mut Self::DeviceType, info: &SwapChainInfo) -> Self;

    fn acquire_next_scan_buffer_view(
        &mut self,
        semaphore: Option<&mut Self::SemaphoreType>,
        fence: Option<&mut Self::FenceType>,
    ) -> Self::ColorTargetViewType;
}
