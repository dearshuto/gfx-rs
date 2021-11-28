use super::{ColorTargetView, Device, Fence, ScanBufferView, Semaphore, Texture};

pub struct SwapChainInfo {}

impl<'a> SwapChainInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait ISwapChainImpl<'a> {
    fn new(device: &'a Device, info: &SwapChainInfo) -> Self;

    fn acquire_next_scan_buffer_view(&self) -> ScanBufferView;

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>];

    fn get_scan_buffers_mut(&mut self) -> &mut [Texture<'a>];

    fn get_scan_buffers_and_views(&mut self) -> (&mut [Texture<'a>], &mut [ColorTargetView<'a>]);

    fn acquire_next_scan_buffer_index(
        &mut self,
        semaphore: Option<&mut Semaphore>,
        fence: Option<&mut Fence>,
    ) -> i32;
}

pub struct TSwapChain<'a, T>
where
    T: ISwapChainImpl<'a>,
{
    _impl: T,
    _marker_a: std::marker::PhantomData<&'a u32>,
}

impl<'a, T: ISwapChainImpl<'a>> TSwapChain<'a, T> {
    pub fn new(device: &'a Device, info: &SwapChainInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker_a: std::marker::PhantomData,
        }
    }

    pub fn acquire_next_scan_buffer_view(&self) -> ScanBufferView {
        self._impl.acquire_next_scan_buffer_view()
    }

    pub fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>] {
        self._impl.get_scan_buffer_views_mut()
    }

    pub fn get_scan_buffers_mut(&mut self) -> &mut [Texture<'a>] {
        self._impl.get_scan_buffers_mut()
    }

    pub fn get_scan_buffers_and_views(
        &mut self,
    ) -> (&mut [Texture<'a>], &mut [ColorTargetView<'a>]) {
        self._impl.get_scan_buffers_and_views()
    }

    pub fn acquire_next_scan_buffer_index(
        &mut self,
        semaphore: Option<&mut Semaphore>,
        fence: Option<&mut Fence>,
    ) -> i32 {
        self._impl.acquire_next_scan_buffer_index(semaphore, fence)
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self._impl
    }
}
