use crate::gfx::{Device, ViewportScissorState};
use ash::version::DeviceV1_0;

pub struct SetViewportScissorStateCommandBuilder<'a> {
    _device: &'a Device,
    _viewport_scissor_state: &'a ViewportScissorState<'a>,
    _command_buffer: ash::vk::CommandBuffer,
}

impl<'a> SetViewportScissorStateCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        viewport_scissor_state: &'a ViewportScissorState,
        command_buffer: ash::vk::CommandBuffer,
    ) -> Self {
        Self {
            _device: device,
            _viewport_scissor_state: viewport_scissor_state,
            _command_buffer: command_buffer,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let viewports = self._viewport_scissor_state.to_data().get_viewports();
        let scissors = self._viewport_scissor_state.to_data().get_scissor_state();

        unsafe {
            device_ash.cmd_set_viewport(self._command_buffer, 0 /*first_viewport*/, viewports);
            device_ash.cmd_set_scissor(self._command_buffer, 0, scissors);
        }
    }
}
