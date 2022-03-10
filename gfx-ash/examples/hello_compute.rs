use sjgfx_ash::{BufferAsh, CommandBufferAsh, DeviceAsh, QueueAsh, ShaderAsh};
use sjgfx_interface::{CommandBufferInfo, DeviceInfo, QueueInfo, BufferInfo, GpuAccess, ShaderInfo};

fn main() {
    let device = DeviceAsh::new(&DeviceInfo::new());
    let mut queue = QueueAsh::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferAsh::new(&device, &CommandBufferInfo::new());

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
    let shader = ShaderAsh::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );

    let buffer = BufferAsh::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER).set_size(std::mem::size_of::<u32>() * 64));

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    buffer.invalidate_mapped_range(0/*offset*/, std::mem::size_of::<u32>() * 4);
    buffer.map_as_slice(64, |x: &[u32]| {
        println!("{}, {}, {}, {}, {}", x[0], x[1], x[2], x[3], x[4]);
    });
}
