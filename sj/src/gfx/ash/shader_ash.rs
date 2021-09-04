use ash::version::DeviceV1_0;
use std::ops::Index;

use crate::gfx::{ShaderInterfaceSlotType, ShaderStage};

use super::super::shader_api::{IShaderImpl, ShaderInfo};
use super::super::{Device, GpuAccess};

pub struct ShaderImpl<'a> {
    _device: &'a Device,
    _shader: [Option<ash::vk::ShaderModule>; 6],
    _descriptor_set_layout: ash::vk::DescriptorSetLayout,
    _pipeline_layout: ash::vk::PipelineLayout,
    _layout_table_vertex: Option<std::sync::Arc<LayoutTable>>,
    _layout_table_pixel: Option<std::sync::Arc<LayoutTable>>,
    _layout_table_compute: Option<std::sync::Arc<LayoutTable>>,
    _interface_slot_table: [std::collections::HashMap<String, i32>; 2],
}

impl<'a> ShaderImpl<'a> {
    pub fn get_vertex_shader_module(&self) -> &ash::vk::ShaderModule {
        &self._shader[0].as_ref().unwrap()
    }

    pub fn get_pixel_shader_module(&self) -> &ash::vk::ShaderModule {
        &self._shader[4].as_ref().unwrap()
    }

    pub fn get_compute_shader_modules(&self) -> &ash::vk::ShaderModule {
        &self._shader[5].as_ref().unwrap()
    }

    pub fn get_descriptor_set_layout(&self) -> &ash::vk::DescriptorSetLayout {
        &self._descriptor_set_layout
    }

    pub fn get_pipeline_layout(&self) -> &ash::vk::PipelineLayout {
        &self._pipeline_layout
    }

    pub fn get_layout_table(&self, shader_stage: &ShaderStage) -> &LayoutTable {
        match shader_stage {
            &ShaderStage::Vertex => self._layout_table_vertex.as_ref().unwrap(),
            &ShaderStage::Pixel => self._layout_table_pixel.as_ref().unwrap(),
            &ShaderStage::Compute => self._layout_table_compute.as_ref().unwrap(),
        }
    }

    fn new_as_compute_shader(device: &'a Device, info: &ShaderInfo) -> Self {
        let shader_modiles = [
            None, // Vertex
            None, // Hull
            None, // Domain
            None, // Geometry
            None, // Pixel
            ShaderImpl::create_shader_module(device, info.get_compute_shader_binary()),
        ];

        let device_impl = device.to_data().get_device();
        let layout_table = Self::create_descriptor_set_layout_bindings(
            info.get_shader_binary(),
            ShaderStage::Compute,
        );
        let descriptor_set_layout_bindings = layout_table.get_descriptor_set_layout_bindings();
        unsafe {
            let descriptor_set_layout = device_impl
                .create_descriptor_set_layout(
                    &ash::vk::DescriptorSetLayoutCreateInfo::builder()
                        .bindings(descriptor_set_layout_bindings)
                        .build(),
                    None,
                )
                .unwrap();
            let pipeline_layout = device_impl
                .create_pipeline_layout(
                    &ash::vk::PipelineLayoutCreateInfo::builder()
                        .set_layouts(&[descriptor_set_layout])
                        .build(),
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _shader: shader_modiles,
                _descriptor_set_layout: descriptor_set_layout,
                _pipeline_layout: pipeline_layout,
                _layout_table_vertex: None,
                _layout_table_pixel: None,
                _layout_table_compute: Some(std::sync::Arc::new(layout_table)),
                _interface_slot_table: [
                    std::collections::HashMap::<String, i32>::new(),
                    std::collections::HashMap::<String, i32>::new(),
                ],
            }
        }
    }

    fn new_as_graphics_shader(device: &'a Device, info: &ShaderInfo) -> Self {
        let shader_modiles = [
            Self::create_shader_module(device, info.get_vertex_shader_binary()), // Vertex
            None,                                                                // Hull
            None,                                                                // Domain
            None,                                                                // Geometry
            Self::create_shader_module(device, info.get_pixel_shader_binary()),  // Pixel
            None,                                                                // Compute
        ];

        let device_impl = device.to_data().get_device();
        let layout_table = Self::create_descriptor_set_layout_bindings(
            info.get_vertex_shader_binary().unwrap(),
            ShaderStage::Vertex,
        );
        let vertex_descriptor_set_layout_bindings =
            layout_table.get_descriptor_set_layout_bindings();
        unsafe {
            let descriptor_set_layout = device_impl
                .create_descriptor_set_layout(
                    &ash::vk::DescriptorSetLayoutCreateInfo::builder()
                        .bindings(vertex_descriptor_set_layout_bindings)
                        .build(),
                    None,
                )
                .unwrap();
            let pipeline_layout = device_impl
                .create_pipeline_layout(
                    &ash::vk::PipelineLayoutCreateInfo::builder()
                        .set_layouts(&[descriptor_set_layout])
                        .build(),
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _shader: shader_modiles,
                _descriptor_set_layout: descriptor_set_layout,
                _pipeline_layout: pipeline_layout,
                _layout_table_vertex: Some(std::sync::Arc::new(layout_table)),
                _layout_table_pixel: None,
                _layout_table_compute: None,
                _interface_slot_table: [
                    std::collections::HashMap::<String, i32>::new(),
                    std::collections::HashMap::<String, i32>::new(),
                ],
            }
        }
    }

    fn create_shader_module(
        device: &Device,
        shader_binary_opt: &Option<&[u8]>,
    ) -> Option<ash::vk::ShaderModule> {
        match shader_binary_opt {
            Some(shader_binary) => {
                let device_impl = device.to_data().get_device();
                let mut compute_shader_binary = std::io::Cursor::new(shader_binary);
                let compute_shader_code =
                    ash::util::read_spv(&mut compute_shader_binary).expect("");
                let compute_shader_module_create_info = ash::vk::ShaderModuleCreateInfo::builder()
                    .code(&compute_shader_code)
                    .build();

                unsafe {
                    let shader_module_result =
                        device_impl.create_shader_module(&compute_shader_module_create_info, None);
                    match shader_module_result {
                        Ok(result) => Some(result),
                        Err(_message) => None,
                    }
                }
            }
            None => None,
        }
    }

    fn create_descriptor_set_layout_bindings(
        shader_binary: &[u8],
        shader_stage: ShaderStage,
    ) -> LayoutTable {
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_binary).unwrap();
        let bindings = module.enumerate_descriptor_bindings(None).unwrap();

        let mut uniform_buffer_count = 0;
        let mut storage_buffer_count = 0;
        for set in bindings {
            match set.descriptor_type {
                spirv_reflect::types::ReflectDescriptorType::UniformBuffer => {
                    uniform_buffer_count += 1;
                }
                spirv_reflect::types::ReflectDescriptorType::StorageBuffer => {
                    storage_buffer_count += 1
                }

                _ => {}
            };
        }

        let table = LayoutTable::new(
            shader_stage,
            uniform_buffer_count,
            storage_buffer_count,
            0,
            0,
        );
        table
    }
}

impl<'a> IShaderImpl<'a> for ShaderImpl<'a> {
    fn new(device: &'a Device, info: &ShaderInfo) -> Self {
        if info.get_compute_shader_binary().is_some() {
            Self::new_as_compute_shader(device, info)
        } else {
            Self::new_as_graphics_shader(device, info)
        }
    }

    fn get_interface_slot(&self, interface_slot_type: ShaderInterfaceSlotType, name: &str) -> i32 {
        -1
    }
}

impl<'a> Drop for ShaderImpl<'a> {
    fn drop(&mut self) {
        let device_impl = self._device.to_data().get_device();

        // パイプラインレイアウトの破棄
        unsafe {
            device_impl.destroy_pipeline_layout(self._pipeline_layout, None);
        }

        unsafe {
            device_impl.destroy_descriptor_set_layout(self._descriptor_set_layout, None);
        }

        // シェーダモジュールの破棄
        for shader_module_option in self._shader.iter() {
            if let Some(shader_module) = shader_module_option {
                unsafe {
                    device_impl.destroy_shader_module(*shader_module, None);
                }
            }
        }
    }
}

impl ShaderStage {
    pub fn to_ash(&self) -> ash::vk::ShaderStageFlags {
        match self {
            &ShaderStage::Vertex => ash::vk::ShaderStageFlags::VERTEX,
            &ShaderStage::Pixel => ash::vk::ShaderStageFlags::FRAGMENT,
            &ShaderStage::Compute => ash::vk::ShaderStageFlags::COMPUTE,
        }
    }
}

pub struct LayoutTable {
    _descriptor_set_layout_bindings: Vec<ash::vk::DescriptorSetLayoutBinding>,
    _indices: [Vec<u32>; 4],
}

impl LayoutTable {
    pub fn new(
        shader_stage: ShaderStage,
        uniform_block_count: u32,
        shader_storage_block_count: u32,
        _texture_count: u32,
        _image_count: u32,
    ) -> Self {
        let mut descriptor_set_layout_bindings = Vec::new();

        // Uniform Block
        if uniform_block_count > 0 {
            descriptor_set_layout_bindings.push(
                ash::vk::DescriptorSetLayoutBinding::builder()
                    .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(uniform_block_count)
                    .stage_flags(shader_stage.to_ash())
                    .binding(0)
                    .build(),
            );
        }

        // Shader Storage Block
        if shader_storage_block_count > 0 {
            descriptor_set_layout_bindings.push(
                ash::vk::DescriptorSetLayoutBinding::builder()
                    .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(shader_storage_block_count)
                    .stage_flags(shader_stage.to_ash())
                    .build(),
            );
        }

        Self {
            _descriptor_set_layout_bindings: descriptor_set_layout_bindings,
            _indices: [
                (0..uniform_block_count).map(|x| x).collect::<Vec<u32>>(),
                (0..shader_storage_block_count)
                    .map(|x| uniform_block_count + x)
                    .collect::<Vec<u32>>(),
                Vec::new(),
                Vec::new(),
            ],
        }
    }

    pub fn get_descriptor_set_layout_bindings(&self) -> &[ash::vk::DescriptorSetLayoutBinding] {
        &self._descriptor_set_layout_bindings
    }

    fn enum_to_index(gpu_access: &GpuAccess) -> usize {
        match gpu_access {
            &GpuAccess::CONSTANT_BUFFER => 0,
            &GpuAccess::UNORDERED_ACCESS_BUFFER => 1,
            &GpuAccess::TEXTURE => 2,
            &GpuAccess::IMAGE => 3,
            _ => todo!(),
        }
    }
}

impl Index<GpuAccess> for LayoutTable {
    type Output = [u32];

    fn index(&self, index: GpuAccess) -> &Self::Output {
        let actual_index = LayoutTable::enum_to_index(&index);
        let array = &self._indices[actual_index];
        &array
    }
}
