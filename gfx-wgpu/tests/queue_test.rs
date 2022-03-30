use sjgfx_interface::{
    BufferInfo, CommandBufferInfo, DebugMode, DeviceInfo, GpuAccess, IDevice, ImageFormat,
    QueueInfo, ShaderInfo, TextureInfo, TextureViewInfo,
};
use sjgfx_wgpu::{
    BufferWgpu, CommandBufferWgpu, DeviceWgpu, QueueWgpu, ShaderWgpu, TextureViewWgpu, TextureWgpu,
};

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
    let buffer = BufferWgpu::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER)
            .set_size(1024),
    );
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
    let shader = ShaderWgpu::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );

    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());
    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_unordered_access_buffer(0, &buffer);
    command_buffer.dispatch(1, 1, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    buffer.map(|x: &[u32; 64]| {
        assert_eq!(x[0], 0);
        assert_eq!(x[1], 1);
        assert_eq!(x[2], 2);
        assert_eq!(x[3], 3);
        assert_eq!(x[4], 4);
    });
}

#[test]
fn image_write_test() {
    let device = DeviceWgpu::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
    let mut queue = QueueWgpu::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWgpu::new(&device, &CommandBufferInfo::new());

    let shader_source = "
        		#version 450

            layout (local_size_x=8, local_size_y=8, local_size_z=1) in;

            layout (binding=0, r8ui) uniform uimage2D u_Image;

        		void main() {
              int x = int(gl_GlobalInvocationID.x);
              int y = int(gl_GlobalInvocationID.y);

              imageStore(u_Image, ivec2(x, y), uvec4(1, 0, 0, 0));
        		}";
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

    let image = TextureWgpu::new(
        &device,
        &TextureInfo::new()
            .set_width(640)
            .set_height(640)
            .set_gpu_access_flags(GpuAccess::IMAGE)
            .set_image_format(sjgfx_interface::ImageFormat::R8Uint),
    );
    let view = TextureViewWgpu::new(
        &device,
        &TextureViewInfo::new().set_format(ImageFormat::R8Uint),
        &image,
    );

    command_buffer.begin();
    command_buffer.set_shader(&shader);
    command_buffer.set_image(0, &view);
    command_buffer.dispatch(8, 8, 1);
    command_buffer.end();

    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();
}
