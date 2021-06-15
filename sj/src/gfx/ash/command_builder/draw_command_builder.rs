use super::super::super::Device;
use ash::version::DeviceV1_0;

pub struct DrawCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _pipeline_layout: ash::vk::PipelineLayout,
    _descriptor_set: ash::vk::DescriptorSet,
    _vertex_count: u32,
    _instance_count: u32,
    _first_vertex: u32,
    _first_instance: u32,
}

impl<'a> DrawCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_layout: ash::vk::PipelineLayout,
        descriptor_set: ash::vk::DescriptorSet,

        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _pipeline_layout: pipeline_layout,
            _descriptor_set: descriptor_set,
            _vertex_count: vertex_count,
            _instance_count: instance_count,
            _first_vertex: first_vertex,
            _first_instance: first_instance,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.cmd_bind_descriptor_sets(
                self._command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self._pipeline_layout,
                0,                       // first point
                &[self._descriptor_set], // descriptor set
                &[],                     // dynamic offset
            );

            device_ash.cmd_draw(
                self._command_buffer,
                self._vertex_count,
                self._instance_count,
                self._first_vertex,
                self._first_instance,
            );
        }
    }
}
