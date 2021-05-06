pub struct CommandBufferWriteDescriptorSetBuilder {
    _write_descriptor_set: Vec<ash::vk::WriteDescriptorSet>,
    _descriptor_buffer_info: Vec<ash::vk::DescriptorBufferInfo>,
    _write_descriptor_set_types: Vec<WriteDescriptorSetType>,
}

impl CommandBufferWriteDescriptorSetBuilder {
    pub fn new() -> Self {
        Self {
            _write_descriptor_set: Vec::new(),
            _descriptor_buffer_info: Vec::new(),
            _write_descriptor_set_types: Vec::new(),
        }
    }

    pub fn push(&mut self, buffer: ash::vk::Buffer, offset: i64, size: u64) {
        let descriptor_buffer_info = ash::vk::DescriptorBufferInfo::builder()
            .buffer(buffer)
            .offset(offset as ash::vk::DeviceSize)
            .range(size)
            .build();

        let buffer_view = WriteDescriptorSetType::BufferView(descriptor_buffer_info);
        self._write_descriptor_set_types.push(buffer_view);
    }

    pub fn build(
        &self,
        descriptor_set: &ash::vk::DescriptorSet,
    ) -> Vec<ash::vk::WriteDescriptorSet> {
        let mut result: Vec<ash::vk::WriteDescriptorSet> = Vec::new();
        for item in &self._write_descriptor_set_types {
            let write_descriptor_set = match item {
                WriteDescriptorSetType::BufferView(descriptor_buffer_info) => {
                    self.build_buffer(descriptor_set, &descriptor_buffer_info)
                } //				WriteDescriptorSetType::TextureView => ash::vk::WriteDescriptorSet::builder().build(),
            };

            result.push(write_descriptor_set);
        }

        result
    }

    fn build_buffer(
        &self,
        descriptor_set: &ash::vk::DescriptorSet,
        descriptor_buffer_info: &ash::vk::DescriptorBufferInfo,
    ) -> ash::vk::WriteDescriptorSet {
        ash::vk::WriteDescriptorSet {
            dst_set: *descriptor_set,
            descriptor_count: 1,
            descriptor_type: ash::vk::DescriptorType::STORAGE_BUFFER,
            p_buffer_info: descriptor_buffer_info,
            ..Default::default()
        }

        // ash::vk::WriteDescriptorSet::builder()
        //     .dst_set(*descriptor_set)
        //     .descriptor_type(ash::vk::DescriptorType::STORAGE_BUFFER)
        //     .buffer_info(&[descriptor_buffer_info])
        //     .build()
    }
}

enum WriteDescriptorSetType {
    BufferView(ash::vk::DescriptorBufferInfo),
    //	TextureView,
}
