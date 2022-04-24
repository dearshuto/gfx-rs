fn main() {
    let mut compiler = sjgfx_util::ShaderCompiler::new();

    // アルマジロ
    compiler.build_graphics_shader(
        &"resources/shaders/armadillo.vs",
        &"resources/shaders/armadillo.fs",
    );

    // デファード
    compiler.build_graphics_shader(
        &"resources/shaders/geometry_output.vs",
        &"resources/shaders/geometry_output.fs",
    );
    compiler.build_graphics_shader(
        &"resources/shaders/deffered_shading.vs",
        &"resources/shaders/deffered_shading.fs",
    );

    // マンデルブロ集合
    compiler.build_graphics_shader(
        &"resources/shaders/mandelbrot.vs",
        &"resources/shaders/mandelbrot.fs",
    );

    // 波形
    compiler.build_graphics_shader(&"resources/shaders/wave.vs", &"resources/shaders/wave.fs");
}
