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
