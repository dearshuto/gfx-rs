use sjgfx_interface::{GpuAccess, ITexture, ImageFormat, TextureInfo};

use crate::api::IApi;

pub struct TTextureBuilder<'a, TApi: IApi> {
    info: TextureInfo,
    data: Option<&'a [u8]>,
    _marker: std::marker::PhantomData<TApi>,
}

impl<'a, TApi: IApi> TTextureBuilder<'a, TApi> {
    pub fn new() -> Self {
        Self {
            info: TextureInfo::new(),
            data: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &TApi::Device) -> TApi::Texture {
        if let Some(data) = self.data {
            TApi::Texture::new_with_data(device, &self.info, data)
        } else {
            TApi::Texture::new(device, &self.info)
        }
    }

    pub fn enable_sampler(self) -> Self {
        self.enable_bit(GpuAccess::TEXTURE)
    }

    pub fn enable_image(self) -> Self {
        self.enable_bit(GpuAccess::IMAGE)
    }

    pub fn with_size(self, width: i32, height: i32) -> Self {
        Self {
            info: self.info.set_width(width).set_height(height),
            data: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_format(self, format: ImageFormat) -> Self {
        Self {
            info: self.info.set_image_format(format),
            data: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn with_data(self, data: &'a [u8]) -> Self {
        Self {
            info: self.info,
            data: Some(data),
            _marker: std::marker::PhantomData,
        }
    }

    fn enable_bit(self, gpu_access: GpuAccess) -> Self {
        let gpu_access = *self.info.get_gpu_access_flags() | gpu_access;

        Self {
            info: self.info.set_gpu_access_flags(gpu_access),
            data: None,
            _marker: std::marker::PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::ImageFormat;

    use crate::{api, TDeviceBuilder, TTextureBuilder};

    #[test]
    fn new() {
        let device = TDeviceBuilder::<api::Ash>::new().build();
        let texture_builder = TTextureBuilder::<api::Ash>::new()
            .with_size(640, 640)
            .with_format(ImageFormat::R8Uint)
            .enable_sampler()
            .enable_image();

        let _ = texture_builder.build(&device);
        let _ = texture_builder.build(&device);
    }
}
