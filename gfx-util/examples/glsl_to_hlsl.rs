use sjgfx_util::{Glsl, Hlsl, SpirV};
use sjgfx_util::{ShaderConverter, Wgsl};

fn main() {
    let source = "#version 450

    layout(local_size_x = 64, local_size_y = 1, local_size_z = 1) in;

    layout(set = 0, binding = 0) buffer Data {
        uint u_Data[];
    };

    void main()
    {
        uint index = gl_GlobalInvocationID.x;
        u_Data[index] = index;
    }
";
    let wglsl = ShaderConverter::<Wgsl, Glsl>::convert_glsl_to_wgsl(&source);
    println!("{}", wglsl);

    let _data = ShaderConverter::<SpirV, Wgsl>::convert_wgsl_to_spirv(&wglsl);
}
