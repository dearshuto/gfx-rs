use sjgfx_ash::{DeviceAsh, TextureAsh};
use sjgfx_interface::{DeviceInfo, ITexture, TextureInfo, ImageFormat, GpuAccess};

fn main() {
    let device = DeviceAsh::new(&DeviceInfo::new());
    let _texture = TextureAsh::new(&device, &TextureInfo::new().set_width(640).set_height(640).set_image_format(ImageFormat::R8Uint).set_gpu_access_flags(GpuAccess::TEXTURE | GpuAccess::IMAGE));
}
