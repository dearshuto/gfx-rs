use std::{io::Write, path::Path};

fn main() {
    let mut compiler = Compiler::new();

    // アルマジロ
    compiler.build_graphics_shader(
        &"resources/shaders/armadillo.vs",
        &"resources/shaders/armadillo.fs",
    );

    // マンデルブロ集合
    compiler.build_graphics_shader(
        &"resources/shaders/mandelbrot.vs",
        &"resources/shaders/mandelbrot.fs",
    );

    // 波形
    compiler.build_graphics_shader(&"resources/shaders/wave.vs", &"resources/shaders/wave.fs");
}

struct Compiler {
    compiler: shaderc::Compiler,
}
impl Compiler {
    pub fn new() -> Self {
        Self {
            compiler: shaderc::Compiler::new().unwrap(),
        }
    }

    pub fn build_graphics_shader<TPath: AsRef<Path>>(
        &mut self,
        vertex_shader_path: &TPath,
        pixel_shader_path: &TPath,
    ) {
        let vertex_shader_source = std::fs::read_to_string(vertex_shader_path).unwrap();
        let pixel_shader_source = std::fs::read_to_string(pixel_shader_path).unwrap();
        let vertex_shader_binary = self
            .compiler
            .compile_into_spirv(
                &vertex_shader_source,
                shaderc::ShaderKind::Vertex,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();
        let pixel_shader_binary = self
            .compiler
            .compile_into_spirv(
                &pixel_shader_source,
                shaderc::ShaderKind::Fragment,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();

        let output_directory_path = std::path::Path::new("outputs/resources/shaders");
        let _ = std::fs::create_dir_all(&output_directory_path).expect("");

        let vertex_shader_file_path = vertex_shader_path.as_ref().with_extension("vs.spv");
        let pixel_shader_file_path = vertex_shader_path.as_ref().with_extension("fs.spv");

        // 頂点シェーダ
        std::fs::File::create(
            output_directory_path.join(vertex_shader_file_path.file_name().unwrap()),
        )
        .unwrap()
        .write_all(vertex_shader_binary.as_binary_u8())
        .unwrap();

        // ピクセルシェーダ
        std::fs::File::create(
            output_directory_path.join(pixel_shader_file_path.file_name().unwrap()),
        )
        .unwrap()
        .write_all(pixel_shader_binary.as_binary_u8())
        .unwrap();
    }
}
