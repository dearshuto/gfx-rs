fn main()
{	
    let device_info = sj::gfx::DeviceInfo::new();
    let device = sj::gfx::Device::new(&device_info);

	let source = include_bytes!("../resources/shaders/test.spv");
	let shader_info = sj::gfx::ShaderInfo::new()
		.set_shader_binary(source);
	let _shader = sj::gfx::Shader::new(&device, &shader_info);
}
