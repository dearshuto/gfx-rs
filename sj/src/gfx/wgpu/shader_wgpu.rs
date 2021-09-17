use super::super::shader_api::IShaderImpl;
use super::super::shader_api::ShaderInfo;
use super::super::Device;
use std::sync::Arc;
use std::borrow::Cow;
use std::marker::PhantomData;

pub struct ShaderImpl<'a> {
	_vertex_shader_source: Option<String>,
	_pixel_shader_source: Option<String>,
	_compute_shader_source: Option<String>,
    _vertex_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _pixel_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _compute_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _marker: PhantomData<&'a i32>,
}

impl<'a> ShaderImpl<'a> {
	pub fn clone_vertex_shader_module(&self) -> Arc<wgpu::ShaderModule> {
		self._vertex_shader_module.as_ref().unwrap().clone()
	}

	pub fn clone_pixel_shader_module(&self) -> Arc<wgpu::ShaderModule> {
		self._pixel_shader_module.as_ref().unwrap().clone()
	}

	pub fn clone_compute_shader_module(&self) -> Arc<wgpu::ShaderModule> {
		self._compute_shader_module.as_ref().unwrap().clone()
	}

    fn create_shader_module(device: &wgpu::Device, sprv_binary: &[u8]) -> (wgpu::ShaderModule, String) {
        let options = naga::front::spv::Options {
            //adjust_coordinate_space: !self.features.contains(hal::Features::NDC_Y_UP),
            strict_capabilities: true,
            flow_graph_dump_prefix: None,
            adjust_coordinate_space: true,
        };
        let module = naga::front::spv::parse_u8_slice(sprv_binary, &options).unwrap();
        let module_info = naga::valid::Validator::new(
            naga::valid::ValidationFlags::empty(),
            naga::valid::Capabilities::empty(), //TODO: PUSH_CONSTANT
        )
        .validate(&module)
        .unwrap();

        let shader_source_string = naga::back::wgsl::write_string(&module, &module_info).unwrap();
		let shader_source_str: &str = &shader_source_string;
        let shader_source = std::borrow::Cow::Borrowed(shader_source_str);
		println!("{}", shader_source);
		
        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source),
        });
        (shader_module, shader_source_string)
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
		let shader = Arc::new(device.to_data().get_device().create_shader_module(&wgpu::ShaderModuleDescriptor {
			label: None,
			source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
		}));
		
        let (vertex_shader_module, vertex_shader_source) = match info.get_vertex_shader_binary() {
            Some(vertex_shader_binary) => {
				let (module, source) = ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &vertex_shader_binary,
				);
				(Some(Arc::new(module)), Some(source))
			},
            None => (None, None),
        };
        let (pixel_shader_module, pixel_shader_source) = match info.get_pixel_shader_binary() {
            Some(pixel_shader_binary) => {
				let (module, source) = ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &pixel_shader_binary,
				);
				(Some(Arc::new(module)), Some(source))
			},
            None => (None, None),
        };
        let _compute_shader_module = match info.get_compute_shader_binary() {
            Some(compute_shader_binary) => Some(Arc::new(ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &compute_shader_binary,
            ))),
            None => None,
        };

        Self {
			_vertex_shader_source: vertex_shader_source,
			_pixel_shader_source: pixel_shader_source,
			_compute_shader_source: None,
            // _vertex_shader_module: vertex_shader_module,
            // _pixel_shader_module: pixel_shader_module,
            _vertex_shader_module: Some(shader.clone()),
            _pixel_shader_module: Some(shader.clone()),
			
            _compute_shader_module: None,//compute_shader_module,
            _marker: PhantomData,
        }
    }
}
