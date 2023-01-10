use sjgfx_interface::{
    BufferCopyRegion, BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IDevice, QueueInfo,
    ShaderInfo,
};
use sjgfx_wgpu::{BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu};

fn main() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = sjgfx_util::ShaderCompiler::new();
    let shader_binary = compiler.create_binary(
        &include_str!("../../resources/examples/shaders/hello_compute.glsl"),
        sjgfx_util::ShaderStage::Compute,
    );
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(&shader_binary),
    );

    let buffer_size = std::mem::align_of::<u32>() * 64;
    let buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER | GpuAccess::READ)
            .set_size(buffer_size),
    );
    let mut dst_buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::WRITE)
            .set_size(buffer_size),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);

    command_buffer.copy_buffer_to_buffer(
        &mut dst_buffer,
        &buffer,
        &BufferCopyRegion::default().set_copy_size(buffer_size),
    );

    queue.flush();
    queue.sync();

    dst_buffer.map_as_slice(64, |x: &[u32]| {
        for value in x {
            println!("{}", value);
        }
    });
}
