use sjgfx::{api::IApi, TDeviceBuilder, TTextureBuilder};
use sjgfx_interface::ImageFormat;

#[test]
fn new_texture_sampler() {
    new_texture_sampler_impl::<sjgfx::api::Ash>();
}

fn new_texture_sampler_impl<TApi: IApi>() {
    let mut device = TDeviceBuilder::<TApi>::new().build();
    let _texture = TTextureBuilder::<TApi>::new()
        .with_size(640, 640)
        .with_format(ImageFormat::R8G8B8A8Unorm)
        .enable_sampler()
        .build(&mut device);

    let _texture = TTextureBuilder::<TApi>::new()
        .with_size(640, 640)
        .with_format(ImageFormat::R8Unorm)
        .enable_sampler()
        .build(&mut device);
}

#[test]
fn new_image() {
    new_image_impl::<sjgfx::api::Ash>();
}

fn new_image_impl<TApi: IApi>() {
    let mut device = TDeviceBuilder::<TApi>::new().build();
    let _texture = TTextureBuilder::<TApi>::new()
        .with_size(640, 640)
        .with_format(ImageFormat::R8Unorm)
        .enable_image()
        .build(&mut device);
}
