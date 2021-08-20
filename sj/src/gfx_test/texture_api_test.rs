use crate::gfx::{
    Device, DeviceInfo, GpuAccess, ImageFormat, MemoryPool, MemoryPoolInfo, MemoryPoolProperty,
    Texture, TextureInfo,
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
    let _texture = Texture::new(&device, &texture_info, &memory_pool, 0, required_size);
}
