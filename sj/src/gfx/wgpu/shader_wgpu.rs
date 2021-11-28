use crate::gfx::shader_api::{IShaderImpl, ShaderInfo};
use crate::gfx::Device;
use std::marker::PhantomData;
use std::sync::Arc;

pub struct ShaderImpl<'a> {
    _vertex_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _pixel_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _compute_shader_module: Option<Arc<wgpu::ShaderModule>>,
    _vertex_attributes: Vec<wgpu::VertexAttribute>,
    _bind_group_layout: wgpu::BindGroupLayout,
    _pipeline_layout: wgpu::PipelineLayout,
    _marker: PhantomData<&'a ()>,
}

impl<'a> ShaderImpl<'a> {
    pub fn get_vertex_shader_module(&self) -> &wgpu::ShaderModule {
        &self._vertex_shader_module.as_ref().unwrap()
    }

    pub fn get_pixel_shader_module(&self) -> &wgpu::ShaderModule {
        &self._pixel_shader_module.as_ref().unwrap()
    }

    pub fn clone_vertex_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._vertex_shader_module.as_ref().unwrap().clone()
    }

    pub fn clone_pixel_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._pixel_shader_module.as_ref().unwrap().clone()
    }

    pub fn clone_compute_shader_module(&self) -> Arc<wgpu::ShaderModule> {
        self._compute_shader_module.as_ref().unwrap().clone()
    }

    pub fn get_vertex_attributes(&self) -> &[wgpu::VertexAttribute] {
        &self._vertex_attributes
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self._bind_group_layout
    }

    pub fn get_pipeline_layout(&self) -> &wgpu::PipelineLayout {
        &self._pipeline_layout
    }

    fn create_shader_module(
        device: &wgpu::Device,
        sprv_binary_opt: &Option<&[u8]>,
    ) -> Option<Arc<wgpu::ShaderModule>> {
        match sprv_binary_opt {
            Some(sprv_binary) => Some(Arc::new(device.create_shader_module(
                &wgpu::ShaderModuleDescriptor {
                    label: None,
                    source: wgpu::util::make_spirv(sprv_binary),
                },
            ))),
            None => None,
        }
    }

    pub fn create_vertex_attributes(shader_source: &[u8]) -> Vec<wgpu::VertexAttribute> {
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_source).unwrap();
        module
            .enumerate_input_variables(None)
            .unwrap()
            .into_iter()
            .map(|x| wgpu::VertexAttribute {
                format: Self::convert_attribute_format(x.format),
                offset: 0,
                shader_location: x.location,
            })
            .collect::<Vec<wgpu::VertexAttribute>>()
            .to_vec()
    }

    pub fn create_bind_group_layout_entries(
        shader_source: &[u8],
    ) -> Vec<wgpu::BindGroupLayoutEntry> {
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_source).unwrap();
        let _entry_point_name = module.get_entry_point_name();
        let shader_stage = module.get_shader_stage();
        let _bindings = module.enumerate_descriptor_bindings(None).unwrap();
        let _sets = module.enumerate_descriptor_sets(None).unwrap();

        module
            .enumerate_descriptor_bindings(None)
            .unwrap()
            .into_iter()
            .map(|x| match x.resource_type {
                spirv_reflect::types::ReflectResourceType::Undefined => todo!(),
                spirv_reflect::types::ReflectResourceType::Sampler => todo!(),
                spirv_reflect::types::ReflectResourceType::CombinedImageSampler => todo!(),
                spirv_reflect::types::ReflectResourceType::ConstantBufferView => {
                    wgpu::BindGroupLayoutEntry {
                        binding: x.binding,
                        visibility: Self::convert_shader_stage(shader_stage),
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                }
                spirv_reflect::types::ReflectResourceType::ShaderResourceView => todo!(),
                spirv_reflect::types::ReflectResourceType::UnorderedAccessView => {
                    wgpu::BindGroupLayoutEntry {
                        binding: x.binding,
                        visibility: Self::convert_shader_stage(shader_stage),
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Storage { read_only: false },
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }
                }
            })
            .collect::<Vec<wgpu::BindGroupLayoutEntry>>()
            .to_vec()
    }

    fn convert_attribute_format(format: spirv_reflect::types::ReflectFormat) -> wgpu::VertexFormat {
        match format {
            spirv_reflect::types::ReflectFormat::Undefined => todo!(),
            spirv_reflect::types::ReflectFormat::R32_UINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32_SINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32_SFLOAT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32_UINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32_SINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32_SFLOAT => wgpu::VertexFormat::Float32x2,
            spirv_reflect::types::ReflectFormat::R32G32B32_UINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32B32_SINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32B32_SFLOAT => wgpu::VertexFormat::Float32x3,
            spirv_reflect::types::ReflectFormat::R32G32B32A32_UINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32B32A32_SINT => todo!(),
            spirv_reflect::types::ReflectFormat::R32G32B32A32_SFLOAT => todo!(),
        }
    }

    fn convert_shader_stage(
        stage: spirv_reflect::types::ReflectShaderStageFlags,
    ) -> wgpu::ShaderStages {
        if stage.contains(spirv_reflect::types::ReflectShaderStageFlags::COMPUTE) {
            wgpu::ShaderStages::COMPUTE
        } else {
            todo!()
        }
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        let device_impl = device.to_data().get_device();
        let vertex_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_vertex_shader_binary());
        let pixel_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_pixel_shader_binary());
        let compute_shader_module =
            ShaderImpl::create_shader_module(device_impl, info.get_compute_shader_binary());

        let vertex_attributes = if let Some(shader_source) = info.get_vertex_shader_binary() {
            Self::create_vertex_attributes(&shader_source)
        } else {
            Vec::new()
        };
        let entries = if info.get_compute_shader_binary().is_some() {
            Self::create_bind_group_layout_entries(info.get_compute_shader_binary().unwrap())
        } else {
            let mut vertex_entries =
                Self::create_bind_group_layout_entries(info.get_vertex_shader_binary().unwrap());
            let mut pixel_entries =
                Self::create_bind_group_layout_entries(info.get_pixel_shader_binary().unwrap());
            vertex_entries.append(&mut pixel_entries);
            vertex_entries
        };
        let bind_group_layout = device.to_data().get_device().create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &entries,
            },
        );

        let pipeline_layout =
            device
                .to_data()
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        Self {
            _vertex_shader_module: vertex_shader_module,
            _pixel_shader_module: pixel_shader_module,
            _compute_shader_module: compute_shader_module,
            _vertex_attributes: vertex_attributes,
            _bind_group_layout: bind_group_layout,
            _pipeline_layout: pipeline_layout,
            _marker: PhantomData,
        }
    }
}
