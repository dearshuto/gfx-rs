use ash::version::DeviceV1_0;

use super::super::super::Device;

pub struct SetVertexBufferCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _vertex_buffer: ash::vk::Buffer,
}

impl<'a> SetVertexBufferCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        vertex_buffer: ash::vk::Buffer,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _vertex_buffer: vertex_buffer,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.cmd_bind_vertex_buffers(
                self._command_buffer,
                0, // first bindings
                &[self._vertex_buffer],
                &[0], // offset
            );
        }
    }
}
