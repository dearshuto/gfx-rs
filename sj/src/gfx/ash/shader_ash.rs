use ash::version::DeviceV1_0;
use std::marker::PhantomData;

use super::super::shader_api::{IShaderImpl, ShaderInfo};
use super::super::Device;

pub struct ShaderImpl<'a> {
    _device: &'a Device,
    _shader: [Option<ash::vk::ShaderModule>; 6],
    _descriptor_set_layout: ash::vk::DescriptorSetLayout,
    _pipeline_layout: ash::vk::PipelineLayout,
    _compute_pipeline: Option<ash::vk::Pipeline>,
    _marker: PhantomData<&'a u32>,
}

impl<'a> ShaderImpl<'a> {
    pub fn get_compute_shader_modules(&self) -> &ash::vk::ShaderModule {
        &self._shader[0].as_ref().unwrap()
    }

    pub fn get_descriptor_set_layout(&self) -> &ash::vk::DescriptorSetLayout {
        &self._descriptor_set_layout
    }

    pub fn get_pipeline_layout(&self) -> &ash::vk::PipelineLayout {
        &self._pipeline_layout
    }

    pub fn get_compute_pipeline(&self) -> &ash::vk::Pipeline {
        &self._compute_pipeline.as_ref().unwrap()
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
        let device_impl = device.to_data().get_device();
        let mut compute_shader_binary = std::io::Cursor::new(info.get_shader_binary());
        let compute_shader_code = ash::util::read_spv(&mut compute_shader_binary).expect("");
        let compute_shader_module_create_info = ash::vk::ShaderModuleCreateInfo::builder()
            .code(&compute_shader_code)
            .build();

        unsafe {
            //let vertex_shader = device_impl.create_shader_module(&vertex_shader_module_create_info, None).unwrap();
            //let pixel_shader = device_impl.create_shader_module(&pixel_shader_module_create_info, None).unwrap();
            let compute_shader_module = device_impl
                .create_shader_module(&compute_shader_module_create_info, None)
                .unwrap();
            let shader_modiles = [
                None, //Some(vertex_shader),
                None, // Hull
                None, // Domain
                None, // Geometry
                None, //Some(pixel_shader),
                Some(compute_shader_module),
            ];

            let descriptor_set_layout_bindings =
                ShaderImpl::create_descriptor_set_layout_bindings(info.get_shader_binary());
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

            let shader_entry_name = std::ffi::CString::new("main").unwrap();
            let shader_stage_create_info = ash::vk::PipelineShaderStageCreateInfo::builder()
                .module(compute_shader_module)
                .name(&shader_entry_name)
                .stage(ash::vk::ShaderStageFlags::COMPUTE)
                .build();
            let compute_pipeline_create_info = ash::vk::ComputePipelineCreateInfo::builder()
                .stage(shader_stage_create_info)
                .layout(pipeline_layout)
                .build();
            let pipelines = device_impl
                .create_compute_pipelines(
                    ash::vk::PipelineCache::null(),
                    &[compute_pipeline_create_info],
                    None,
                )
                .unwrap();

            Self {
                _device: device,
                _shader: shader_modiles,
                _descriptor_set_layout: descriptor_set_layout,
                _pipeline_layout: pipeline_layout,
                _compute_pipeline: Some(pipelines[0]),
                _marker: PhantomData,
            }
        }
    }
}

impl<'a> Drop for ShaderImpl<'a> {
    fn drop(&mut self) {
        let device_impl = self._device.to_data().get_device();

        // パイプラインの破棄
        if let Some(pipeline) = self._compute_pipeline {
            unsafe {
                device_impl.destroy_pipeline(pipeline, None);
            }
        }

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
