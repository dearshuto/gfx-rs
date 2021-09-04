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

#[cfg(test)]
pub mod tests {
    use crate::gfx::{
        ColorTargetView, ColorTargetViewInfo, Device, DeviceInfo, GpuAccess, ImageFormat,
        MemoryPool, MemoryPoolInfo, MemoryPoolProperty, Texture, TextureInfo,
    };

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let texture_info = TextureInfo::new()
            .set_width(640)
            .set_height(480)
            .set_image_format(ImageFormat::R8G8B8A8Unorm)
            .set_gpu_access_flags(GpuAccess::TEXTURE);

        let required_size = Texture::calculate_required_size(&device, &texture_info);
        let memory_pool = MemoryPool::new(
            &device,
            &MemoryPoolInfo::new()
                .set_size(required_size)
                .set_memory_pool_property(
                    MemoryPoolProperty::CPU_INVISIBLE | MemoryPoolProperty::GPU_CACHED,
                ),
        );
        let texture = Texture::new(&device, &texture_info, &memory_pool, 0, required_size);

        let color_target_view_info =
            ColorTargetViewInfo::new(&texture).set_image_format(ImageFormat::R8G8B8A8Unorm);
        let _color_target_view = ColorTargetView::new(&device, &color_target_view_info);
    }
}
