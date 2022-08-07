#[cfg(not(target_arch = "wasm32"))]
use std::io::Write;

use std::path::Path;

pub enum ShaderStage {
    Vertex,
    Pixel,
    Compute,
}

pub struct ShaderCompiler {
    #[cfg(not(target_arch = "wasm32"))]
    compiler: shaderc::Compiler,
}

impl ShaderCompiler {
    pub fn new() -> Self {
        Self {
            #[cfg(not(target_arch = "wasm32"))]
            compiler: shaderc::Compiler::new().unwrap(),
        }
    }

    pub fn create_binary(
        &mut self,
        #[allow(unused_variables)] source: &str,
        #[allow(unused_variables)] shader_stage: ShaderStage,
    ) -> Vec<u8> {
        #[cfg(target_arch = "wasm32")]
        {
            // wasm ビルドでもエラーにならないように空実装を提供しておく
            Vec::new()
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            let vertex_shader_binary = self
                .compiler
                .compile_into_spirv(
                    source,
                    Self::convert(shader_stage),
                    "test.glsl",
                    "main",
                    None,
                )
                .unwrap();

            vertex_shader_binary.as_binary_u8().to_vec()
        }
    }

    pub fn build_graphics_shader<TPath: AsRef<Path>>(
        &mut self,
        #[allow(unused_variables)] vertex_shader_path: &TPath,
        #[allow(unused_variables)] pixel_shader_path: &TPath,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let vertex_shader_source = std::fs::read_to_string(vertex_shader_path).unwrap();
            let pixel_shader_source = std::fs::read_to_string(pixel_shader_path).unwrap();
            let vertex_shader_binary = self
                .compiler
                .compile_into_spirv(
                    &vertex_shader_source,
                    shaderc::ShaderKind::Vertex,
                    "VertexShader",
                    "main",
                    None,
                )
                .unwrap();
            let pixel_shader_binary = self
                .compiler
                .compile_into_spirv(
                    &pixel_shader_source,
                    shaderc::ShaderKind::Fragment,
                    "PixelShader",
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

    #[cfg(not(target_arch = "wasm32"))]
    fn convert(shader_stage: ShaderStage) -> shaderc::ShaderKind {
        match shader_stage {
            ShaderStage::Vertex => shaderc::ShaderKind::Vertex,
            ShaderStage::Pixel => shaderc::ShaderKind::Fragment,
            ShaderStage::Compute => shaderc::ShaderKind::Compute,
        }
    }
}
