use sjgfx_interface::{
    AttributeFormat, BufferInfo, ColorTargetViewInfo, CommandBufferInfo, DeviceInfo, GpuAccess,
    ImageFormat, PrimitiveTopology, QueueInfo, ShaderInfo, TextureInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo, VertexStateInfo,
};
use sjgfx_vulkano::CommandBufferVk;
use sjgfx_vulkano::{
    BufferVk, ColorTargetViewVk, DeviceVk, QueueVk, ShaderVk, TextureVk, VertexStateVk,
};

#[test]
pub fn new_test() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let _queue = QueueVk::new(&device, &QueueInfo::new());
}

#[test]
pub fn execute_test() {
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

    let device = DeviceVk::new(&DeviceInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );
    let buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_size(1024)
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER),
    );

    for _index in 0..5 {
        command_buffer.begin();
        command_buffer.set_shader(&shader);
        command_buffer.set_constant_buffer(0, &buffer);
        command_buffer.dispatch(1, 1, 1);
        command_buffer.end();

        queue.execute(&command_buffer);
        queue.flush();
        queue.sync();
    }

    buffer.map(|x: &[u32; 64]| {
        assert_eq!(x[0], 0);
        assert_eq!(x[1], 1);
        assert_eq!(x[2], 2);
        assert_eq!(x[3], 3);
        assert_eq!(x[4], 4);
    });
}

#[test]
pub fn execute_array_test() {
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

    let device = DeviceVk::new(&DeviceInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
    );
    let buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_size(1024)
            .set_gpu_access_flags(GpuAccess::UNORDERED_ACCESS_BUFFER),
    );

    for _index in 0..5 {
        command_buffer.begin();
        command_buffer.set_shader(&shader);
        command_buffer.set_constant_buffer(0, &buffer);
        command_buffer.dispatch(1, 1, 1);
        command_buffer.end();

        queue.execute(&command_buffer);
        queue.flush();
        queue.sync();
    }

    buffer.map_as_array(|x: &[u32]| {
        assert_eq!(x[0], 0);
        assert_eq!(x[1], 1);
        assert_eq!(x[2], 2);
        assert_eq!(x[3], 3);
        assert_eq!(x[4], 4);
    });
}

#[test]
pub fn execute_hello_triangle() {
    let device = DeviceVk::new(&DeviceInfo::new());
    let mut queue = QueueVk::new(&device, &QueueInfo::new());
    let mut command_buffer = CommandBufferVk::new(&device, &CommandBufferInfo::new());

    let texture = TextureVk::new(
        &device,
        &TextureInfo::new()
            .set_width(640)
            .set_height(480)
            .set_gpu_access_flags(GpuAccess::TEXTURE | GpuAccess::COLOR_BUFFER)
            .set_image_format(ImageFormat::R8G8B8A8Unorm),
    );
    let color_target = ColorTargetViewVk::new(
        &device,
        &ColorTargetViewInfo::new().set_image_format(ImageFormat::R8G8B8A8Unorm),
        &texture,
    );

    let vertex_shader_source = include_str!("../../resources/tests/hello_triangle.vs");
    let pixel_shader_source = include_str!("../../resources/tests/hello_triangle.fs");
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &vertex_shader_source,
            shaderc::ShaderKind::Vertex,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &pixel_shader_source,
            shaderc::ShaderKind::Fragment,
            "test.glsl",
            "main",
            None,
        )
        .unwrap();
    let shader = ShaderVk::new(
        &device,
        &ShaderInfo::new()
            .set_vertex_shader_binary(&vertex_shader_binary.as_binary_u8())
            .set_pixel_shader_binary(&pixel_shader_binary.as_binary_u8()),
    );

    let vertex_state =
        VertexStateVk::new(
            &device,
            &VertexStateInfo::new()
                .set_attribute_state_info_array(
                    [VertexAttributeStateInfo::new()
                        .set_buffer_index(0)
                        .set_format(AttributeFormat::Float32_32)
                        .set_offset(0)
                        .set_slot(0)
                        .set_slot(0)]
                    .into_iter(),
                )
                .set_buffer_state_info_array(
                    [VertexBufferStateInfo::new()
                        .set_stride((std::mem::size_of::<f32>() * 2) as i64)]
                    .into_iter(),
                ),
        );

    let vertex_buffer = BufferVk::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(128),
    );

    {
        command_buffer.begin();
        command_buffer.set_render_targets_ref([&color_target].into_iter(), None);
        command_buffer.set_shader(&shader);
        command_buffer.set_vertex_state(&vertex_state);
        command_buffer.set_vertex_buffer(0, &vertex_buffer);
        command_buffer.draw(
            PrimitiveTopology::TriangleList,
            3, /*vertex_count*/
            0, /*vertex_offset*/
        );
        command_buffer.end();

        queue.execute(&command_buffer);
        queue.flush();
        queue.sync();
    }
}
