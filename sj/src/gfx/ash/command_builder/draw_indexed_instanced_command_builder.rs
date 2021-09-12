use ash::version::DeviceV1_0;

use crate::gfx::{Device, GpuAddress, IndexFormat, PrimitiveTopology};

pub struct DrawIndexedInstancedCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _pipeline_layout: ash::vk::PipelineLayout,
    _descriptor_set: ash::vk::DescriptorSet,
    _index_buffer: ash::vk::Buffer,
    _index_type: ash::vk::IndexType,
    _offset: ash::vk::DeviceSize,
    _index_count: u32,
    _base_vertex: i32,
    _instance_count: u32,
    _base_instance: u32,
}

impl<'a> DrawIndexedInstancedCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        pipeline_layout: ash::vk::PipelineLayout,
        descriptor_set: ash::vk::DescriptorSet,
        _primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _pipeline_layout: pipeline_layout,
            _descriptor_set: descriptor_set,
            _index_buffer: gpu_address.to_data().get_buffer().get_buffer(),
            _index_type: index_format.to_ash(),
            _offset: gpu_address.to_data().get_offset() as u64,
            _index_count: index_count as u32,
            _base_vertex: base_vertex,
            _instance_count: instance_count as u32,
            _base_instance: base_instance as u32,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.cmd_bind_index_buffer(
                self._command_buffer,
                self._index_buffer,
                self._offset,
                self._index_type,
            );

            device_ash.cmd_bind_descriptor_sets(
                self._command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self._pipeline_layout,
                0,                       // first point
                &[self._descriptor_set], // descriptor set
                &[],                     // dynamic offset
            );
            device_ash.cmd_draw_indexed(
                self._command_buffer,
                self._index_count,
                self._instance_count,
                0,
                self._base_vertex,
                self._base_instance,
            );
        }
    }
}

impl IndexFormat {
    pub fn to_ash(&self) -> ash::vk::IndexType {
        match self {
            &IndexFormat::Uint32 => ash::vk::IndexType::UINT32,
        }
    }
}
