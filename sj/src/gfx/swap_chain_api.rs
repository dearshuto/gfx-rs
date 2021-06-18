use super::super::vi::Layer;
use super::{ColorTargetView, Device, Fence, Semaphore, Texture};

pub struct SwapChainInfo<'a> {
    _layer: &'a mut Layer,
}

impl<'a> SwapChainInfo<'a> {
    pub fn new(layer: &'a mut Layer) -> Self {
        SwapChainInfo { _layer: layer }
    }

    pub fn get_layer(&'a mut self) -> &'a mut super::super::vi::Layer {
        self._layer
    }
}

pub trait ISwapChainImpl<'a> {
    fn new(device: &'a Device, info: &'a mut SwapChainInfo<'a>) -> Self;

    fn get_scan_buffer_views_mut(&mut self) -> &mut [ColorTargetView<'a>];

    fn get_scan_buffers_mut(&mut self) -> &mut [Texture<'a>];

    fn get_scan_buffers_and_views(&mut self) -> (&mut [Texture<'a>], &mut [ColorTargetView<'a>]);

    fn acquire_next_scan_buffer_index(
        &mut self,
        semaphore: Option<&mut Semaphore>,
        fence: Option<&mut Fence>,
    ) -> i32;

    fn update(&mut self);
}

pub struct TSwapChain<'a, T>
where
    T: ISwapChainImpl<'a>,
{
    _impl: T,
    _marker_a: std::marker::PhantomData<&'a u32>,
}

impl<'a, T: ISwapChainImpl<'a>> TSwapChain<'a, T> {
    pub fn new(device: &'a Device, info: &'a mut SwapChainInfo<'a>) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker_a: std::marker::PhantomData,
        }
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

    // モジュール内に隠蔽したい
    pub fn update(&mut self) {
        self.to_data_mut().update();
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }

    pub fn to_data_mut(&mut self) -> &mut T {
        &mut self._impl
    }
}
