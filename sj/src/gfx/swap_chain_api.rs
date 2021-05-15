use super::Device;

pub struct SwapChainInfo<'a> {
    _layer: super::super::vi::Layer<'a>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> SwapChainInfo<'a> {
    pub fn new(layer: super::super::vi::Layer<'a>) -> Self {
        SwapChainInfo {
            _layer: layer,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_layer(self) -> super::super::vi::Layer<'a> {
        self._layer
    }
}

pub trait ISwapChainImpl<'a, 'ref_layer: 'a, 'layer: 'ref_layer> {
    fn new(device: &Device, info: SwapChainInfo<'a>) -> Self;

    fn update(&mut self);
}

pub struct TSwapChain<'a, 'ref_layer: 'a, 'layer: 'ref_layer, T>
where
    T: ISwapChainImpl<'a, 'ref_layer, 'layer>,
{
    _impl: T,
    _marker_a: std::marker::PhantomData<&'a u32>,
    _marker_b: std::marker::PhantomData<&'layer u32>,
    _marker_c: std::marker::PhantomData<&'ref_layer u32>,
}

impl<'a, 'ref_layer: 'a, 'layer: 'ref_layer, T: ISwapChainImpl<'a, 'ref_layer, 'layer>>
    TSwapChain<'a, 'ref_layer, 'layer, T>
{
    pub fn new(device: &Device, info: SwapChainInfo<'a>) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker_a: std::marker::PhantomData,
            _marker_b: std::marker::PhantomData,
            _marker_c: std::marker::PhantomData,
        }
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
