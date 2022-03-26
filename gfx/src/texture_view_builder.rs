use sjgfx_interface::{ITextureView, TextureViewInfo};

use crate::api::IApi;

pub struct TTextureViewBuilder<TApi: IApi> {
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> TTextureViewBuilder<TApi> {
    pub fn new() -> Self {
        Self {
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &TApi::Device, texture: &TApi::Texture) -> TApi::TextureView {
        TApi::TextureView::new(device, &TextureViewInfo::new(), texture)
    }
}
