use sj::gfx::{
    AttributeFormat, BlendStateInfo, BlendTargetStateInfo, ColorTargetView, ColorTargetViewInfo,
    CommandBuffer, CommandBufferInfo, DepthStencilStateInfo, Device, DeviceInfo, GpuAccess,
    GraphicsPipelineInfo, ImageFormat, MemoryPool, MemoryPoolInfo, MemoryPoolProperty, Pipeline,
    Queue, QueueInfo, RasterizerStateInfo, Texture, TextureInfo, VertexAttributeStateInfo,
    VertexBufferStateInfo, VertexStateInfo,
};

fn create_test_shader() -> (Vec<u8>, Vec<u8>) {
    let vertex_shader_source = &include_str!("./resources/shaders/hello_graphics_pipeline.vs");
    let pixel_shader_source = &include_str!("./resources/shaders/hello_graphics_pipeline.fs");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();

    let vertex_shader_resource = compiler
        .compile_into_spirv(
            vertex_shader_source,
            shaderc::ShaderKind::Vertex,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap();

    let pixel_shader_resource = compiler
        .compile_into_spirv(
            pixel_shader_source,
            shaderc::ShaderKind::Fragment,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap();

    (
        vertex_shader_resource.as_binary_u8().to_vec(),
        pixel_shader_resource.as_binary_u8().to_vec(),
    )
}

#[test]
fn empty_command_test() {
    let device = Device::new(&DeviceInfo::new());
    let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut queue = Queue::new(&device, &QueueInfo::new());

    command_buffer.begin();
    command_buffer.end();
    queue.execute(&command_buffer);
    queue.sync();
}

#[test]
fn set_render_target_test() {
    let device = Device::new(&DeviceInfo::new());

    let texture_info = TextureInfo::new()
        .set_width(640)
        .set_height(480)
        .set_depth(1)
        .set_gpu_access_flags(GpuAccess::COLOR_BUFFER)
        .set_image_format(ImageFormat::R8G8B8A8Unorm);
    let texture_size = Texture::calculate_required_size(&device, &texture_info);
    let memory_pool = MemoryPool::new(
        &device,
        &MemoryPoolInfo::new()
            .set_memory_pool_property(
                MemoryPoolProperty::CPU_INVISIBLE | MemoryPoolProperty::GPU_CACHED,
            )
            .set_size(texture_size),
    );
    let texture = Texture::new(&device, &texture_info, &memory_pool, 0, 128);

    let color_target_view_info = ColorTargetViewInfo::new(&texture);
    let color_target_view = ColorTargetView::new(&device, &color_target_view_info);

    let (vertex_shader_binary, pixel_shader_binary) = create_test_shader();
    let shader_info = sj::gfx::ShaderInfo::new()
        .set_vertex_shader_binary(&vertex_shader_binary)
        .set_pixel_shader_binary(&pixel_shader_binary);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let rasterizer_state_info = RasterizerStateInfo::new();
    let depth_stencil_state_info = DepthStencilStateInfo::new();
    let blend_target_state_info_array = [BlendTargetStateInfo::new()];
    let blend_state_info =
        BlendStateInfo::new().set_target_state_info(&blend_target_state_info_array);
    let vertex_attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(AttributeFormat::Float32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<f32>() as i64)];
    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let graphics_pipeline_info = GraphicsPipelineInfo::new()
        .set_shader(&shader)
        .set_rasterizer_state(&rasterizer_state_info)
        .set_depth_stencil_state(&depth_stencil_state_info)
        .set_blend_stae(&blend_state_info)
        .set_vertex_state_info(&vertex_state_info);
    let graphics_pipeline = Pipeline::new_as_graphics(&device, &graphics_pipeline_info);

    let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
    let render_targets = [&color_target_view];
    command_buffer.begin();
    command_buffer.set_pipeline(&graphics_pipeline);
    //command_buffer.set_render_targets(&render_targets, None);
    command_buffer.end();
}

#[test]
fn set_graphics_pipeline_test() {
    let device = Device::new(&DeviceInfo::new());

    let (vertex_shader_binary, pixel_shader_binary) = create_test_shader();
    let shader_info = sj::gfx::ShaderInfo::new()
        .set_vertex_shader_binary(&vertex_shader_binary)
        .set_pixel_shader_binary(&pixel_shader_binary);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let rasterizer_state_info = RasterizerStateInfo::new();
    let depth_stencil_state_info = DepthStencilStateInfo::new();
    let blend_target_state_info_array = [BlendTargetStateInfo::new()];
    let blend_state_info =
        BlendStateInfo::new().set_target_state_info(&blend_target_state_info_array);
    let vertex_attribute_state_info_array = [VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(AttributeFormat::Float32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array =
        [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<f32>() as i64)];
    let vertex_state_info = VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let graphics_pipeline_info = GraphicsPipelineInfo::new()
        .set_shader(&shader)
        .set_rasterizer_state(&rasterizer_state_info)
        .set_depth_stencil_state(&depth_stencil_state_info)
        .set_blend_stae(&blend_state_info)
        .set_vertex_state_info(&vertex_state_info);
    let graphics_pipeline = Pipeline::new_as_graphics(&device, &graphics_pipeline_info);

    let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
    command_buffer.begin();
    command_buffer.set_pipeline(&graphics_pipeline);
    command_buffer.end();
}
