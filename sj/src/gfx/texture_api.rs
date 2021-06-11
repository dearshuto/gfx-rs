use super::{Device, GpuAccess, ImageFormat, MemoryPool};

pub struct TextureInfo {
    _width: i32,
    _height: i32,
    _depth: i32,
    _gpu_access_flags: GpuAccess,
    _image_format: ImageFormat,
}

impl TextureInfo {
    pub fn new() -> Self {
        Self {
            _width: 1,
            _height: 1,
            _depth: 1,
            _gpu_access_flags: GpuAccess::empty(),
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

    pub fn get_gpu_access_flags(&self) -> &GpuAccess {
        &self._gpu_access_flags
    }

    pub fn set_gpu_access_flags(mut self, gpu_access: GpuAccess) -> Self {
        self._gpu_access_flags = gpu_access;
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

pub struct TextureCopyRegion {
    _offset_u: i32,
    _offset_v: i32,
    _offset_w: i32,
    _width: i32,
    _height: i32,
    _depth: i32,
    _array_length: i32,
    _texture_subresource: TextureSubResource,
}

impl TextureCopyRegion {
    pub fn new() -> Self {
        Self {
            _offset_u: 0,
            _offset_v: 0,
            _offset_w: 0,
            _width: 1,
            _height: 1,
            _depth: 1,
            _array_length: 1,
            _texture_subresource: TextureSubResource::new(),
        }
    }

    pub fn get_offset_u(&self) -> i32 {
        self._offset_u
    }

    pub fn set_offset_u(mut self, offset_u: i32) -> Self {
        self._offset_u = offset_u;
        self
    }

    pub fn get_offset_v(&self) -> i32 {
        self._offset_v
    }

    pub fn set_offset_v(mut self, offset_v: i32) -> Self {
        self._offset_v = offset_v;
        self
    }

    pub fn get_offset_w(&self) -> i32 {
        self._offset_w
    }

    pub fn set_offset_w(mut self, offset_w: i32) -> Self {
        self._offset_w = offset_w;
        self
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

    pub fn get_array_length(&self) -> i32 {
        self._array_length
    }

    pub fn set_array_length(mut self, array_length: i32) -> Self {
        self._array_length = array_length;
        self
    }

    pub fn get_texture_subresource(&self) -> &TextureSubResource {
        &self._texture_subresource
    }

    pub fn edit_texture_subresource(
        mut self,
        updater: fn(TextureSubResource) -> TextureSubResource,
    ) -> Self {
        self._texture_subresource = updater(self._texture_subresource);
        self
    }
}

pub struct TextureSubResource {
    _mip_level: i32,
    _array_index: i32,
}

impl TextureSubResource {
    pub fn new() -> Self {
        Self {
            _mip_level: 0,
            _array_index: 0,
        }
    }

    pub fn get_mip_level(&self) -> i32 {
        self._mip_level
    }

    pub fn set_mip_level(mut self, mip_level: i32) -> Self {
        self._mip_level = mip_level;
        self
    }

    pub fn get_array_index(&self) -> i32 {
        self._array_index
    }

    pub fn set_array_index(mut self, array_index: i32) -> Self {
        self._array_index = array_index;
        self
    }
}

pub struct BufferTextureCopyRegion {
    _offset: i32,
    _image_width: i32,
    _image_height: i32,
    _texture_copy_region: TextureCopyRegion,
}

impl BufferTextureCopyRegion {
    pub fn new() -> Self {
        Self {
            _offset: 0,
            _image_width: 1,
            _image_height: 1,
            _texture_copy_region: TextureCopyRegion::new(),
        }
    }

    pub fn get_offset(&self) -> i32 {
        self._offset
    }

    pub fn set_offset(mut self, offset: i32) -> Self {
        self._offset = offset;
        self
    }

    pub fn get_image_width(&self) -> i32 {
        self._image_width
    }

    pub fn set_image_width(mut self, image_width: i32) -> Self {
        self._image_width = image_width;
        self
    }

    pub fn get_image_height(&self) -> i32 {
        self._image_height
    }

    pub fn set_image_height(mut self, image_height: i32) -> Self {
        self._image_height = image_height;
        self
    }

    pub fn get_texture_copy_region(&self) -> &TextureCopyRegion {
        &self._texture_copy_region
    }

    pub fn edit_texture_copy_region(
        mut self,
        updater: fn(TextureCopyRegion) -> TextureCopyRegion,
    ) -> Self {
        self._texture_copy_region = updater(self._texture_copy_region);
        self
    }
}

pub struct TextureArrayRange {
    _length: i32,
    _base_index: i32,
}

impl TextureArrayRange {
    pub fn new() -> Self {
        Self {
            _length: 1,
            _base_index: 0,
        }
    }

    pub fn get_length(&self) -> i32 {
        self._length
    }

    pub fn set_length(mut self, length: i32) -> Self {
        self._length = length;
        self
    }

    pub fn get_base_index(&self) -> i32 {
        self._base_index
    }

    pub fn set_base_index(mut self, base_index: i32) -> Self {
        self._base_index = base_index;
        self
    }
}

pub struct MipRange {
    _min_mip_level: i32,
    _mip_count: i32,
}

impl MipRange {
    pub fn new() -> Self {
        Self {
            _min_mip_level: 1,
            _mip_count: 1,
        }
    }

    pub fn get_mip_level(&self) -> i32 {
        self._min_mip_level
    }

    pub fn set_mip_level(mut self, min_mip_level: i32) -> Self {
        self._min_mip_level = min_mip_level;
        self
    }

    pub fn get_mip_count(&self) -> i32 {
        self._mip_count
    }

    pub fn set_mip_count(mut self, mip_count: i32) -> Self {
        self._mip_count = mip_count;
        self
    }
}

pub struct TextureSubresourceRange {
    _texture_array_range: TextureArrayRange,
    _mip_range: MipRange,
}

impl TextureSubresourceRange {
    pub fn new() -> Self {
        Self {
            _texture_array_range: TextureArrayRange::new(),
            _mip_range: MipRange::new(),
        }
    }

    pub fn get_texture_subresource_range(&self) -> &TextureArrayRange {
        &self._texture_array_range
    }

    pub fn edit_texture_array_range(
        mut self,
        updater: fn(TextureArrayRange) -> TextureArrayRange,
    ) -> Self {
        self._texture_array_range = updater(self._texture_array_range);
        self
    }

    pub fn get_mip_range(&self) -> &MipRange {
        &self._mip_range
    }

    pub fn edit_mip_range(mut self, updater: fn(MipRange) -> MipRange) -> Self {
        self._mip_range = updater(self._mip_range);
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
