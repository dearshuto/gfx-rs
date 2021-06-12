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
            float32: [self._red, self._green, self._blue, self._alpha],
        };
        let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
            .src_access_mask(ash::vk::AccessFlags::MEMORY_READ | ash::vk::AccessFlags::MEMORY_WRITE)
            .dst_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE)
            .old_layout(ash::vk::ImageLayout::UNDEFINED)
            .new_layout(ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL)
            .src_queue_family_index(0)
            .dst_queue_family_index(0)
            .image(self._image)
            .subresource_range(
                ash::vk::ImageSubresourceRange::builder()
                    .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1)
                    .build(),
            )
            .build();

        unsafe {
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::DependencyFlags::empty(),
                &[], // memory_barrier
                &[], // buffer_memory_barriier
                &[image_memory_bariier],
            );

            device_ash.cmd_clear_color_image(
                self._command_buffer,
                self._image,
                ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL,
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

        unsafe {
            let image_memory_bariier = ash::vk::ImageMemoryBarrier::builder()
                .src_access_mask(ash::vk::AccessFlags::TRANSFER_WRITE)
                .dst_access_mask(ash::vk::AccessFlags::TRANSFER_READ)
                .old_layout(ash::vk::ImageLayout::TRANSFER_DST_OPTIMAL)
                .new_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                .src_queue_family_index(0)
                .dst_queue_family_index(0)
                .image(self._image)
                .subresource_range(
                    ash::vk::ImageSubresourceRange::builder()
                        .aspect_mask(ash::vk::ImageAspectFlags::COLOR)
                        .base_mip_level(0)
                        .level_count(1)
                        .base_array_layer(0)
                        .layer_count(1)
                        .build(),
                )
                .build();
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::PipelineStageFlags::TRANSFER,
                ash::vk::DependencyFlags::empty(),
                &[], // memory_barrier
                &[], // buffer_memory_barriier
                &[image_memory_bariier],
            );
        }
    }
}
