use ash::version::DeviceV1_0;

use crate::gfx::Device;

pub struct ClearColorCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _image: ash::vk::Image,
    _red: f32,
    _green: f32,
    _blue: f32,
    _alpha: f32,
}

impl<'a> ClearColorCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        image: ash::vk::Image,
        red: f32,
        green: f32,
        blue: f32,
        alpha: f32,
    ) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
            _image: image,
            _red: red,
            _green: green,
            _blue: blue,
            _alpha: alpha,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let clear_color_value = ash::vk::ClearColorValue {
            float32: [0.0, 0.0, 1.0, 1.0],
        };
        unsafe {
            device_ash.cmd_clear_color_image(
                self._command_buffer,
                self._image,
                ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
                &clear_color_value,
                &[ash::vk::ImageSubresourceRange {
                    aspect_mask: ash::vk::ImageAspectFlags::COLOR,
                    base_mip_level: 0,
                    level_count: 1,
                    base_array_layer: 0,
                    layer_count: 1,
                }],
            );
        }
    }
}
