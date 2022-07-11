use sjgfx_ash::{BufferAsh, CommandBufferAsh, DeviceAsh, QueueAsh, ShaderAsh};
use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IBuffer, ICommandBuffer, IDevice, IQueue,
    IShader, QueueInfo, ShaderInfo,
};

fn main() {
    run::<DeviceAsh, QueueAsh, CommandBufferAsh, ShaderAsh, BufferAsh>();
}

fn run<TDevice, TQueue, TCommandBuffer, TShader, TBuffer>()
where
    TDevice: IDevice,
    TQueue: IQueue<DeviceType = TDevice, CommandBufferType = TCommandBuffer>,
    TCommandBuffer:
        ICommandBuffer<DeviceType = TDevice, BufferType = TBuffer, ShaderType = TShader>,
    TShader: IShader<DeviceType = TDevice>,
    TBuffer: IBuffer<DeviceType = TDevice>,
{
    let mut device = TDevice::new(&DeviceInfo::new());
    let mut queue = TQueue::new(&device, &QueueInfo::new());
    let mut command_buffer = TCommandBuffer::new(&device, &CommandBufferInfo::new());

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
    let shader = TShader::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );

    let buffer = TBuffer::new(
        &mut device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
            .set_size(std::mem::size_of::<u32>() * 64),
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    buffer.invalidate_mapped_range(0 /*offset*/, std::mem::size_of::<u32>() * 4);
    buffer.map_as_slice(|x: &[u32]| {
        println!("{}, {}, {}, {}, {}", x[0], x[1], x[2], x[3], x[4]);
    });
}
