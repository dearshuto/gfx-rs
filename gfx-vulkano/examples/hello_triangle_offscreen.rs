use sjgfx_interface::{
    BufferInfo, ColorTargetViewInfo, CommandBufferInfo, DeviceInfo, GpuAccess, IDevice,
    ImageFormat, PrimitiveTopology, ShaderInfo, TextureInfo, VertexStateInfo,
};
use sjgfx_vulkano::{
    BufferVk, ColorTargetViewVk, CommandBufferVk, DeviceVk, ShaderVk, TextureVk, VertexStateVk,
};

fn main() {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_triangle.vs"),
            shaderc::ShaderKind::Vertex,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &include_str!("../../resources/examples/shaders/hello_triangle.fs"),
            shaderc::ShaderKind::Fragment,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();

    let device = DeviceVk::new(&DeviceInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());
    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
    );
    let vertex_state = VertexStateVk::new(&device, &VertexStateInfo::new());
    let vertex_buffer = BufferVk::new(&device, &BufferInfo::new());

    let texture = TextureVk::new(
        &device,
        &TextureInfo::new()
            .set_width(640)
            .set_height(480)
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
            .set_image_format(ImageFormat::R8G8B8A8Unorm),
    );
    let color_target_view = ColorTargetViewVk::new(
        &device,
        &ColorTargetViewInfo::new().set_image_format(ImageFormat::R8G8B8A8Unorm),
        &texture,
    );

    command_buffer.begin();
    command_buffer.set_render_targets_ref([&color_target_view].into_iter(), None);
    command_buffer.set_shader(&shader);
    command_buffer.set_vertex_state(&vertex_state);
    command_buffer.set_vertex_buffer(0, &vertex_buffer);
    command_buffer.draw(
        PrimitiveTopology::TriangleList,
        3, /*vertex_count*/
        0, /*vertex_offset*/
    );
    command_buffer.end();
}
