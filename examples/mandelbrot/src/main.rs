use winit::platform::run_return::EventLoopExtRunReturn;
use winit::{event::Event, event_loop::ControlFlow};

fn main() {
    let mut display = sj::vi::create_display();
    let mut layer = sj::vi::create_layer(&mut display);

    let device_info = sj::gfx::DeviceInfo::new().set_layer(Some(&layer));
    let device = sj::gfx::Device::new(&device_info);

    let mut swap_shain_info = sj::gfx::SwapChainInfo::new();
    let mut swap_chain = sj::gfx::SwapChain::new(&device, &mut swap_shain_info);

    let vertex_shader_source = include_bytes!("../resources/shaders/mandelbrot_vs.spv");
    let pixel_shader_source = include_bytes!("../resources/shaders/mandelbrot_fs.spv");
    let shader_info = sj::gfx::ShaderInfo::new()
        .set_vertex_shader_binary(vertex_shader_source)
        .set_pixel_shader_binary(pixel_shader_source);
    let shader = sj::gfx::Shader::new(&device, &shader_info);

    // let viewport_state_info = [sj::gfx::ViewportStateInfo::new()
    //     .set_origin_x(0.0)
    //     .set_origin_y(0.0)
    //     .set_width(640.0)
    //     .set_height(480.0)];
    // let scissor_state_info = [sj::gfx::ScissorStateInfo::new()
    //     .set_origin_x(0)
    //     .set_origin_y(0)
    //     .set_width(640)
    //     .set_height(480)];
    // let viewport_scissor_state_info = sj::gfx::ViewportScissorStateInfo::new()
    //     .set_viewport_state_info_array(&viewport_state_info)
    //     .set_scissor_state_info_array(&scissor_state_info);
    // let _viewport_scissor_state =
    //     sj::gfx::ViewportScissorState::new(&device, &viewport_scissor_state_info);

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

    let buffer_info = sj::gfx::BufferInfo::new()
        .set_gpu_access_flags(sj::gfx::GpuAccess::VERTEX_BUFFER)
        .set_size(128);

    // 画面いっぱいに四角形を描く
    let vertex_buffer = sj::gfx::Buffer::new(&device, &buffer_info, None, 0, 0);
    {
        let mut mapped_data = vertex_buffer.map_as_slice_mut::<f32>(12);
        mapped_data[0] = -1.0;
        mapped_data[1] = 1.0;
        mapped_data[2] = -1.0;
        mapped_data[3] = -1.0;
        mapped_data[4] = 1.0;
        mapped_data[5] = -1.0;

        mapped_data[6] = -1.0;
        mapped_data[7] = 1.0;
        mapped_data[8] = 1.0;
        mapped_data[9] = -1.0;
        mapped_data[10] = 1.0;
        mapped_data[11] = 1.0;
    }
    vertex_buffer.flush_mapped_range(0, 0x40);
    vertex_buffer.unmap();

    let mut semaphore = sj::gfx::Semaphore::new(&device, &sj::gfx::SemaphoreInfo::new());

    let queue_info = sj::gfx::QueueInfo::new();
    let mut queue = sj::gfx::Queue::new(&device, &queue_info);

    loop {
        layer
            .get_event_loop_mut()
            .run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                match event {
                    Event::RedrawRequested(_) => {
                        queue.sync_semaphore(&mut semaphore);

                        let next_scan_buffer_view = swap_chain.acquire_next_scan_buffer_view();
                        let mut command_buffer = sj::gfx::CommandBuffer::new(
                            &device,
                            &sj::gfx::CommandBufferInfo::new(),
                        );

                        command_buffer.begin();
                        command_buffer.set_pipeline(&pipeline);
                        command_buffer
                            .set_vertex_buffer(0, sj::gfx::GpuAddress::new(&vertex_buffer));

                        let vertex_count = 6;
                        let vertex_offset = 0;
                        command_buffer.draw(
                            sj::gfx::PrimitiveTopology::TriangleList,
                            vertex_count,
                            vertex_offset,
                        );

                        command_buffer.flush_memory(sj::gfx::GpuAccess::COLOR_BUFFER);

                        let mut command_buffer =
                            command_buffer.set_scan_buffer_view(next_scan_buffer_view);
                        command_buffer.end();

                        queue.execute_scan_buffer_command(command_buffer);

                        queue.flush();
                        queue.present(&mut swap_chain, 1);
                        queue.sync();
                        std::thread::sleep(std::time::Duration::from_millis(32));
                    }
                    _ => {}
                }
            });
    }
}
