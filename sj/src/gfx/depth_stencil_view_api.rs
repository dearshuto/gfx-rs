use super::{Device, Texture};

pub struct DepthStencilViewInfo<'a> {
    _texture: &'a Texture<'a>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> DepthStencilViewInfo<'a> {
    pub fn new(texture: &'a Texture<'a>) -> Self {
        Self {
            _texture: texture,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn get_texture(&self) -> &'a Texture {
        self._texture
    }
}

pub trait IDepthStencilView<'a> {
    fn new(device: &'a Device, info: &DepthStencilViewInfo) -> Self;
}

pub struct TDepthStencilView<'a, T: IDepthStencilView<'a>> {
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: IDepthStencilView<'a>> TDepthStencilView<'a, T> {
    pub fn new(device: &'a Device, info: &DepthStencilViewInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }
}
