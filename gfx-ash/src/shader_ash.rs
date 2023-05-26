use sjgfx_interface::{IShader, ShaderInfo, ShaderStage};

use crate::DeviceAsh;

struct DescriptorInfo {
    pub stage: ShaderStage,
    pub constant_buffer_count: u32,
    pub unordered_access_buffer_count: u32,
    #[allow(dead_code)]
    pub sampler_count: u32,
    pub image_count: u32,
}

pub struct ShaderAsh {
    device: ash::Device,
    descriptor_set_layouts: Vec<ash::vk::DescriptorSetLayout>,
    pipeline_layout: ash::vk::PipelineLayout,
    compute_shader_object: Option<ash::extensions::ext::ShaderObject>,
    render_shader_object: Option<ash::extensions::ext::ShaderObject>,
    id: uuid::Uuid,
    // vertex_stage_descriptor_info: Option<DescriptorInfo>,
    // pixel_stage_descriptor_info: Option<DescriptorInfo>,
    // compute_state_descriptor_info: Option<DescriptorInfo>,
}

impl ShaderAsh {
    pub fn new(device: &DeviceAsh, info: &ShaderInfo) -> Self {
        if info.get_compute_shader_binary().is_some() {
            Self::new_as_compute(device, info)
        } else {
            Self::new_as_graphics(device, info)
        }
    }

    pub fn is_compute(&self) -> bool {
        self.compute_shader_object.is_some()
    }

    pub fn get_compute_shader_object(&self) -> ash::extensions::ext::ShaderObject {
        self.compute_shader_object.as_ref().unwrap().clone()
    }

    pub fn get_render_shader_object(&self) -> ash::extensions::ext::ShaderObject {
        self.render_shader_object.as_ref().unwrap().clone()
    }

    pub fn get_pipeline_layout(&self) -> ash::vk::PipelineLayout {
        self.pipeline_layout
    }

    pub fn get_descriptor_set_layouts(&self) -> &[ash::vk::DescriptorSetLayout] {
        &self.descriptor_set_layouts
    }

    pub fn get_id(&self) -> &uuid::Uuid {
        &self.id
    }

    fn new_as_graphics(device: &DeviceAsh, info: &ShaderInfo) -> Self {
        let vertex_stage_dexcriptor_info = Self::create_descriptor_info(
            ShaderStage::Vertex,
            info.get_vertex_shader_binary().unwrap(),
        );
        let pixel_stage_dexcriptor_info = Self::create_descriptor_info(
            ShaderStage::Pixel,
            info.get_pixel_shader_binary().unwrap(),
        );
        let (descriptor_set_layout, pipeline_layout) =
            Self::create_descriptor_set_layout_and_pipeline_layout(
                device,
                &[&vertex_stage_dexcriptor_info, &pixel_stage_dexcriptor_info],
            );

        Self {
            device: device.get_device(),
            compute_shader_object: None,
            render_shader_object: Some(Self::create_shader_object(device, &[])),
            descriptor_set_layouts: vec![descriptor_set_layout],
            pipeline_layout,
            id: uuid::Uuid::new_v4(),
        }
    }

    fn new_as_compute(device: &DeviceAsh, info: &ShaderInfo) -> Self {
        let descriptor_info = Self::create_descriptor_info(
            ShaderStage::Compute,
            info.get_compute_shader_binary().unwrap(),
        );
        let (descriptor_set_layout, pipeline_layout) =
            Self::create_descriptor_set_layout_and_pipeline_layout(device, &[&descriptor_info]);

        Self {
            device: device.get_device(),
            compute_shader_object: Some(Self::create_shader_object(
                device,
                // &[info.get_compute_shader_binary().unwrap()],
                &[],
            )),
            render_shader_object: None,
            descriptor_set_layouts: vec![descriptor_set_layout],
            pipeline_layout,
            id: uuid::Uuid::new_v4(),
        }
    }

    fn create_shader_object(
        device: &DeviceAsh,
        shader_binary: &[&[u8]],
    ) -> ash::extensions::ext::ShaderObject {
        let instance = device.get_instance();
        let device = device.get_device_ref();
        let mut shader_object = ash::extensions::ext::ShaderObject::new(instance, device);

        let shader_create_info_array = shader_binary
            .iter()
            .map(|binary| ash::vk::ShaderCreateInfoEXT::default().code(binary))
            .collect::<Vec<ash::vk::ShaderCreateInfoEXT>>();
        unsafe { shader_object.create_shaders(&shader_create_info_array, None) };
        shader_object
    }

    fn crate_shader_module(device: &DeviceAsh, shader_binary: &[u8]) -> ash::vk::ShaderModule {
        let mut compute_shader_binary = std::io::Cursor::new(shader_binary);
        let shader_code = ash::util::read_spv(&mut compute_shader_binary).expect("");

        let shader_module_create_info =
            ash::vk::ShaderModuleCreateInfo::default().code(&shader_code);

        let shader_module = unsafe {
            device
                .get_device()
                .create_shader_module(&shader_module_create_info, None)
        }
        .unwrap();
        shader_module
    }

    fn create_descriptor_info(shader_stage: ShaderStage, shader_binary: &[u8]) -> DescriptorInfo {
        #[allow(unused_variables)]
        let mut sampler_count = 0;
        let mut unordered_access_buffer_count = 0;
        let mut constant_buffer_count = 0;
        let mut image_count = 0;

        let module = spirv_reflect::ShaderModule::load_u8_data(shader_binary).unwrap();
        for item in module.enumerate_descriptor_bindings(None).unwrap().iter() {
            match item.descriptor_type {
                spirv_reflect::types::ReflectDescriptorType::Undefined => todo!(),
                spirv_reflect::types::ReflectDescriptorType::Sampler => sampler_count += 1,
                spirv_reflect::types::ReflectDescriptorType::CombinedImageSampler => todo!(),
                spirv_reflect::types::ReflectDescriptorType::SampledImage => todo!(),
                spirv_reflect::types::ReflectDescriptorType::StorageImage => image_count += 1,
                spirv_reflect::types::ReflectDescriptorType::UniformTexelBuffer => todo!(),
                spirv_reflect::types::ReflectDescriptorType::StorageTexelBuffer => todo!(),
                spirv_reflect::types::ReflectDescriptorType::UniformBuffer => {
                    constant_buffer_count += 1
                }
                spirv_reflect::types::ReflectDescriptorType::StorageBuffer => {
                    unordered_access_buffer_count += 1
                }
                spirv_reflect::types::ReflectDescriptorType::UniformBufferDynamic => todo!(),
                spirv_reflect::types::ReflectDescriptorType::StorageBufferDynamic => todo!(),
                spirv_reflect::types::ReflectDescriptorType::InputAttachment => todo!(),
                spirv_reflect::types::ReflectDescriptorType::AccelerationStructureNV => todo!(),
            }
        }

        DescriptorInfo {
            stage: shader_stage,
            constant_buffer_count,
            unordered_access_buffer_count,
            sampler_count,
            image_count,
        }
    }

    fn create_descriptor_set_layout_and_pipeline_layout(
        device: &DeviceAsh,
        descriptor_infos: &[&DescriptorInfo],
    ) -> (ash::vk::DescriptorSetLayout, ash::vk::PipelineLayout) {
        let tables = descriptor_infos.iter().map(|x| {
            LayoutTable::new(
                Self::to_ash(x.stage.clone()),
                x.constant_buffer_count,
                x.unordered_access_buffer_count,
                0, /*texture_count*/
                x.image_count,
            )
        });

        let mut descriptor_set_layout_bindings = Vec::new();
        // for table in tables {
        //     for binding in table.get_descriptor_set_layout_bindings() {
        //         descriptor_set_layout_bindings.push(*binding);
        //     }
        // }

        let descriptor_set_layout = unsafe {
            device.get_device().create_descriptor_set_layout(
                &ash::vk::DescriptorSetLayoutCreateInfo::default()
                    .bindings(&descriptor_set_layout_bindings),
                None,
            )
        }
        .unwrap();

        let pipeline_layout = unsafe {
            device.get_device().create_pipeline_layout(
                &ash::vk::PipelineLayoutCreateInfo::default().set_layouts(&[descriptor_set_layout]),
                None,
            )
        }
        .unwrap();

        (descriptor_set_layout, pipeline_layout)
    }

    pub fn to_ash(shader_stage: ShaderStage) -> ash::vk::ShaderStageFlags {
        match shader_stage {
            ShaderStage::Vertex => ash::vk::ShaderStageFlags::VERTEX,
            ShaderStage::Pixel => ash::vk::ShaderStageFlags::FRAGMENT,
            ShaderStage::Compute => ash::vk::ShaderStageFlags::COMPUTE,
        }
    }
}

impl IShader for ShaderAsh {
    type DeviceType = DeviceAsh;

    fn new(device: &mut Self::DeviceType, info: &ShaderInfo) -> Self {
        Self::new(device, info)
    }
}

impl Drop for ShaderAsh {
    fn drop(&mut self) {
        // パイプラインレイアウトの破棄
        unsafe {
            self.device
                .destroy_pipeline_layout(self.pipeline_layout, None);
        }

        // デスクリプタセットレイアウトの破棄
        for descriptor_set_layout in &self.descriptor_set_layouts {
            unsafe {
                self.device
                    .destroy_descriptor_set_layout(*descriptor_set_layout, None);
            }
        }

        // 演算シェーダの破棄
        if let Some(_compute_shader_object) = &self.compute_shader_object {
            // unsafe { compute_shader_object.destroy_shader(shader, allocator) }
        }

        // 描画シェーダの破棄
        if let Some(_render_shader_object) = &self.render_shader_object {
            // unsafe { compute_shader_object.destroy_shader(shader, allocator) }
        }
    }
}

pub struct LayoutTable<'a> {
    _descriptor_set_layout_bindings: Vec<ash::vk::DescriptorSetLayoutBinding<'a>>,
}

impl<'a> LayoutTable<'a> {
    pub fn new(
        shader_stage: ash::vk::ShaderStageFlags,
        uniform_block_count: u32,
        shader_storage_block_count: u32,
        _texture_count: u32,
        image_count: u32,
    ) -> Self {
        let mut descriptor_set_layout_bindings = Vec::new();

        // Uniform Block
        if uniform_block_count > 0 {
            descriptor_set_layout_bindings.push(
                ash::vk::DescriptorSetLayoutBinding::default()
                    .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
                    .descriptor_count(uniform_block_count)
                    .stage_flags(shader_stage)
                    .binding(0),
            );
        }

        // Shader Storage Block
        if shader_storage_block_count > 0 {
            descriptor_set_layout_bindings.push(
                ash::vk::DescriptorSetLayoutBinding::default()
                    .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
                    .descriptor_count(shader_storage_block_count)
                    .stage_flags(shader_stage),
            );
        }

        // Image
        if image_count > 0 {
            let binding = ash::vk::DescriptorSetLayoutBinding::default()
                .descriptor_type(ash::vk::DescriptorType::STORAGE_IMAGE)
                .descriptor_count(image_count)
                .stage_flags(shader_stage);
            descriptor_set_layout_bindings.push(binding);
        }

        Self {
            _descriptor_set_layout_bindings: descriptor_set_layout_bindings,
        }
    }

    pub fn get_descriptor_set_layout_bindings(&self) -> &[ash::vk::DescriptorSetLayoutBinding] {
        &self._descriptor_set_layout_bindings
    }
}
