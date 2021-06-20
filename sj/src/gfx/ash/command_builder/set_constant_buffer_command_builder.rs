use ash::version::DeviceV1_0;

use crate::gfx::{Device, GpuAddress};

pub struct SetConstantBufferCommandBuilder<'a> {
    _device: &'a Device,
    _descriptor_set: ash::vk::DescriptorSet,
    _descriptor_buffer_info: ash::vk::DescriptorBufferInfo,
}

impl<'a> SetConstantBufferCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        descriptor_set: ash::vk::DescriptorSet,
        gpu_address: &GpuAddress,
        size: usize,
    ) -> Self {
        let buffer_ash = gpu_address.to_data().get_buffer().get_buffer();
        let offset = gpu_address.to_data().get_offset();
        let descriptor_buffer_info = ash::vk::DescriptorBufferInfo::builder()
            .buffer(buffer_ash)
            .offset(offset as ash::vk::DeviceSize)
            .range(size as ash::vk::DeviceSize)
            .build();

        Self {
            _device: device,
            _descriptor_set: descriptor_set,
            _descriptor_buffer_info: descriptor_buffer_info,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        // let write_descriptor_set = ash::vk::WriteDescriptorSet::builder()
        //     .dst_set(self._descriptor_set)
        //     .descriptor_type(ash::vk::DescriptorType::UNIFORM_BUFFER)
        //     .buffer_info(&[self._descriptor_buffer_info])
        //     //.dst_binding(0)
        //     .build();
        let write_descriptor_set = ash::vk::WriteDescriptorSet {
            dst_set: self._descriptor_set,
            descriptor_count: 1,
            descriptor_type: ash::vk::DescriptorType::UNIFORM_BUFFER,
            p_buffer_info: &self._descriptor_buffer_info,
            dst_binding: 0,
            ..Default::default()
        };

        unsafe {
            device_ash.update_descriptor_sets(&[write_descriptor_set], &[]);
        }
    }
}
