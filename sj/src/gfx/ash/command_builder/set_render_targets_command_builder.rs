use crate::gfx::{ColorTargetView, DepthStencilView, Device};
use ash::version::DeviceV1_0;

pub struct SetRenderTargetsCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _render_pass: ash::vk::RenderPass,
    _frame_buffer: ash::vk::Framebuffer,
}

impl<'a> SetRenderTargetsCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) -> Self {
        let device_ash = device.to_data().get_device();
        let mut render_pass_attatchments: Vec<ash::vk::AttachmentDescription> = color_target_views
            .iter()
            .map(|info| info.to_attachment_description())
            .collect();
        if let Some(_value) = depth_stencil_state_view {
            render_pass_attatchments.push(
                ash::vk::AttachmentDescription::builder()
                    .format(ash::vk::Format::D32_SFLOAT)
                    .samples(ash::vk::SampleCountFlags::TYPE_1)
                    .load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
                    .store_op(ash::vk::AttachmentStoreOp::STORE)
                    .initial_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .final_layout(ash::vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL)
                    .build(),
            );
        }

        let color_attachment_references = [ash::vk::AttachmentReference::builder()
            .attachment(0)
            .layout(ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
            .build()];
        let depth_attachment_reference = ash::vk::AttachmentReference::builder()
            .attachment(1) // TODO: Multiple Render Target
            .layout(ash::vk::ImageLayout::DEPTH_STENCIL_ATTACHMENT_OPTIMAL)
            .build();

        let dependencies = [ash::vk::SubpassDependency {
            src_subpass: ash::vk::SUBPASS_EXTERNAL,
            src_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: ash::vk::AccessFlags::COLOR_ATTACHMENT_READ
                | ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];

        let mut subpass_description_builder = ash::vk::SubpassDescription::builder()
            .color_attachments(&color_attachment_references)
            .pipeline_bind_point(ash::vk::PipelineBindPoint::GRAPHICS);
        if depth_stencil_state_view.is_some() {
            subpass_description_builder =
                subpass_description_builder.depth_stencil_attachment(&depth_attachment_reference);
        }
        let subpasses = [subpass_description_builder.build()];

        let render_pass_create_info = ash::vk::RenderPassCreateInfo::builder()
            .attachments(&render_pass_attatchments)
            .subpasses(&subpasses)
            .dependencies(&dependencies)
            .build();

        let mut framebuffer_attachments: Vec<ash::vk::ImageView> = color_target_views
            .iter()
            .map(|x| *x.to_data().get_image_view())
            .collect();
        if let Some(depth_stencil_view_ref) = depth_stencil_state_view {
            let depth_image_view = depth_stencil_view_ref.to_data().get_image_view();
            framebuffer_attachments.push(depth_image_view);
        }

        unsafe {
            let render_pass = device_ash
                .create_render_pass(&render_pass_create_info, None)
                .unwrap();

            let color_target_view_impl = color_target_views.iter().next().unwrap().to_data();
            let width = color_target_view_impl.get_width() as u32;
            let height = color_target_view_impl.get_height() as u32;
            let framebuffer_create_info = ash::vk::FramebufferCreateInfo::builder()
                .render_pass(render_pass)
                .attachments(&framebuffer_attachments)
                .width(width)
                .height(height)
                .layers(1)
                .build();
            let framebuffer = device_ash
                .create_framebuffer(&framebuffer_create_info, None)
                .unwrap();

            Self {
                _device: device,
                _command_buffer: command_buffer,
                _render_pass: render_pass,
                _frame_buffer: framebuffer,
            }
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let render_pass_begin_info = ash::vk::RenderPassBeginInfo::builder()
            .render_pass(self._render_pass)
            .framebuffer(self._frame_buffer)
            .clear_values(&[])
            .render_area(ash::vk::Rect2D {
                offset: ash::vk::Offset2D { x: 0, y: 0 },
                extent: ash::vk::Extent2D::builder().width(640).height(480).build(),
            })
            .build();

        unsafe {
            device_ash.cmd_begin_render_pass(
                self._command_buffer,
                &render_pass_begin_info,
                ash::vk::SubpassContents::INLINE,
            );
        }
    }

    pub fn get_render_pass(&self) -> &ash::vk::RenderPass {
        &self._render_pass
    }
}

impl<'a> Drop for SetRenderTargetsCommandBuilder<'a> {
    fn drop(&mut self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.destroy_render_pass(self._render_pass, None);
            device_ash.destroy_framebuffer(self._frame_buffer, None);
        }
    }
}

pub struct EndRenderPassCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
}

impl<'a> EndRenderPassCommandBuilder<'a> {
    pub fn new(device: &'a Device, command_buffer: ash::vk::CommandBuffer) -> Self {
        Self {
            _device: device,
            _command_buffer: command_buffer,
        }
    }

    pub fn buld(&self) {
        let device_ash = self._device.to_data().get_device();

        unsafe {
            device_ash.cmd_end_render_pass(self._command_buffer);
        }
    }
}

impl<'a> ColorTargetView<'a> {
    pub fn to_attachment_description(&self) -> ash::vk::AttachmentDescription {
        let format = self.to_data().get_format();

        ash::vk::AttachmentDescription::builder()
            .format(format)
            .samples(ash::vk::SampleCountFlags::TYPE_1)
            .load_op(ash::vk::AttachmentLoadOp::DONT_CARE)
            .store_op(ash::vk::AttachmentStoreOp::STORE)
            .final_layout(ash::vk::ImageLayout::PRESENT_SRC_KHR)
            .build()
    }
}
