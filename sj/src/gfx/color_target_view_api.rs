use super::{Device, ImageFormat, Texture};

pub struct ColorTargetViewInfo<'a> {
    _texture: &'a Texture<'a>,
    _image_format: ImageFormat,
}

impl<'a> ColorTargetViewInfo<'a> {
    pub fn new(texture: &'a Texture<'a>) -> Self {
        Self {
            _texture: texture,
            _image_format: ImageFormat::R8G8B8A8Unorm,
        }
    }

    pub fn get_texture(&self) -> &'a Texture {
        self._texture
    }

    pub fn get_image_format(&self) -> &ImageFormat {
        &self._image_format
    }

    pub fn set_image_format(mut self, image_format: ImageFormat) -> Self {
        self._image_format = image_format;
        self
    }
}

pub trait IColorTargetViewImpl<'a> {
    fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self;
}

pub struct TColorTargetView<'a, T>
where
    T: IColorTargetViewImpl<'a>,
{
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: IColorTargetViewImpl<'a>> TColorTargetView<'a, T> {
    pub fn new(device: &'a Device, info: &'a ColorTargetViewInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn new_internal(impl_instance: T) -> Self {
        Self {
            _impl: impl_instance,
            _marker: std::marker::PhantomData,
        }
    }
    pub fn to_data(&self) -> &T {
        &self._impl
    }
}

pub struct TRenderTargetSlice<'a, T>
where
    T: IColorTargetViewImpl<'a>,
{
    _color_target_views: &'a [TColorTargetView<'a, T>],
}

impl<'a, T> TRenderTargetSlice<'a, T>
where
    T: IColorTargetViewImpl<'a>,
{
    pub fn new(color_target_views: &'a [TColorTargetView<'a, T>]) -> Self {
        Self {
            _color_target_views: color_target_views,
        }
    }
}
