use super::super::command_buffer_api::{CommandBufferInfo, ICommandBufferImpl};
use super::super::Device;

pub struct CommandBuffer<'a> {
    device: &'a wgpu::Device,
    command_encoder: Option<wgpu::CommandEncoder>,
}

impl<'a> ICommandBufferImpl<'a> for CommandBuffer<'a> {
    fn new(device: &'a Device, info: &CommandBufferInfo) -> Self {
        CommandBuffer {
            device: device.to_data().get_device(),
            command_encoder: None,
        }
    }
}

impl<'a> CommandBuffer<'a> {
    pub fn begin(&mut self) {
        let command_encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        self.command_encoder = Some(command_encoder);
    }
	

    pub fn end(&mut self) {
        self.command_encoder = None;
    }

    pub fn get_command_buffer(&'a mut self) -> wgpu::CommandBuffer {
        let command_encoder = std::mem::replace(&mut self.command_encoder, None);
        command_encoder.unwrap().finish()
    }
}
