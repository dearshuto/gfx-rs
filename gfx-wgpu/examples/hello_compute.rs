use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IDevice, QueueInfo, ShaderInfo,
};
use sjgfx_wgpu::{BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu};

fn main() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let mut compiler = shaderc::Compiler::new().unwrap();
    let shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_compute.glsl"),
            shaderc::ShaderKind::Compute,
            "compute.glsl",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );

    let buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
            .set_size(std::mem::align_of::<u32>() * 64),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&mut command_buffer);
    queue.flush();
    queue.sync();

    buffer.map_as_slice(64, |x: &[u32]| {
        for value in x {
            println!("{}", value);
        }
    });
}
