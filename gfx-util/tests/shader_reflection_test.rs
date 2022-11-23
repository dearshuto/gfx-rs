use sjgfx_interface::AttributeFormat;
use sjgfx_util::ShaderCompiler;
use sjgfx_util::ShaderReflection;

#[test]
fn vertex_attributes() {
    let shader_source = "
            #version 450
            
            layout(location = 0) out vec2 v_Uv;
            
            layout(location = 0) in vec3 i_Position;
            layout(location = 1) in vec2 i_Uv;
            
            void main()
            {
                v_Uv = i_Uv;
                gl_Position = vec4(i_Position, 1.0);
            }";
    let shader_binary =
        ShaderCompiler::new().create_binary(&shader_source, sjgfx_util::ShaderStage::Vertex);
    let shader_reflection = ShaderReflection::new_from_biinary(&shader_binary);
    let attributes = shader_reflection.entry_point.attribures();
    assert_eq!(attributes.len(), 2);

    for attribute in attributes.iter() {
        match attribute.location() {
            0 => assert_eq!(attribute.format(), AttributeFormat::Float32_32_32),
            1 => assert_eq!(attribute.format(), AttributeFormat::Float32_32),
            _ => assert!(false),
        }
    }
}

#[test]
fn constant_buffer_reflection() {
    let shader_source = "
            #version 450
            
            layout(location = 0) out vec2 v_Uv;
            
            layout(location = 0) in vec2 i_Position;
            
            layout(binding = 0) uniform Block0
            {
                vec4 u_Data0;
            };

            layout(binding = 23) uniform Block1
            {
                vec4 u_Data1;
                vec4 u_Data11;
            };

            void main()
            {
                gl_Position = u_Data0 + u_Data1 + u_Data11 + vec4(i_Position, 0.0, 1.0);
            }";
    let shader_binary =
        ShaderCompiler::new().create_binary(&shader_source, sjgfx_util::ShaderStage::Vertex);
    let shader_reflection = ShaderReflection::new_from_biinary(&shader_binary);

    // 定数バッファをすべて抽出できていることをテスト
    assert_eq!(shader_reflection.uniform_buffers().len(), 2);

    assert!(shader_reflection
        .uniform_buffers()
        .iter()
        .find(|x| x.binding == 0)
        .is_some());
    assert!(shader_reflection
        .uniform_buffers()
        .iter()
        .find(|x| x.binding == 23)
        .is_some());

    // それぞれの定数バッファのサイズを取得できていることをテスト
    for uniform_buffer in shader_reflection.uniform_buffers() {
        match uniform_buffer.binding {
            0 => assert_eq!(uniform_buffer.size, 16),
            23 => assert_eq!(uniform_buffer.size, 32),
            _ => assert!(false),
        }
    }
}

#[test]
fn constant_buffer_reflection_array() {
    let shader_source = "
            #version 450
            
            layout(location = 0) out vec2 v_Uv;
            
            layout(binding = 1) uniform Block0
            {
                vec4 u_Data[4];
            };

            void main()
            {
                gl_Position = u_Data[0] + u_Data[1] + u_Data[2] + u_Data[3];
            }";
    let shader_binary =
        ShaderCompiler::new().create_binary(&shader_source, sjgfx_util::ShaderStage::Vertex);
    let shader_reflection = ShaderReflection::new_from_biinary(&shader_binary);

    let uniform_buffer = shader_reflection.uniform_buffers().first().unwrap();
    assert_eq!(uniform_buffer.size, 64);
}
