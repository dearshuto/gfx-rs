use ash::version::DeviceV1_0;
use std::marker::PhantomData;

use super::super::shader_api::{IShaderImpl, ShaderInfo};
use super::super::Device;

pub struct ShaderImpl<'a> {
    _device: &'a Device,
    _shader: [Option<ash::vk::ShaderModule>; 6],
    _descriptor_set_layout: ash::vk::DescriptorSetLayout,
    _pipeline_layout: ash::vk::PipelineLayout,
    _marker: PhantomData<&'a u32>,
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
        let descriptor_set_layout_bindings =
            Self::create_descriptor_set_layout_bindings(info.get_shader_binary());
        unsafe {
            let descriptor_set_layout = device_impl
                .create_descriptor_set_layout(
                    &ash::vk::DescriptorSetLayoutCreateInfo::builder()
                        .bindings(descriptor_set_layout_bindings.as_slice())
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
                _marker: PhantomData,
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
        let vertex_descriptor_set_layout_bindings =
            Self::create_descriptor_set_layout_bindings(info.get_vertex_shader_binary().unwrap());
        unsafe {
            let descriptor_set_layout = device_impl
                .create_descriptor_set_layout(
                    &ash::vk::DescriptorSetLayoutCreateInfo::builder()
                        .bindings(vertex_descriptor_set_layout_bindings.as_slice())
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
                _marker: PhantomData,
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
    ) -> Vec<ash::vk::DescriptorSetLayoutBinding> {
        let module = spirv_reflect::ShaderModule::load_u8_data(shader_binary).unwrap();
        let bindings = module.enumerate_descriptor_bindings(None).unwrap();

        let mut result = Vec::new();
        let mut _uniform_buffer_count = 0;
        let mut storage_buffer_count = 0;
        for set in bindings {
            match set.descriptor_type {
                //spirv_reflect::types::ReflectDescriptorType::UniformBuffer => uniform_buffer_count += 1,
                spirv_reflect::types::ReflectDescriptorType::StorageBuffer => {
                    storage_buffer_count += 1
                }
                _ => {}
            };
        }

        result.push(
            ash::vk::DescriptorSetLayoutBinding::builder()
                .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
                .descriptor_count(storage_buffer_count)
                .stage_flags(ash::vk::ShaderStageFlags::COMPUTE)
                .build(),
        );
        result
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
