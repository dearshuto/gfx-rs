use super::{Device, GpuAccess, ImageFormat, MemoryPool};

pub struct TextureInfo {
    _width: i32,
    _height: i32,
    _depth: i32,
    _gpu_access: GpuAccess,
    _image_format: ImageFormat,
}

impl TextureInfo {
    pub fn new() -> Self {
        Self {
            _width: 1,
            _height: 1,
            _depth: 1,
            _gpu_access: GpuAccess::empty(),
            _image_format: ImageFormat::R8G8B8A8Unorm,
        }
    }

    pub fn get_width(&self) -> i32 {
        self._width
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self._width = width;
        self
    }

    pub fn get_height(&self) -> i32 {
        self._height
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self._height = height;
        self
    }

    pub fn get_depth(&self) -> i32 {
        self._depth
    }

    pub fn set_depth(mut self, depth: i32) -> Self {
        self._depth = depth;
        self
    }

    pub fn get_gpu_access(&self) -> &GpuAccess {
        &self._gpu_access
    }

    pub fn set_gpu_access(mut self, gpu_access: GpuAccess) -> Self {
        self._gpu_access = gpu_access;
        self
    }

    pub fn get_image_format(&self) -> &ImageFormat {
        &self._image_format
    }

    pub fn set_image_format(mut self, image_format: ImageFormat) -> Self {
        self._image_format = image_format;
        self
    }
}

pub trait ITexture<'a> {
    fn calculate_required_size(device: &Device, info: &TextureInfo) -> u64;

    fn calculate_required_alignment(device: &Device, info: &TextureInfo) -> u64;

    fn new(
        device: &'a Device,
        info: &TextureInfo,
        memory_pool: &MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self;
}

pub struct TTexture<'a, T>
where
    T: ITexture<'a>,
{
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: ITexture<'a>> TTexture<'a, T> {
    pub fn calculate_required_size(device: &Device, info: &TextureInfo) -> u64 {
        T::calculate_required_size(device, info)
    }

    pub fn calculate_required_alignment(device: &Device, info: &TextureInfo) -> u64 {
        T::calculate_required_alignment(device, info)
    }

    pub fn new(
        device: &'a Device,
        info: &TextureInfo,
        memory_pool: &MemoryPool,
        offset: i64,
        size: u64,
    ) -> Self {
        Self {
            _impl: T::new(device, info, memory_pool, offset, size),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }
}
