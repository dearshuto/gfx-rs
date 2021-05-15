pub struct SwapChain<'a> {
    layer: &'a mut super::super::super::vi::Layer<'a>,
}

impl<'a> SwapChain<'a> {
    pub fn new(
        _device: &super::device_vk::Device,
        info: &'a mut super::super::swap_chain::SwapChainInfo<'a>,
    ) -> SwapChain<'a> {
        let layer = info.get_layer().unwrap();
        SwapChain { layer }
    }

    pub fn get_layer(&mut self) -> &'a mut super::super::super::vi::Layer {
        self.layer
    }
}
