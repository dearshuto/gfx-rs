use std::sync::Arc;

use sjgfx_interface::{IShader, ShaderInfo};
use wgpu::ComputePipelineDescriptor;

use crate::DeviceWgpu;

pub struct ShaderWgpu {
    shader_data: ShaderData,
}

impl ShaderWgpu {
    pub fn new(device: &DeviceWgpu, info: &ShaderInfo) -> Self {
        if let Some(compute_shader_binary) = info.get_compute_shader_binary() {
            return Self::new_as_compute(device, &compute_shader_binary);
        } else {
            return Self::new_as_graphics(
                device,
                info.get_vertex_shader_binary().as_ref().unwrap(),
                info.get_pixel_shader_binary().as_ref().unwrap(),
            );
        }
    }

    pub fn view(&self) -> ShaderView {
        ShaderView::new(self)
    }

    fn clone_shader_data(&self) -> ShaderData {
        self.shader_data.clone()
    }

    fn new_as_compute(device: &DeviceWgpu, shader_binary: &[u8]) -> Self {
        let compute_shader =
            Self::create_shader_module(device.get_device(), &Some(shader_binary)).unwrap();
        let entries = Self::create_bind_group_layout_entries(shader_binary);
        let bind_group_layout =
            device
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });
        let pipeline_layout =
            device
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        let compute_pipeline =
            device
                .get_device()
                .create_compute_pipeline(&ComputePipelineDescriptor {
                    label: None,
                    layout: Some(&pipeline_layout),
                    module: &compute_shader,
                    entry_point: "main",
                });

        Self {
            shader_data: ShaderData {
                compute_shader: Some(Arc::new(compute_shader)),
                vertex_shader: None,
                pixel_shader: None,
                compute_pipeline: Some(Arc::new(compute_pipeline)),
                vertex_attributes: None,
                bind_group_layout: Arc::new(bind_group_layout),
                pipeline_layout: Arc::new(pipeline_layout),
            },
        }
    }

    fn new_as_graphics(
        device: &DeviceWgpu,
        vertex_shader_binary: &[u8],
        pixel_shader_binary: &[u8],
    ) -> Self {
        let vertex_attributes = Self::create_vertex_attributes(&vertex_shader_binary);

        let vertex_shader =
            Self::create_shader_module(device.get_device(), &Some(vertex_shader_binary));
        let pixel_shader =
            Self::create_shader_module(device.get_device(), &Some(pixel_shader_binary));

        let entries = {
            let mut vertex_entries = Self::create_bind_group_layout_entries(vertex_shader_binary);
            let mut pixel_entries = Self::create_bind_group_layout_entries(pixel_shader_binary);
            vertex_entries.append(&mut pixel_entries);
            vertex_entries
        };
        let bind_group_layout =
            device
                .get_device()
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: None,
                    entries: &entries,
                });
        let pipeline_layout =
            device
                .get_device()
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&bind_group_layout],
                    push_constant_ranges: &[],
                });

        Self {
            shader_data: ShaderData {
                compute_shader: None,
                vertex_shader: Some(Arc::new(vertex_shader.unwrap())),
                pixel_shader: Some(Arc::new(pixel_shader.unwrap())),
                compute_pipeline: None,
                vertex_attributes: Some(Arc::new(vertex_attributes)),
                bind_group_layout: Arc::new(bind_group_layout),
                pipeline_layout: Arc::new(pipeline_layout),
            },
        }
    }

    fn create_shader_module(
        device: &wgpu::Device,
        sprv_binary_opt: &Option<&[u8]>,
    ) -> Option<wgpu::ShaderModule> {
        match sprv_binary_opt {
            Some(sprv_binary) => Some(device.create_shader_module(&wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::util::make_spirv(sprv_binary),
            })),
            None => None,
        }
    }

    fn create_bind_group_layout_entries(shader_source: &[u8]) -> Vec<wgpu::BindGroupLayoutEntry> {
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
                            min_binding_size: wgpu::BufferSize::new(x.block.size as u64),
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

    fn create_vertex_attributes(shader_source: &[u8]) -> Vec<wgpu::VertexAttribute> {
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_source).unwrap();
        module
            .enumerate_input_variables(None)
            .unwrap()
            .into_iter()
            .filter(|x| x.location < 31)
            .map(|x| wgpu::VertexAttribute {
                format: Self::convert_attribute_format(x.format),
                offset: 0,
                shader_location: x.location,
            })
            .collect::<Vec<wgpu::VertexAttribute>>()
            .to_vec()
    }

    fn convert_attribute_format(format: spirv_reflect::types::ReflectFormat) -> wgpu::VertexFormat {
        match format {
            spirv_reflect::types::ReflectFormat::Undefined => todo!(),
            spirv_reflect::types::ReflectFormat::R32_UINT => wgpu::VertexFormat::Uint32,
            spirv_reflect::types::ReflectFormat::R32_SINT => wgpu::VertexFormat::Sint32,
            spirv_reflect::types::ReflectFormat::R32_SFLOAT => wgpu::VertexFormat::Float32,
            spirv_reflect::types::ReflectFormat::R32G32_UINT => wgpu::VertexFormat::Uint32x2,
            spirv_reflect::types::ReflectFormat::R32G32_SINT => wgpu::VertexFormat::Sint32x2,
            spirv_reflect::types::ReflectFormat::R32G32_SFLOAT => wgpu::VertexFormat::Float32x2,
            spirv_reflect::types::ReflectFormat::R32G32B32_UINT => wgpu::VertexFormat::Uint32x3,
            spirv_reflect::types::ReflectFormat::R32G32B32_SINT => wgpu::VertexFormat::Sint32x3,
            spirv_reflect::types::ReflectFormat::R32G32B32_SFLOAT => wgpu::VertexFormat::Float32x3,
            spirv_reflect::types::ReflectFormat::R32G32B32A32_UINT => wgpu::VertexFormat::Uint32x4,
            spirv_reflect::types::ReflectFormat::R32G32B32A32_SINT => wgpu::VertexFormat::Sint32x4,
            spirv_reflect::types::ReflectFormat::R32G32B32A32_SFLOAT => {
                wgpu::VertexFormat::Float32x4
            }
        }
    }

    fn convert_shader_stage(
        stage: spirv_reflect::types::ReflectShaderStageFlags,
    ) -> wgpu::ShaderStages {
        if stage.contains(spirv_reflect::types::ReflectShaderStageFlags::COMPUTE) {
            wgpu::ShaderStages::COMPUTE
        } else if stage.contains(spirv_reflect::types::ReflectShaderStageFlags::VERTEX) {
            wgpu::ShaderStages::VERTEX
        } else if stage.contains(spirv_reflect::types::ReflectShaderStageFlags::FRAGMENT) {
            wgpu::ShaderStages::FRAGMENT
        } else {
            todo!()
        }
    }
}

impl IShader for ShaderWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &Self::DeviceType, info: &ShaderInfo) -> Self {
        Self::new(device, info)
    }
}

pub struct ShaderView {
    shader_data: ShaderData,
}

impl ShaderView {
    pub fn new(shader: &ShaderWgpu) -> Self {
        Self {
            shader_data: shader.clone_shader_data(),
        }
    }
}

impl ShaderView {
    pub fn is_compute(&self) -> bool {
        self.shader_data.compute_shader.is_some()
    }

    pub fn get_compute_shader_module(&self) -> &wgpu::ShaderModule {
        self.shader_data.compute_shader.as_ref().unwrap()
    }

    pub fn get_vertex_shader_module(&self) -> &wgpu::ShaderModule {
        self.shader_data.vertex_shader.as_ref().unwrap()
    }

    pub fn get_pixel_shader_module(&self) -> &wgpu::ShaderModule {
        self.shader_data.pixel_shader.as_ref().unwrap()
    }

    pub fn get_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.shader_data.bind_group_layout
    }

    pub fn get_pipeline_layout(&self) -> &wgpu::PipelineLayout {
        &self.shader_data.pipeline_layout
    }

    pub fn get_compute_pipeline(&self) -> &wgpu::ComputePipeline {
        self.shader_data.compute_pipeline.as_ref().unwrap()
    }
}

#[derive(Debug, Clone)]
struct ShaderData {
    pub compute_shader: Option<Arc<wgpu::ShaderModule>>,
    pub vertex_shader: Option<Arc<wgpu::ShaderModule>>,
    pub pixel_shader: Option<Arc<wgpu::ShaderModule>>,
    pub compute_pipeline: Option<Arc<wgpu::ComputePipeline>>,
    #[allow(dead_code)]
    pub vertex_attributes: Option<Arc<Vec<wgpu::VertexAttribute>>>,
    pub bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub pipeline_layout: Arc<wgpu::PipelineLayout>,
}

#[cfg(test)]
mod tests {
    // use sjgfx_interface::{IDevice, DeviceInfo, ShaderInfo};
    // use wgpu::VertexFormat;

    // use crate::{DeviceWgpu, ShaderWgpu};

    // #[test]
    // fn attribute() {
    //     let vertex_shader_source = "
    // 		#version 450
    // 		layout(location = 0) in vec2 i_Position;
    // 		layout(location = 1) in vec3 i_Normal;

    // 		layout(location = 0) out vec3 v_Normal;

    // 		void main() {
    // 			gl_Position = vec4(i_Position, 0.0, 1.0);
    //       v_Normal = i_Normal;
    // 		}";
    //     let pixel_shader_source = "
    // 		#version 450
    // 		layout(location = 0) out vec4 o_Color;
    // 		void main() {
    // 			o_Color = vec4(1.0, 0.0, 0.0, 1.0);
    // 		}";
    //     let mut compiler = shaderc::Compiler::new().unwrap();
    //     let vertex_shader_binary = compiler
    //         .compile_into_spirv(
    //             &vertex_shader_source,
    //             shaderc::ShaderKind::Vertex,
    //             "test.glsl",
    //             "main",
    //             None,
    //         )
    //         .unwrap();
    //     let pixel_shader_binary = compiler
    //         .compile_into_spirv(
    //             &pixel_shader_source,
    //             shaderc::ShaderKind::Fragment,
    //             "test.glsl",
    //             "main",
    //             None,
    //         )
    //         .unwrap();
    //     let device = DeviceWgpu::new(&DeviceInfo::new());
    //     let shader = ShaderWgpu::new(&device, &ShaderInfo::new().set_vertex_shader_binary(vertex_shader_binary.as_binary_u8()).set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()));

    //     let attributes = shader.get_vertex_attributes();
    //     assert_eq!(attributes.len(), 2);
    //     assert_eq!(attributes[0].shader_location, 0);
    //     assert_eq!(attributes[0].format, VertexFormat::Float32x2);
    //     assert_eq!(attributes[1].shader_location, 1);
    //     assert_eq!(attributes[1].format, VertexFormat::Float32x3);
    // }
}
