use std::io::Write;

fn main()
{
	let source = &include_str!("resources/shaders/hello_compute.glsl");
	//let source = "#version 310 es\n void main() {}";
	
	let mut compiler = shaderc::Compiler::new().unwrap();
	let mut options = shaderc::CompileOptions::new().unwrap();
	//options.set_optimization_level(shaderc::OptimizationLevel::Size);
	options.set_generate_debug_info();
	options.set_source_language(shaderc::SourceLanguage::GLSL);

	let binary_result = compiler.compile_into_spirv(
		source, shaderc::ShaderKind::Compute,
		"shader.glsl", "main", Some(&options)).unwrap();

	let mut file = std::fs::File::create("resources/shaders/hello_compute.spv").unwrap();
	let _result = file.write_all(binary_result.as_binary_u8());	
}
