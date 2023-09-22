use std::{fs::File, io::Write};

use sjgfx_util::ShaderCompiler;

fn main() {
    let binary_data = ShaderCompiler::new().create_binary(
        include_str!("examples/hello_compute.glsl"),
        sjgfx_util::ShaderStage::Compute,
    );

    let path = format!("outputs/shader.spv");
    let mut file = File::create(path).unwrap();
    file.write_all(&binary_data).unwrap();
}
