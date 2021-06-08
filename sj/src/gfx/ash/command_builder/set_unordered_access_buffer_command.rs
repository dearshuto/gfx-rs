use crate::gfx::Device;
use ash::version::DeviceV1_0;

pub struct SetUnorderedAccessBufferParams<'a> {
    _device: &'a Device,
    _descriptor_set: ash::vk::DescriptorSet,
    _descriptor_buffer_info: ash::vk::DescriptorBufferInfo,
}

impl<'a> SetUnorderedAccessBufferParams<'a> {
    pub fn new(
        device: &'a Device,
        descriptor_set: ash::vk::DescriptorSet,
        buffer: ash::vk::Buffer,
        _slot: i32,
        offset: u64,
        size: u64,
    ) -> Self {
        let descriptor_buffer_info = ash::vk::DescriptorBufferInfo::builder()
            .buffer(buffer)
            .offset(offset)
            .range(size)
            .build();

        Self {
            _device: device,
            _descriptor_set: descriptor_set,
            _descriptor_buffer_info: descriptor_buffer_info,
        }
    }

    pub fn build(&self) {
        let write_descriptor_set = ash::vk::WriteDescriptorSet {
            dst_set: self._descriptor_set,
            descriptor_count: 1,
            descriptor_type: ash::vk::DescriptorType::STORAGE_BUFFER,
            p_buffer_info: &self._descriptor_buffer_info,
            ..Default::default()
        };
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.update_descriptor_sets(&[write_descriptor_set], &[]);
        }
    }
}
