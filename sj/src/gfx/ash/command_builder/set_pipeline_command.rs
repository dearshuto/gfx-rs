use ash::version::DeviceV1_0;

use super::super::super::{Device, Pipeline};

pub struct SetPipelineParams<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _descriptor_pool: ash::vk::DescriptorPool,
    _descriptor_set: ash::vk::DescriptorSet,
    _pipeline: ash::vk::Pipeline,
    _render_pass: Option<ash::vk::RenderPass>,
}

impl<'a> SetPipelineParams<'a> {
    pub fn new(
        device: &'a Device,
        pipeline: &'a Pipeline,
        command_buffer: ash::vk::CommandBuffer,
        descriptor_pool: ash::vk::DescriptorPool,
        render_pass: Option<ash::vk::RenderPass>,
    ) -> Self {
        let device_ash = device.to_data().get_device();
        let layout = pipeline
            .to_data()
            .get_shader()
            .to_data()
            .get_descriptor_set_layout();

        unsafe {
            let descriptor_set = device_ash
                .allocate_descriptor_sets(
                    &ash::vk::DescriptorSetAllocateInfo::builder()
                        .set_layouts(&[*layout])
                        .descriptor_pool(descriptor_pool)
                        .build(),
                )
                .unwrap();

            if pipeline.to_data().is_graphics_pipeline() {
                let pipeline = pipeline
                    .to_data()
                    .create_graphics_pipeline(render_pass.unwrap());

                Self {
                    _device: device,
                    _command_buffer: command_buffer,
                    _descriptor_pool: descriptor_pool,
                    _descriptor_set: descriptor_set[0],
                    _pipeline: pipeline,
                    _render_pass: render_pass,
                }
            } else {
                Self {
                    _device: device,
                    _command_buffer: command_buffer,
                    _descriptor_pool: descriptor_pool,
                    _descriptor_set: descriptor_set[0],
                    _pipeline: *pipeline.to_data().get_pipeline(),
                    _render_pass: None,
                }
            }
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();

        let bind_point = if self.is_graphics() {
            ash::vk::PipelineBindPoint::GRAPHICS
        } else {
            ash::vk::PipelineBindPoint::COMPUTE
        };

        unsafe {
            device_ash.cmd_bind_pipeline(self._command_buffer, bind_point, self._pipeline);
        }
    }

    pub fn get_descriptor_set(&self) -> &ash::vk::DescriptorSet {
        &self._descriptor_set
    }

    fn is_graphics(&self) -> bool {
        self._render_pass.is_some()
    }
}

impl<'a> Drop for SetPipelineParams<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_pipeline(self._pipeline, None);
        }

        // 明示的に開放するとむしろエラーがでたのでコメントアウトしておく
        //device_ash.free_descriptor_sets(self._descriptor_pool, &[self._descriptor_set]).unwrap();
    }
}
