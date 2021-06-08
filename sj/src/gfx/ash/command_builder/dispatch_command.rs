use crate::gfx::Device;
use ash::version::DeviceV1_0;

pub struct DispatchParams<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _pipeline_layout: ash::vk::PipelineLayout,
    _descriptor_set: ash::vk::DescriptorSet,
    _group_count_x: u32,
    _group_count_y: u32,
    _group_count_z: u32,
}

impl<'a> DispatchParams<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_layout: ash::vk::PipelineLayout,
        descriptor_set: ash::vk::DescriptorSet,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _pipeline_layout: pipeline_layout,
            _descriptor_set: descriptor_set,
            _group_count_x: group_count_x,
            _group_count_y: group_count_y,
            _group_count_z: group_count_z,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.cmd_bind_descriptor_sets(
                self._command_buffer,
                ash::vk::PipelineBindPoint::COMPUTE,
                self._pipeline_layout,
                0, /*first_point*/
                &[self._descriptor_set],
                &[], /*dynamic_offset*/
            );

            device_ash.cmd_dispatch(
                self._command_buffer,
                self._group_count_x,
                self._group_count_y,
                self._group_count_z,
            );
        }
    }
}
