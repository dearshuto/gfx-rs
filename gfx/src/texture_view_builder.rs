use sjgfx_interface::{ITextureView, ImageFormat, TextureViewInfo};

use crate::api::IApi;

pub struct TTextureViewBuilder<TApi: IApi> {
    info: TextureViewInfo,
    _marker: std::marker::PhantomData<TApi>,
}

impl<TApi: IApi> TTextureViewBuilder<TApi> {
    pub fn new() -> Self {
        Self {
            info: TextureViewInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &TApi::Device, texture: &TApi::Texture) -> TApi::TextureView {
        TApi::TextureView::new(device, &self.info, texture)
    }

    pub fn with_format(self, image_format: ImageFormat) -> Self {
        Self {
            info: self.info.set_format(image_format),
            _marker: std::marker::PhantomData,
        }
    }
}
