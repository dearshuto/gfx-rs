use sjgfx::{
    api::IApi, TBufferBuilder, TColorTargetViewBuilder, TCommandBufferBuilder, TDeviceBuilder,
    TQueueBuilder, TShaderBuilder, TTextureBuilder, TVertexStateBuilder,
};
use sjgfx_interface::{
    AttributeFormat, IBuffer, ICommandBuffer, IQueue, PrimitiveTopology, VertexAttributeStateInfo,
    VertexBufferStateInfo,
};

#[test]
fn make_command_ash() {
    make_commnd_impl::<sjgfx::api::Ash>();
}

// #[test]
// fn make_command_wgpu() {
//     make_commnd_impl::<sjgfx::api::Wgpu>();
// }

#[test]
fn make_command_vulkano() {
    make_commnd_impl::<sjgfx::api::Vulkano>();
}

fn make_commnd_impl<TApi: IApi>() {
    let device = TDeviceBuilder::<TApi>::new()
        .enable_debug_assertion()
        .build();
    let mut queue = TQueueBuilder::<TApi>::new().build(&device);
    let mut command_buffer = TCommandBufferBuilder::<TApi>::new().build(&device);

    struct Vertex {
        position_x: f32,
        position_y: f32,
        color_r: f32,
        color_g: f32,
        color_b: f32,

        _padding_0: f32,
        _padding_1: f32,
        _padding_2: f32,
    }
    let vertex_shader_source = "
        		#version 450

            layout(location = 0) out vec3 v_Color;

            layout(location = 0) in vec2 i_Position;
            layout(location = 1) in vec3 i_Color;

        		void main() {
                v_Color = i_Color;
                gl_Position = vec4(i_Position, 0.0, 1.0);
        		}";
    let pixel_shader_source = "
        		#version 450

            layout(location = 0) out vec4 o_Color;
            layout(location = 0) in vec3 v_Color;

        		void main() {
                o_Color = vec4(v_Color, 1.0);
        		}";
    let mut compiler = shaderc::Compiler::new().unwrap();
    let vertex_shader_binary = compiler
        .compile_into_spirv(
            &vertex_shader_source,
            shaderc::ShaderKind::Vertex,
            "test.vs",
            "main",
            None,
        )
        .unwrap();
    let pixel_shader_binary = compiler
        .compile_into_spirv(
            &pixel_shader_source,
            shaderc::ShaderKind::Fragment,
            "test.fs",
            "main",
            None,
        )
        .unwrap();
    let shader = TShaderBuilder::<TApi>::new()
        .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
        .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8())
        .build(&device);

    let texture = TTextureBuilder::<TApi>::new()
        .enable_color_buffer()
        .build(&device);
    let color_target_view = TColorTargetViewBuilder::<TApi>::new().build(&device, &texture);

    let vertex_buffer = TBufferBuilder::<TApi>::new()
        .enable_vertex_buffer()
        .with_size(std::mem::size_of::<Vertex>() * 3)
        .build(&device);
    vertex_buffer.map_as_slice_mut(|x: &mut [Vertex]| {
        x[0].position_x = -0.5;
        x[0].position_y = -0.5;
        x[0].color_r = 1.0;
        x[0].color_g = 0.0;
        x[0].color_b = 0.0;

        x[1].position_x = 0.5;
        x[1].position_y = -0.5;
        x[1].color_r = 0.0;
        x[1].color_g = 1.0;
        x[1].color_b = 0.0;

        x[2].position_x = 0.0;
        x[2].position_y = 0.5;
        x[2].color_r = 0.0;
        x[2].color_g = 0.0;
        x[2].color_b = 1.0;
    });

    // 頂点ステート
    let vertex_state = TVertexStateBuilder::<TApi>::new()
        .set_vertex_attribute_states(
            [
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_format(AttributeFormat::Float32_32)
                    .set_offset(0)
                    .set_slot(0),
                VertexAttributeStateInfo::new()
                    .set_buffer_index(0)
                    .set_format(AttributeFormat::Float32_32_32)
                    .set_offset((std::mem::size_of::<f32>() * 2) as i64)
                    .set_slot(1),
            ]
            .into_iter(),
        )
        .set_vertex_buffer_states(
            [VertexBufferStateInfo::new().set_stride(std::mem::size_of::<Vertex>() as i64)]
                .into_iter(),
        )
        .build(&device);

    command_buffer.begin();
    command_buffer.set_render_targets(&[&color_target_view], None);
    command_buffer.set_shader(&shader);
    command_buffer.set_vertex_buffer(0, &vertex_buffer);
    command_buffer.set_vertex_state(&vertex_state);
    command_buffer.draw(PrimitiveTopology::TriangleList, 3, 0);
    command_buffer.end();

    queue.flush();
    queue.sync();
}
