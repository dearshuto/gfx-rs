use sjgfx::wgpu::*;
use sjgfx::CommandBufferBuilder;
use sjgfx::DeviceBuilder;
use sjgfx::QueueBuilder;

#[test]
fn new() {
    let device = DeviceBuilder::new().build();
    let queue = QueueBuilder::new().build(&device);
    let command_buffer = CommandBufferBuilder::new().build(&device);

    command_buffer.begin();
    command_buffer.end();

    //queue.execute(&command_buffer);
    queue.flush();
}
