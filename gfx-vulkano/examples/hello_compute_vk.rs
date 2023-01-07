use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, QueueInfo, ShaderInfo,
};
use sjgfx_vulkano::{BufferVk, CommandBufferVk, DeviceVk, QueueVk, ShaderVk};

fn main() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    let shader_source = include_str!("resources/shaders/hello_compute.glsl");
    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let shader_binary = compiler.create_binary(&shader_source, sjgfx_util::ShaderStage::Compute);
    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(&shader_binary),
    );

    let buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_size(std::mem::size_of::<u32>() * 64)
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    buffer.map_as_array(|x: &[u32]| {
        for item in x {
            print!("{} ", item);
        }

        println!();
    });
}
