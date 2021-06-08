use ash::version::DeviceV1_0;

use crate::gfx::Device;

pub struct ClearColorCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _red: f32,
    _green: f32,
    _blue: f32,
    _alpha: f32,
}

impl<'a> ClearColorCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _red: red,
            _green: green,
            _blue: blue,
            _alpha: alpha,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let render_pass_begin_info = ash::vk::RenderPassBeginInfo::builder()
            //.render_pass()
            .build();
        unsafe {
            device_ash.cmd_begin_render_pass(
                self._command_buffer,
                &&render_pass_begin_info,
                ash::vk::SubpassContents::INLINE,
            );
        }
    }
}
