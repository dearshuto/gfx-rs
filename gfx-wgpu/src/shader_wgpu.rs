use std::sync::Arc;

use sjgfx_interface::{IShader, ShaderInfo, ShaderStage};
use sjgfx_util::ShaderReflection;
use uuid::Uuid;
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

    pub fn id(&self) -> &Uuid {
        &self.shader_data.id
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
        let shader_reflection = ShaderReflection::new_from_biinary(&shader_binary);
        let entries = Self::create_bind_group_layout_entries(
            shader_binary,
            &shader_reflection,
            &ShaderStage::Compute,
        );
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
                bind_group_layout: Arc::new(bind_group_layout),
                pipeline_layout: Arc::new(pipeline_layout),
                id: Uuid::new_v4(),
            },
        }
    }

    fn new_as_graphics(
        device: &DeviceWgpu,
        vertex_shader_binary: &[u8],
        pixel_shader_binary: &[u8],
    ) -> Self {
        let vertex_shader =
            Self::create_shader_module(device.get_device(), &Some(vertex_shader_binary));
        let pixel_shader =
            Self::create_shader_module(device.get_device(), &Some(pixel_shader_binary));

        let bind_group_layout = crate::util::create_bind_group_layout(
            device.get_device(),
            vertex_shader_binary,
            pixel_shader_binary,
        );

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
                bind_group_layout: Arc::new(bind_group_layout),
                pipeline_layout: Arc::new(pipeline_layout),
                id: Uuid::new_v4(),
            },
        }
    }

    fn create_shader_module(
        device: &wgpu::Device,
        sprv_binary_opt: &Option<&[u8]>,
    ) -> Option<wgpu::ShaderModule> {
        match sprv_binary_opt {
            Some(sprv_binary) => Some(device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: None,
                source: wgpu::util::make_spirv(sprv_binary),
            })),
            None => None,
        }
    }

    fn create_bind_group_layout_entries(
        shader_source: &[u8],
        shader_reflection: &ShaderReflection,
        shader_stage: &ShaderStage,
    ) -> Vec<wgpu::BindGroupLayoutEntry> {
        #[cfg(not(target_arch = "wasm32"))]
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_source).unwrap();

        #[cfg(target_arch = "wasm32")]
        let mut entries = Vec::new();

        #[cfg(not(target_arch = "wasm32"))]
        let mut entries = {
            module
                .enumerate_descriptor_bindings(None)
                .unwrap()
                .into_iter()
                .filter_map(|x| match x.descriptor_type {
                    spirv_reflect::types::ReflectDescriptorType::Undefined => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::Sampler => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: x.binding,
                            visibility: Self::convert_shader_stage(shader_stage),
                            ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                            count: None,
                        })
                    }
                    spirv_reflect::types::ReflectDescriptorType::CombinedImageSampler => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::SampledImage => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: x.binding,
                            visibility: Self::convert_shader_stage(shader_stage),
                            ty: Self::create_texture_bind_group_entry(&x),
                            count: None,
                        })
                    }
                    spirv_reflect::types::ReflectDescriptorType::StorageImage => {
                        Some(wgpu::BindGroupLayoutEntry {
                            binding: x.binding,
                            visibility: Self::convert_shader_stage(shader_stage),
                            ty: Self::create_image_bind_group_layout_entry(&x),
                            count: None,
                        })
                    }
                    spirv_reflect::types::ReflectDescriptorType::UniformTexelBuffer => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::StorageTexelBuffer => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::UniformBuffer => None/* util 実装に載せ替えた */,
                    spirv_reflect::types::ReflectDescriptorType::StorageBuffer => None/* util 実装に載せ替えた */,
                    spirv_reflect::types::ReflectDescriptorType::UniformBufferDynamic => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::StorageBufferDynamic => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::InputAttachment => todo!(),
                    spirv_reflect::types::ReflectDescriptorType::AccelerationStructureNV => todo!(),
                })
                .collect::<Vec<wgpu::BindGroupLayoutEntry>>()
                .to_vec()
        };

        let mut partial_entries =
            crate::util::create_bind_group_layout_entries(shader_reflection, shader_stage);

        entries.append(&mut partial_entries);
        entries
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_image_bind_group_layout_entry(
        info: &spirv_reflect::types::ReflectDescriptorBinding,
    ) -> wgpu::BindingType {
        wgpu::BindingType::StorageTexture {
            access: wgpu::StorageTextureAccess::ReadWrite,
            format: Self::convert_reflect_image_format(info.image.image_format),
            view_dimension: Self::convert_reflect_dimension(info.image.dim),
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_texture_bind_group_entry(
        info: &spirv_reflect::types::ReflectDescriptorBinding,
    ) -> wgpu::BindingType {
        let sample_type = if Self::is_float_format(info.image.image_format.clone()) {
            wgpu::TextureSampleType::Float { filterable: true }
        } else if Self::is_signed_int_format(info.image.image_format.clone()) {
            wgpu::TextureSampleType::Sint
        } else {
            wgpu::TextureSampleType::Uint
        };
        wgpu::BindingType::Texture {
            sample_type,
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn is_float_format(format: spirv_reflect::types::ReflectImageFormat) -> bool {
        match format {
            spirv_reflect::types::ReflectImageFormat::Undefined => true,
            spirv_reflect::types::ReflectImageFormat::RGBA32_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA16_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::R32_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA8 => true,
            spirv_reflect::types::ReflectImageFormat::RGBA8_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::RG32_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::RG16_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::R11G11B10_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::R16_FLOAT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA16 => true,
            spirv_reflect::types::ReflectImageFormat::RGB10A2 => true,
            spirv_reflect::types::ReflectImageFormat::RG16 => true,
            spirv_reflect::types::ReflectImageFormat::RG8 => true,
            spirv_reflect::types::ReflectImageFormat::R16 => true,
            spirv_reflect::types::ReflectImageFormat::R8 => true,
            spirv_reflect::types::ReflectImageFormat::RGBA16_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::RG16_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::RG8_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::R16_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::R8_SNORM => true,
            spirv_reflect::types::ReflectImageFormat::RGBA32_INT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16_INT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA8_INT => false,
            spirv_reflect::types::ReflectImageFormat::R32_INT => false,
            spirv_reflect::types::ReflectImageFormat::RG32_INT => false,
            spirv_reflect::types::ReflectImageFormat::RG16_INT => false,
            spirv_reflect::types::ReflectImageFormat::RG8_INT => false,
            spirv_reflect::types::ReflectImageFormat::R16_INT => false,
            spirv_reflect::types::ReflectImageFormat::R8_INT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA8_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGB10A2_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG8_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R8_UINT => false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn is_signed_int_format(format: spirv_reflect::types::ReflectImageFormat) -> bool {
        match format {
            spirv_reflect::types::ReflectImageFormat::Undefined => todo!(),
            spirv_reflect::types::ReflectImageFormat::RGBA32_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::R32_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA8 => false,
            spirv_reflect::types::ReflectImageFormat::RGBA8_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::RG32_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::RG16_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::R11G11B10_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::R16_FLOAT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16 => false,
            spirv_reflect::types::ReflectImageFormat::RGB10A2 => false,
            spirv_reflect::types::ReflectImageFormat::RG16 => false,
            spirv_reflect::types::ReflectImageFormat::RG8 => false,
            spirv_reflect::types::ReflectImageFormat::R16 => false,
            spirv_reflect::types::ReflectImageFormat::R8 => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::RG16_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::RG8_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::R16_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::R8_SNORM => false,
            spirv_reflect::types::ReflectImageFormat::RGBA32_INT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA16_INT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA8_INT => true,
            spirv_reflect::types::ReflectImageFormat::R32_INT => true,
            spirv_reflect::types::ReflectImageFormat::RG32_INT => true,
            spirv_reflect::types::ReflectImageFormat::RG16_INT => true,
            spirv_reflect::types::ReflectImageFormat::RG8_INT => true,
            spirv_reflect::types::ReflectImageFormat::R16_INT => true,
            spirv_reflect::types::ReflectImageFormat::R8_INT => true,
            spirv_reflect::types::ReflectImageFormat::RGBA32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGBA8_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RGB10A2_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG32_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::RG8_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R16_UINT => false,
            spirv_reflect::types::ReflectImageFormat::R8_UINT => false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn convert_reflect_image_format(
        format: spirv_reflect::types::ReflectImageFormat,
    ) -> wgpu::TextureFormat {
        match format {
            spirv_reflect::types::ReflectImageFormat::Undefined => todo!(),
            spirv_reflect::types::ReflectImageFormat::RGBA32_FLOAT => {
                wgpu::TextureFormat::Rgba32Float
            }
            spirv_reflect::types::ReflectImageFormat::RGBA16_FLOAT => {
                wgpu::TextureFormat::Rgba16Float
            }
            spirv_reflect::types::ReflectImageFormat::R32_FLOAT => wgpu::TextureFormat::R32Float,
            spirv_reflect::types::ReflectImageFormat::RGBA8 => wgpu::TextureFormat::Rgba8Unorm,
            spirv_reflect::types::ReflectImageFormat::RGBA8_SNORM => {
                wgpu::TextureFormat::Rgba8Snorm
            }
            spirv_reflect::types::ReflectImageFormat::RG32_FLOAT => wgpu::TextureFormat::Rg32Float,
            spirv_reflect::types::ReflectImageFormat::RG16_FLOAT => wgpu::TextureFormat::Rg16Float,
            spirv_reflect::types::ReflectImageFormat::R11G11B10_FLOAT => {
                wgpu::TextureFormat::Rg11b10Float
            }
            spirv_reflect::types::ReflectImageFormat::R16_FLOAT => wgpu::TextureFormat::R16Float,
            spirv_reflect::types::ReflectImageFormat::RGBA16 => wgpu::TextureFormat::Rgba16Unorm,
            spirv_reflect::types::ReflectImageFormat::RGB10A2 => wgpu::TextureFormat::Rgb10a2Unorm,
            spirv_reflect::types::ReflectImageFormat::RG16 => wgpu::TextureFormat::Rg16Unorm,
            spirv_reflect::types::ReflectImageFormat::RG8 => wgpu::TextureFormat::Rg8Unorm,
            spirv_reflect::types::ReflectImageFormat::R16 => wgpu::TextureFormat::R16Unorm,
            spirv_reflect::types::ReflectImageFormat::R8 => wgpu::TextureFormat::R8Unorm,
            spirv_reflect::types::ReflectImageFormat::RGBA16_SNORM => {
                wgpu::TextureFormat::Rgba16Snorm
            }
            spirv_reflect::types::ReflectImageFormat::RG16_SNORM => wgpu::TextureFormat::Rg16Snorm,
            spirv_reflect::types::ReflectImageFormat::RG8_SNORM => wgpu::TextureFormat::Rg8Snorm,
            spirv_reflect::types::ReflectImageFormat::R16_SNORM => wgpu::TextureFormat::R16Snorm,
            spirv_reflect::types::ReflectImageFormat::R8_SNORM => wgpu::TextureFormat::R8Snorm,
            spirv_reflect::types::ReflectImageFormat::RGBA32_INT => wgpu::TextureFormat::Rgba32Sint,
            spirv_reflect::types::ReflectImageFormat::RGBA16_INT => wgpu::TextureFormat::Rgba16Sint,
            spirv_reflect::types::ReflectImageFormat::RGBA8_INT => wgpu::TextureFormat::Rgba8Sint,
            spirv_reflect::types::ReflectImageFormat::R32_INT => wgpu::TextureFormat::R32Sint,
            spirv_reflect::types::ReflectImageFormat::RG32_INT => wgpu::TextureFormat::Rg32Sint,
            spirv_reflect::types::ReflectImageFormat::RG16_INT => wgpu::TextureFormat::Rg16Sint,
            spirv_reflect::types::ReflectImageFormat::RG8_INT => wgpu::TextureFormat::Rg8Sint,
            spirv_reflect::types::ReflectImageFormat::R16_INT => wgpu::TextureFormat::R16Sint,
            spirv_reflect::types::ReflectImageFormat::R8_INT => wgpu::TextureFormat::R8Sint,
            spirv_reflect::types::ReflectImageFormat::RGBA32_UINT => {
                wgpu::TextureFormat::Rgba32Uint
            }
            spirv_reflect::types::ReflectImageFormat::RGBA16_UINT => {
                wgpu::TextureFormat::Rgba16Uint
            }
            spirv_reflect::types::ReflectImageFormat::RGBA8_UINT => wgpu::TextureFormat::Rgba8Uint,
            spirv_reflect::types::ReflectImageFormat::R32_UINT => wgpu::TextureFormat::R32Uint,
            spirv_reflect::types::ReflectImageFormat::RGB10A2_UINT => todo!(),
            spirv_reflect::types::ReflectImageFormat::RG32_UINT => wgpu::TextureFormat::Rg32Uint,
            spirv_reflect::types::ReflectImageFormat::RG16_UINT => wgpu::TextureFormat::Rg16Uint,
            spirv_reflect::types::ReflectImageFormat::RG8_UINT => wgpu::TextureFormat::Rg8Uint,
            spirv_reflect::types::ReflectImageFormat::R16_UINT => wgpu::TextureFormat::R16Uint,
            spirv_reflect::types::ReflectImageFormat::R8_UINT => wgpu::TextureFormat::R8Uint,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn convert_reflect_dimension(
        dimension: spirv_reflect::types::ReflectDimension,
    ) -> wgpu::TextureViewDimension {
        match dimension {
            spirv_reflect::types::ReflectDimension::Undefined => todo!(),
            spirv_reflect::types::ReflectDimension::Type1d => wgpu::TextureViewDimension::D1,
            spirv_reflect::types::ReflectDimension::Type2d => wgpu::TextureViewDimension::D2,
            spirv_reflect::types::ReflectDimension::Type3d => wgpu::TextureViewDimension::D3,
            spirv_reflect::types::ReflectDimension::Cube => wgpu::TextureViewDimension::Cube,
            spirv_reflect::types::ReflectDimension::Rect => todo!(),
            spirv_reflect::types::ReflectDimension::Buffer => todo!(),
            spirv_reflect::types::ReflectDimension::SubPassData => todo!(),
        }
    }

    fn convert_shader_stage(stage: &ShaderStage) -> wgpu::ShaderStages {
        crate::util::convert_shader_stage(stage.clone())
    }
}

impl IShader for ShaderWgpu {
    type DeviceType = DeviceWgpu;

    fn new(device: &mut Self::DeviceType, info: &ShaderInfo) -> Self {
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

    pub fn get_id(&self) -> &Uuid {
        &self.shader_data.id
    }
}

#[derive(Debug, Clone)]
struct ShaderData {
    pub compute_shader: Option<Arc<wgpu::ShaderModule>>,
    pub vertex_shader: Option<Arc<wgpu::ShaderModule>>,
    pub pixel_shader: Option<Arc<wgpu::ShaderModule>>,
    pub compute_pipeline: Option<Arc<wgpu::ComputePipeline>>,
    pub bind_group_layout: Arc<wgpu::BindGroupLayout>,
    pub pipeline_layout: Arc<wgpu::PipelineLayout>,
    pub id: Uuid,
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

    use sjgfx_interface::{DebugMode, DeviceInfo, IDevice, ShaderInfo};

    use crate::{DeviceWgpu, ShaderWgpu};

    #[test]
    fn new_image_shader() {
        let shader_source = "
        		#version 450

            layout (local_size_x=8, local_size_y=8, local_size_z=1) in;

            layout (binding=0, r32i) uniform iimage2D u_Image;

        		void main() {
              int x = int(gl_GlobalInvocationID.x);
              int y = int(gl_GlobalInvocationID.y);

              imageStore(u_Image, ivec2(x, y), ivec4(1, 0, 0, 0));
        		}";
        let mut compiler = shaderc::Compiler::new().unwrap();
        let shader_binary = compiler
            .compile_into_spirv(
                &shader_source,
                shaderc::ShaderKind::Compute,
                "test.glsl",
                "main",
                None,
            )
            .unwrap();
        let device = DeviceWgpu::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let _ = ShaderWgpu::new(
            &device,
            &ShaderInfo::new().set_compute_shader_binary(shader_binary.as_binary_u8()),
        );
    }

    #[test]
    fn new_texture_shader() {
        let vertex_shader_source = "
        		#version 450
        		void main() {
                gl_Position = vec4(1.0);
        		}";
        let pixel_shader_source = "
        		#version 450

            layout (location = 0) out vec4 o_Color;

            layout (binding = 0) uniform texture2D u_Texture;
            layout (binding = 1) uniform sampler u_Sampler;

        		void main() {
                float value = texture(sampler2D(u_Texture, u_Sampler), vec2(0.0)).r;
                o_Color = vec4(float(value));
        		}";
        let mut compiler = shaderc::Compiler::new().unwrap();
        let vertex_shader_binary = compiler
            .compile_into_spirv(
                &vertex_shader_source,
                shaderc::ShaderKind::Vertex,
                "test.vs",
                "main",
                None,
            )
            .unwrap();
        let pixel_shader_binary = compiler
            .compile_into_spirv(
                &pixel_shader_source,
                shaderc::ShaderKind::Fragment,
                "test.fs",
                "main",
                None,
            )
            .unwrap();
        let device = DeviceWgpu::new(&DeviceInfo::new().set_debug_mode(DebugMode::FullAssertion));
        let _ = ShaderWgpu::new(
            &device,
            &ShaderInfo::new()
                .set_vertex_shader_binary(vertex_shader_binary.as_binary_u8())
                .set_pixel_shader_binary(pixel_shader_binary.as_binary_u8()),
        );
    }
}
