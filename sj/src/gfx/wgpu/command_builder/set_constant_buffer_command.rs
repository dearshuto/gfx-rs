use crate::gfx::wgpu::command_buffer_wgpu::ICommand;

pub struct SetConstantBufferCommand {}

impl ICommand for SetConstantBufferCommand {
    fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {}
}
