use std::io::Write;

fn main()
{
	let source = &include_str!("resources/shaders/test.glsl");
	//let source = "#version 310 es\n void main() {}";
	
	let mut compiler = shaderc::Compiler::new().unwrap();
	let options = shaderc::CompileOptions::new().unwrap();
	//options.set_optimization_level(shaderc::OptimizationLevel::Size);
	let binary_result = compiler.compile_into_spirv(
		source, shaderc::ShaderKind::Compute,
		"shader.glsl", "main", Some(&options)).unwrap();

	let mut file = std::fs::File::create("resources/shaders/test.spv").unwrap();
	let _result = file.write_all(binary_result.as_binary_u8());	
}
