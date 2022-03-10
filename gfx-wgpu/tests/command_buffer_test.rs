use sjgfx_interface::{BufferInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IDevice, ShaderInfo};
use sjgfx_wgpu::{BufferWgpu, CommandBufferWgpu, DeviceWgpu, ShaderWgpu};

#[test]
fn new() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let _command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());
}

#[test]
fn begin_end() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.end();
}

#[test]
fn simple_compute_command() {
    let device = DeviceWgpu::new(&DeviceInfo::new());

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
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );

    let buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
            .set_size(1024),
    );
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();
}

#[test]
fn simple_graphics_command() {
    let device = DeviceWgpu::new(&DeviceInfo::new());
    let command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.end();
}
