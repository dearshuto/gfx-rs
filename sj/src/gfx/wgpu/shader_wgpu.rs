use super::super::shader_api::IShaderImpl;
use super::super::shader_api::ShaderInfo;
use super::super::Device;
use std::marker::PhantomData;

pub struct ShaderImpl<'a> {
    _vertex_shader_module: Option<wgpu::ShaderModule>,
    _pixel_shader_module: Option<wgpu::ShaderModule>,
    _compute_shader_module: Option<wgpu::ShaderModule>,
    _marker: PhantomData<&'a i32>,
}

impl<'a> ShaderImpl<'a> {
    pub fn get_impl(&'a self) -> &'a wgpu::ShaderModule {
        self._vertex_shader_module.as_ref().unwrap()
    }

    pub fn get_vertex_shader_module(&self) -> &wgpu::ShaderModule {
        self._vertex_shader_module.as_ref().unwrap()
    }

    pub fn get_pixel_shader_module(&self) -> &wgpu::ShaderModule {
        self._pixel_shader_module.as_ref().unwrap()
    }

    pub fn get_compute_shader_module(&self) -> &wgpu::ShaderModule {
        self._compute_shader_module.as_ref().unwrap()
    }

    fn create_shader_module(device: &wgpu::Device, sprv_binary: &[u8]) -> wgpu::ShaderModule {
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

        let piixel_shader_source = &*naga::back::wgsl::write_string(&module, &module_info).unwrap();
        let shader_source = std::borrow::Cow::Borrowed(piixel_shader_source);

        let shader_module = device.create_shader_module(&wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(shader_source),
        });
        shader_module
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        let vertex_shader_module = match info.get_vertex_shader_binary() {
            Some(vertex_shader_binary) => Some(ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &vertex_shader_binary,
            )),
            None => None,
        };
        let pixel_shader_module = match info.get_pixel_shader_binary() {
            Some(pixel_shader_binary) => Some(ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &pixel_shader_binary,
            )),
            None => None,
        };
        let compute_shader_module = match info.get_compute_shader_binary() {
            Some(compute_shader_binary) => Some(ShaderImpl::create_shader_module(
                device.to_data().get_device(),
                &compute_shader_binary,
            )),
            None => None,
        };

        Self {
            _vertex_shader_module: vertex_shader_module,
            _pixel_shader_module: pixel_shader_module,
            _compute_shader_module: compute_shader_module,
            _marker: PhantomData,
        }
    }
}
