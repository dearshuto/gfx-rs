use image::Pixel;
use image::{self, GenericImage};
use sj::gfx::{
    Buffer, BufferInfo, BufferTextureCopyRegion, ColorTargetView, ColorTargetViewInfo,
    CommandBuffer, CommandBufferInfo, Device, DeviceInfo, GpuAccess, ImageFormat,
    MemoryPool, MemoryPoolInfo, MemoryPoolProperty, PipelineStageBit, Queue, QueueInfo, Texture,
    TextureInfo, TextureState, TextureSubresourceRange,
};

#[test]
fn ash() {
    let device = Device::new(&DeviceInfo::new());

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

    let shader_info = sj::gfx::ShaderInfo::new()
        .set_vertex_shader_binary(&vertex_shader_resource.as_binary_u8())
        .set_pixel_shader_binary(&pixel_shader_resource.as_binary_u8());
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    let viewport_state_info = [sj::gfx::ViewportStateInfo::new()
        .set_origin_x(0.0)
        .set_origin_y(0.0)
        .set_width(640.0)
        .set_height(480.0)];
    let scissor_state_info = [sj::gfx::ScissorStateInfo::new()
        .set_origin_x(0)
        .set_origin_y(0)
        .set_width(640)
        .set_height(480)];
    let viewport_scissor_state_info = sj::gfx::ViewportScissorStateInfo::new()
        .set_viewport_state_info_array(&viewport_state_info)
        .set_scissor_state_info_array(&scissor_state_info);
    let viewport_scissor_state =
        sj::gfx::ViewportScissorState::new(&device, &viewport_scissor_state_info);

    let vertex_attribute_state_info_array = [sj::gfx::VertexAttributeStateInfo::new()
        .set_slot(0)
        .set_format(sj::gfx::AttributeFormat::Float32_32)
        .set_offset(0)
        .set_buffer_index(0)];
    let vertex_buffer_state_info_array = [
        sj::gfx::VertexBufferStateInfo::new().set_stride((std::mem::size_of::<f32>() * 2) as i64)
    ];
    let vertex_state_info = sj::gfx::VertexStateInfo::new()
        .set_attribute_state_info_array(&vertex_attribute_state_info_array)
        .set_buffer_state_info_array(&vertex_buffer_state_info_array);
    let rasterizer_state_info = sj::gfx::RasterizerStateInfo::new();
    let depth_stencil_state_info = sj::gfx::DepthStencilStateInfo::new();
    let blend_target_state_info_array = [sj::gfx::BlendTargetStateInfo::new()];
    let blend_state_info =
        sj::gfx::BlendStateInfo::new().set_target_state_info(&blend_target_state_info_array);

    let graphics_pipeline_create_info = sj::gfx::GraphicsPipelineInfo::new()
        .set_vertex_state_info(&vertex_state_info)
        .set_rasterizer_state(&rasterizer_state_info)
        .set_depth_stencil_state(&depth_stencil_state_info)
        .set_blend_stae(&blend_state_info)
        .set_shader(&shader);
    let pipeline = sj::gfx::Pipeline::new_as_graphics(&device, &graphics_pipeline_create_info);

    let texture_memory_pool = MemoryPool::new(
        &device,
        &MemoryPoolInfo::new()
            .set_size(16 * 640 * 480)
            .set_memory_pool_property(
                MemoryPoolProperty::CPU_INVISIBLE | MemoryPoolProperty::GPU_UNCACHED,
            ),
    );
    let texture = Texture::new(
        &device,
        &TextureInfo::new()
            .set_width(640)
            .set_height(480)
            .set_depth(1)
            .set_gpu_access_flags(GpuAccess::COLOR_BUFFER | GpuAccess::READ)
            .set_image_format(sj::gfx::ImageFormat::R8G8B8A8Unorm),
        &texture_memory_pool,
        4 * 640 * 480,
        0,
    );

    let color_target_view_info =
        ColorTargetViewInfo::new(&texture).set_image_format(ImageFormat::R8G8B8A8Unorm);
    let mut color_target_view = ColorTargetView::new(&device, &color_target_view_info);

    let vertex_buffer_pool = MemoryPool::new(
        &device,
        &MemoryPoolInfo::new()
            .set_size(256)
            .set_memory_pool_property(
                MemoryPoolProperty::CPU_CACHED | MemoryPoolProperty::GPU_CACHED,
            ),
    );
    let vertex_buffer = sj::gfx::Buffer::new(
        &device,
        &BufferInfo::new()
            .set_gpu_access_flags(GpuAccess::VERTEX_BUFFER)
            .set_size(128),
        &vertex_buffer_pool,
        0,
        128,
    );
    vertex_buffer.map();
    vertex_buffer.write::<[f32; 6]>(|x| {
        x[0] = 0.0;
        x[1] = 1.0;
        x[2] = -1.0;
        x[3] = -1.0;
        x[4] = 1.0;
        x[5] = -1.0;
    });
    vertex_buffer.flush_mapped_range(0, 0x40);
    vertex_buffer.unmap();

    let out_buffet_pool = MemoryPool::new(
        &device,
        &MemoryPoolInfo::new()
            .set_memory_pool_property(
                MemoryPoolProperty::CPU_CACHED | MemoryPoolProperty::GPU_CACHED,
            )
            .set_size(4 * 640 * 480),
    );
    let mut out_buffer = Buffer::new(
        &device,
        &BufferInfo::new()
            .set_size(4 * 640 * 480)
            .set_gpu_access_flags(GpuAccess::WRITE),
        &out_buffet_pool,
        0,
        4 * 640 * 480,
    );

    let mut command_buffer = CommandBuffer::new(&device, &CommandBufferInfo::new());
    let mut queue = Queue::new(&device, &QueueInfo::new());

    command_buffer.begin();
    command_buffer.clear_color(&mut color_target_view, 0.25, 0.25, 0.4, 1.0, None);
    command_buffer.set_pipeline(&pipeline);
    command_buffer.set_render_targets(&[&color_target_view], None);
    command_buffer.set_viewport_scissor_state(&viewport_scissor_state);
    //command_buffer.set_vertex_buffer(0, &GpuAddress::new(&vertex_buffer));
    command_buffer.draw(sj::gfx::PrimitiveTopology::PointList, 6, 0);
    command_buffer.flush_memory(GpuAccess::COLOR_BUFFER | GpuAccess::TEXTURE);

    let texture_subresource_range = TextureSubresourceRange::new();
    command_buffer.set_texture_state_transition(
        &texture,
        &texture_subresource_range,
        TextureState::COLOR_TARGET,
        PipelineStageBit::RENDER_TARGET,
        TextureState::COPY_SOURCE,
        PipelineStageBit::all(),
    );
    command_buffer.flush_memory(GpuAccess::READ);

    let copy_region = BufferTextureCopyRegion::new()
        .set_image_width(640)
        .set_image_height(480)
        .edit_texture_copy_region(|region| region.set_width(640).set_height(480));
    command_buffer.copy_image_to_buffer(&mut out_buffer, &texture, &copy_region);
    command_buffer.flush_memory(sj::gfx::GpuAccess::WRITE);
    command_buffer.end();
    queue.execute(&command_buffer);
    queue.flush();
    queue.sync();

    let mut target_directory = std::env::current_exe().unwrap();
    target_directory.pop();
    target_directory.push("test.png");

    const WIDTH: usize = 640;
    const HEIGHT: usize = 480;
    let mut pixel_data = Vec::new();
    out_buffer.map();
    out_buffer.read_with_user_data::<&[u8; 4 * WIDTH * HEIGHT], Vec<u8>>(
        |x, pixel_data| {
            *pixel_data.unwrap() = x.to_vec();
        },
        Some(&mut pixel_data),
    );
	
    out_buffer.invalidate_mapped_range(0, 4 * 640 * 480);
    out_buffer.unmap();
    let mut image_buffer = image::DynamicImage::new_rgb8(640, 480);

    //let expected_data = include_bytes!("./resources/expected/ash/simple_triangle.png").to_vec();
    //let expected_image = image::RgbImage::from_raw(640, 480, expected_data.to_vec()).unwrap();

    for x in 0..640 {
        for y in 0..480 {
            let index = 4 * (x + y * 640);
            let red = pixel_data[index + 0];
            let green = pixel_data[index + 1];
            let blue = pixel_data[index + 2];
            //let alpha = 1.0 - pixel_data[index + 3];
            image_buffer.put_pixel(
                x as u32,
                y as u32,
                image::Rgba::from_channels(red as u8, green as u8, blue as u8, 0),
            );

            // let expected = expected_image.get_pixel(x as u32, y as u32);
            // let expected_red = expected[0];
            // let expected_blue = expected[1];
            // let expected_green = expected[2];
            // println!("{}", expected_red);
            // assert_eq!(red, expected_red);
            // assert_eq!(green, expected_green);
            // assert_eq!(blue, expected_blue);
        }
    }
    let result = image_buffer.save(target_directory);
    assert!(result.is_ok());
}
