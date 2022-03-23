use sjgfx_interface::{ICommandBuffer, IQueue, ImageFormat};
use sjgfx::api::IApi;

use sjgfx::{TDeviceBuilder, TQueueBuilder, TCommandBufferBuilder, TTextureBuilder};

#[test]
fn new() {
    // let device = DeviceBuilder::new().build();
    // let queue = QueueBuilder::new().build(&device);
    // let command_buffer = CommandBufferBuilder::new().build(&device);

    // command_buffer.begin();
    // command_buffer.end();

    // //queue.execute(&command_buffer);
    // queue.flush();
}

#[test]
fn image() {
    image_impl::<sjgfx::api::Ash>();
    // image_impl::<sjgfx::api::Wgpu>();
}

fn image_impl<TApi: IApi>() {
    let device = TDeviceBuilder::<TApi>::new().build();
    let mut queue = TQueueBuilder::<TApi>::new().build(&device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);

    // let shader = TShaderBuilder::<TApi>::new().set_compute_shader_binary(&[]).build(&device);

    let texture = TTextureBuilder::<TApi>::new()
        .with_size(640, 640)
        .with_format(ImageFormat::R8Unorm)
        .enable_image()
        .build(&device);

    command_buffer.begin();
    // command_buffer.set_shader(&shader);
    command_buffer.set_image(0, &texture);
    command_buffer.dispatch(64, 64, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();
}
