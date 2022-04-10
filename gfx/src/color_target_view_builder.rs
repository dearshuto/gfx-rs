use sjgfx_interface::ColorTargetViewInfo;
use sjgfx_interface::IColorTargetView;

use crate::api::IApi;

pub struct TColorTargetViewBuilder<TApi: IApi> {
    info: ColorTargetViewInfo,
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> TColorTargetViewBuilder<TApi> {
    pub fn new() -> Self {
        Self {
            info: ColorTargetViewInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &TApi::Device, texture: &TApi::Texture) -> TApi::ColorTargetView {
        TApi::ColorTargetView::new(device, &self.info, texture)
    }
}
