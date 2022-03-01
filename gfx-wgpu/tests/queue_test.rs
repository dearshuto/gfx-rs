use sjgfx_interface::{IDevice, DeviceInfo, QueueInfo, CommandBufferInfo, ShaderInfo, BufferInfo, GpuAccess};
use sjwgpu_wgpu::{DeviceWgpu, QueueWgpu, CommandBufferWgpu, ShaderWgpu, BufferWgpu};

#[test]
fn new() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let _queue = QueueWgpu::new(&device, &QueueInfo::new());
}

#[test]
fn flush_empty() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let queue = QueueWgpu::new(&device, &QueueInfo::new());
    queue.flush();
}

#[test]
fn flush_sync() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let queue = QueueWgpu::new(&device, &QueueInfo::new());
    queue.sync();
}

#[test]
fn execute_compute_command() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let buffer = BufferWgpu::new(&device, &BufferInfo::new().set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER).set_size(1024));
    // シェーダ
    let shader_source = include_str!("../../resources/tests/simple_compute.glsl");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let shader_binary = compiler
        .compile_into_spirv(
            &shader_source,
            shaderc::ShaderKind::Compute,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderWgpu::new(&device, &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()));

    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());
    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    buffer.map::<[u32; 64]>(|x| {
        assert_eq!(x[0], 0);
        assert_eq!(x[1], 1);
        assert_eq!(x[2], 2);
        assert_eq!(x[3], 3);
        assert_eq!(x[4], 4);
    });
}
