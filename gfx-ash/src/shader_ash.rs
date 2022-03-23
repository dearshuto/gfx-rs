use std::ops::Index;

use sjgfx_interface::{GpuAccess, IShader, ShaderInfo, ShaderStage};

use crate::DeviceAsh;

struct DescriptorInfo {
    pub stage: ShaderStage,
    pub constant_buffer_count: u32,
    pub unordered_access_buffer_count: u32,
}

pub struct ShaderAsh {
    device: ash::Device,
    descriptor_set_layouts: Vec<ash::vk::DescriptorSetLayout>,
    pipeline_layout: ash::vk::PipelineLayout,
    compute_shader_module: Option<ash::vk::ShaderModule>,
    vertex_shader_module: Option<ash::vk::ShaderModule>,
    pixel_shader_module: Option<ash::vk::ShaderModule>,
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
        self.compute_shader_module.is_some()
    }

    pub fn get_compute_shader_module(&self) -> ash::vk::ShaderModule {
        self.compute_shader_module.unwrap()
    }

    pub fn get_vertex_shader_module(&self) -> ash::vk::ShaderModule {
        self.vertex_shader_module.unwrap()
    }

    pub fn get_pixel_shader_module(&self) -> ash::vk::ShaderModule {
        self.pixel_shader_module.unwrap()
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
            compute_shader_module: None,
            vertex_shader_module: Some(Self::crate_shader_module(
                device,
                info.get_vertex_shader_binary().as_ref().unwrap(),
            )),
            pixel_shader_module: Some(Self::crate_shader_module(
                device,
                info.get_pixel_shader_binary().as_ref().unwrap(),
            )),
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
            compute_shader_module: Some(Self::crate_shader_module(
                device,
                info.get_compute_shader_binary().as_ref().unwrap(),
            )),
            vertex_shader_module: None,
            pixel_shader_module: None,
            descriptor_set_layouts: vec![descriptor_set_layout],
            pipeline_layout,
            id: uuid::Uuid::new_v4(),
        }
    }

    fn crate_shader_module(device: &DeviceAsh, shader_binary: &[u8]) -> ash::vk::ShaderModule {
        let mut compute_shader_binary = std::io::Cursor::new(shader_binary);
        let shader_code = ash::util::read_spv(&mut compute_shader_binary).expect("");

        let shader_module_create_info = ash::vk::ShaderModuleCreateInfo::builder()
            .code(&shader_code)
            .build();

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

        let module = spirv_reflect::ShaderModule::load_u8_data(shader_binary).unwrap();
        for item in module.enumerate_descriptor_bindings(None).unwrap().iter() {
            match item.resource_type {
                spirv_reflect::types::ReflectResourceType::Undefined => todo!(),
                spirv_reflect::types::ReflectResourceType::Sampler => sampler_count += 1,
                spirv_reflect::types::ReflectResourceType::CombinedImageSampler => todo!(),
                spirv_reflect::types::ReflectResourceType::ConstantBufferView => {
                    constant_buffer_count += 1
                }
                spirv_reflect::types::ReflectResourceType::ShaderResourceView => todo!(),
                spirv_reflect::types::ReflectResourceType::UnorderedAccessView => {
                    unordered_access_buffer_count += 1
                }
            }
        }

        DescriptorInfo {
            stage: shader_stage,
            constant_buffer_count,
            unordered_access_buffer_count,
        }
    }

    fn create_descriptor_set_layout_and_pipeline_layout(
        device: &DeviceAsh,
        descriptor_infos: &[&DescriptorInfo],
    ) -> (ash::vk::DescriptorSetLayout, ash::vk::PipelineLayout) {
        let tables = descriptor_infos.iter().map(|x| {
            LayoutTable::new(
                Self::to_ash(x.stage.clone()),
                x.constant_buffer_count,         /*uniform_block_count*/
                x.unordered_access_buffer_count, /*shader_storage_block_count*/
                0,                               /*texture_count*/
                0,                               /*image_count*/
            )
        });

        let mut descriptor_set_layout_bindings = Vec::new();
        for table in tables {
            for binding in table.get_descriptor_set_layout_bindings() {
                descriptor_set_layout_bindings.push(*binding);
            }
        }

        let descriptor_set_layout = unsafe {
            device.get_device().create_descriptor_set_layout(
                &ash::vk::DescriptorSetLayoutCreateInfo::builder()
                    .bindings(&descriptor_set_layout_bindings)
                    .build(),
                None,
            )
        }
        .unwrap();

        let pipeline_layout = unsafe {
            device.get_device().create_pipeline_layout(
                &ash::vk::PipelineLayoutCreateInfo::builder()
                    .set_layouts(&[descriptor_set_layout])
                    .build(),
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

    fn new(device: &Self::DeviceType, info: &ShaderInfo) -> Self {
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
        if let Some(compute_shader_module) = self.compute_shader_module {
            unsafe {
                self.device
                    .destroy_shader_module(compute_shader_module, None)
            }
        }

        // 頂点シェーダの破棄
        if let Some(vertex_shader_module) = self.vertex_shader_module {
            unsafe {
                self.device
                    .destroy_shader_module(vertex_shader_module, None)
            }
        }

        // ピクセルシェーダの破棄
        if let Some(pixel_shader_module) = self.pixel_shader_module {
            unsafe { self.device.destroy_shader_module(pixel_shader_module, None) }
        }
    }
}

pub struct LayoutTable {
    _descriptor_set_layout_bindings: Vec<ash::vk::DescriptorSetLayoutBinding>,
    _indices: [Vec<u32>; 4],
}

impl LayoutTable {
    pub fn new(
        shader_stage: ash::vk::ShaderStageFlags,
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
                    .stage_flags(shader_stage)
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
                    .stage_flags(shader_stage)
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
