use std::io::Write;

fn main() {
    let vertex_shader_source = &include_str!("resources/shaders/hello_3d.vs");
    let pixel_shader_source = &include_str!("resources/shaders/hello_3d.fs");

    let mut compiler = shaderc::Compiler::new().unwrap();
    let options = shaderc::CompileOptions::new().unwrap();

    let binary_result = compiler
        .compile_into_spirv(
            vertex_shader_source,
            shaderc::ShaderKind::Vertex,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap();

    let mut file = std::fs::File::create("resources/shaders/hello_3d_vs.spv").unwrap();
    let _result = file.write_all(binary_result.as_binary_u8());

    let binary_result = compiler
        .compile_into_spirv(
            pixel_shader_source,
            shaderc::ShaderKind::Fragment,
            "shader.glsl",
            "main",
            Some(&options),
        )
        .unwrap();
    let mut file = std::fs::File::create("resources/shaders/hello_3d_fs.spv").unwrap();
    let _result = file.write_all(binary_result.as_binary_u8());
}
