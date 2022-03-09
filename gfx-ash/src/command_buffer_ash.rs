use ash::vk::{Extent2D, Framebuffer, Rect2D};
use sjgfx_interface::{CommandBufferInfo, PrimitiveTopology};

use crate::{ColorTargetViewAsh, DeviceAsh, ShaderAsh};

pub struct CommandBufferAsh {
    #[allow(dead_code)]
    device: ash::Device,
    #[allow(dead_code)]
    command_pool: ash::vk::CommandPool,
    command_buffer: ash::vk::CommandBuffer,

    // レンダーターゲット
    image_view: Option<ash::vk::ImageView>,
    format: Option<ash::vk::Format>,
    render_pass: Option<ash::vk::RenderPass>,
    #[allow(dead_code)]
    framebuffer: Option<Framebuffer>,

    // シェーダ
    compute_shader_module: Option<ash::vk::ShaderModule>,
    vertex_shader_module: Option<ash::vk::ShaderModule>,
    pixel_shader_module: Option<ash::vk::ShaderModule>,
    pipeline: Option<ash::vk::Pipeline>,
    pipeline_layout: Option<ash::vk::PipelineLayout>,

    // 描画コマンド
    vertex_count: Option<u32>,
}

impl CommandBufferAsh {
    pub fn new(device: &DeviceAsh, _info: &CommandBufferInfo) -> Self {
        let queue_family_index = device.get_queue_family_index();
        let device = device.get_device();

        let command_pool_create_info = ash::vk::CommandPoolCreateInfo::builder()
            .flags(ash::vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
            .queue_family_index(queue_family_index)
            .build();
        let command_pool =
            unsafe { device.create_command_pool(&command_pool_create_info, None) }.unwrap();

        let command_buffer_allocate_create_info = ash::vk::CommandBufferAllocateInfo::builder()
            .command_buffer_count(1)
            .command_pool(command_pool)
            .level(ash::vk::CommandBufferLevel::PRIMARY);
        let command_buffers =
            unsafe { device.allocate_command_buffers(&command_buffer_allocate_create_info) }
                .unwrap();

        Self {
            device,
            command_pool,
            command_buffer: command_buffers[0],

            // レンダーターゲット
            image_view: None,
            format: None,
            render_pass: None,
            framebuffer: None,

            compute_shader_module: None,
            vertex_shader_module: None,
            pixel_shader_module: None,
            pipeline: None,
            pipeline_layout: None,

            // 描画コマンド
            vertex_count: None,
        }
    }

    pub fn begin(&mut self) {
        let command_buffer_begin_info = ash::vk::CommandBufferBeginInfo::builder()
            .flags(ash::vk::CommandBufferUsageFlags::empty())
            .build();

        unsafe {
            self.device
                .begin_command_buffer(self.command_buffer, &command_buffer_begin_info)
        }
        .unwrap();
    }

    pub fn end(&mut self) {
        if self.compute_shader_module.is_some() {
            self.push_compute_pass_command();
        } else if self.vertex_shader_module.is_some() && self.pixel_shader_module.is_some() {
            self.push_graphics_pass_command();
        }

        // コマンドを終了
        unsafe { self.device.end_command_buffer(self.command_buffer) }.unwrap();
    }

    pub fn set_render_targets<T>(&mut self, mut color_targets: T, _depth_stencil_view: Option<()>)
    where
        T: Iterator<Item = ColorTargetViewAsh>,
    {
        if let Some(color_target) = color_targets.next() {
            self.image_view = Some(color_target.get_image_view());
            self.format = Some(color_target.get_format());
        }
    }

    pub fn set_shader(&mut self, shader: &ShaderAsh) {
        // シェーダをもとにパイプラインを作る必要があるのでコマンドの作成は end() が呼ばれるまで遅延する
        if shader.is_compute() {
            self.compute_shader_module = Some(shader.get_compute_shader_module());
            self.vertex_shader_module = None;
            self.pixel_shader_module = None;
        } else {
            self.compute_shader_module = None;
            self.vertex_shader_module = Some(shader.get_vertex_shader_module());
            self.pixel_shader_module = Some(shader.get_pixel_shader_module());
        }

        self.pipeline_layout = Some(shader.get_pipeline_layout());
    }

    pub fn draw(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        _vertex_offset: i32,
    ) {
        self.vertex_count = Some(vertex_count as u32);
    }

    pub fn get_command_buffer(&self) -> ash::vk::CommandBuffer {
        self.command_buffer
    }

    fn push_compute_pass_command(&mut self) {
        self.update_pipeline();

        // ディスパッチコマンド
        if false {
            unsafe {
                self.device.cmd_dispatch(self.command_buffer, 1, 1, 1);
            }
        }
    }

    fn push_graphics_pass_command(&mut self) {
        let render_area = ash::vk::Rect2D::builder()
            .extent(ash::vk::Extent2D::builder().width(640).height(480).build())
            .build();

        // レンダーパス
        // TODO: キャッシュ
        let renderpass_attachments = [ash::vk::AttachmentDescription {
            format: self.format.unwrap(),
            samples: ash::vk::SampleCountFlags::TYPE_1,
            load_op: ash::vk::AttachmentLoadOp::CLEAR,
            store_op: ash::vk::AttachmentStoreOp::STORE,
            final_layout: ash::vk::ImageLayout::PRESENT_SRC_KHR,
            ..Default::default()
        }];
        let color_attachment_refs = [ash::vk::AttachmentReference {
            attachment: 0,
            layout: ash::vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        }];
        let dependencies = [ash::vk::SubpassDependency {
            src_subpass: ash::vk::SUBPASS_EXTERNAL,
            src_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_access_mask: ash::vk::AccessFlags::COLOR_ATTACHMENT_READ
                | ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dst_stage_mask: ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            ..Default::default()
        }];
        let subpass = ash::vk::SubpassDescription::builder()
            .color_attachments(&color_attachment_refs)
            .pipeline_bind_point(ash::vk::PipelineBindPoint::GRAPHICS);
        let renderpass_create_info = ash::vk::RenderPassCreateInfo::builder()
            .attachments(&renderpass_attachments)
            .subpasses(std::slice::from_ref(&subpass))
            .dependencies(&dependencies);

        let renderpass = unsafe {
            self.device
                .create_render_pass(&renderpass_create_info, None)
        }
        .unwrap();
        self.render_pass = Some(renderpass);

        // レンダーパスを更新した後じゃないとダメ
        self.update_pipeline();

        // フレームバッファ
        let frame_buffer_attachment = [self.image_view.unwrap()];
        let frame_buffer_create_info = ash::vk::FramebufferCreateInfo::builder()
            .render_pass(renderpass)
            .attachments(&frame_buffer_attachment)
            .width(640)
            .height(480)
            .layers(1);
        let frame_buffer = unsafe {
            self.device
                .create_framebuffer(&frame_buffer_create_info, None)
        }
        .unwrap();
        self.framebuffer = Some(frame_buffer);

        let clear_values = [ash::vk::ClearValue {
            color: ash::vk::ClearColorValue {
                float32: [0.2, 0.0, 0.3, 0.0],
            },
        }];

        // レンダーパス
        let render_pass_begin_info = ash::vk::RenderPassBeginInfo::builder()
            .render_pass(renderpass)
            .framebuffer(frame_buffer)
            .render_area(render_area.clone())
            .clear_values(&clear_values);
        unsafe {
            self.device.cmd_begin_render_pass(
                self.command_buffer,
                &render_pass_begin_info,
                ash::vk::SubpassContents::INLINE,
            );
        }

        // パイプライン
        unsafe {
            self.device.cmd_bind_pipeline(
                self.command_buffer,
                ash::vk::PipelineBindPoint::GRAPHICS,
                self.pipeline.unwrap(),
            )
        };

        // ビューポートシザー
        // TODO: 専用のクラスとセットコマンドを用意する
        let viewports = [ash::vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let scissors = [render_area];
        unsafe {
            self.device
                .cmd_set_viewport(self.command_buffer, 0, &viewports);
        }
        unsafe {
            self.device
                .cmd_set_scissor(self.command_buffer, 0, &scissors);
        }

        // 描画コマンド
        if let Some(vertex_count) = self.vertex_count {
            unsafe {
                self.device.cmd_draw(
                    self.command_buffer,
                    vertex_count,
                    1, /*instance_count*/
                    0, /*first_vertex*/
                    0, /*first_instance*/
                )
            };
        }

        // 描画パス終わり
        unsafe {
            self.device.cmd_end_render_pass(self.command_buffer);
        }
    }

    fn update_pipeline(&mut self) {
        let mut new_pipeline = if self.compute_shader_module.is_some() {
            Some(self.create_compute_pipeline())
        } else if self.vertex_shader_module.is_some() {
            Some(self.create_graphics_pipeline())
        } else {
            None
        };

        std::mem::swap(&mut new_pipeline, &mut self.pipeline);
        if let Some(old_pipeline) = new_pipeline {
            unsafe {
                self.device.destroy_pipeline(old_pipeline, None);
            }
        }
    }

    fn create_compute_pipeline(&self) -> ash::vk::Pipeline {
        let compute_pipeline_create_info = ash::vk::ComputePipelineCreateInfo::builder().build();
        let compute_pipeline = unsafe {
            self.device.create_compute_pipelines(
                ash::vk::PipelineCache::null(),
                &[compute_pipeline_create_info],
                None,
            )
        }
        .unwrap();
        compute_pipeline[0]
    }

    fn create_graphics_pipeline(&self) -> ash::vk::Pipeline {
        let shader_entry_name = unsafe { std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0") };
        let shader_stage_create_infos = [
            ash::vk::PipelineShaderStageCreateInfo {
                module: self.vertex_shader_module.unwrap(),
                p_name: shader_entry_name.as_ptr(),
                stage: ash::vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            },
            ash::vk::PipelineShaderStageCreateInfo {
                s_type: ash::vk::StructureType::PIPELINE_SHADER_STAGE_CREATE_INFO,
                module: self.pixel_shader_module.unwrap(),
                p_name: shader_entry_name.as_ptr(),
                stage: ash::vk::ShaderStageFlags::FRAGMENT,
                ..Default::default()
            },
        ];
        let input_assembly_state_info = ash::vk::PipelineInputAssemblyStateCreateInfo {
            topology: ash::vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };
        let viewports = [ash::vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: 640.0,
            height: 480.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }];
        let rasterization_info = ash::vk::PipelineRasterizationStateCreateInfo {
            front_face: ash::vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: ash::vk::PolygonMode::FILL,
            ..Default::default()
        };
        let multisample_state_info = ash::vk::PipelineMultisampleStateCreateInfo {
            rasterization_samples: ash::vk::SampleCountFlags::TYPE_1,
            ..Default::default()
        };
        let noop_stencil_state = ash::vk::StencilOpState {
            fail_op: ash::vk::StencilOp::KEEP,
            pass_op: ash::vk::StencilOp::KEEP,
            depth_fail_op: ash::vk::StencilOp::KEEP,
            compare_op: ash::vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        let depth_state_info = ash::vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: ash::vk::CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };

        let color_blend_attachment_states = [ash::vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: ash::vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: ash::vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: ash::vk::BlendOp::ADD,
            src_alpha_blend_factor: ash::vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: ash::vk::BlendFactor::ZERO,
            alpha_blend_op: ash::vk::BlendOp::ADD,
            color_write_mask: ash::vk::ColorComponentFlags::RGBA,
        }];
        let color_blend_state = ash::vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(ash::vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [
            ash::vk::DynamicState::VIEWPORT,
            ash::vk::DynamicState::SCISSOR,
        ];
        let dynamic_state_info =
            ash::vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

        let scissors = [Rect2D::builder()
            .extent(Extent2D {
                width: 640,
                height: 480,
            })
            .build()];
        let vertex_input_state_info = ash::vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&[])
            .vertex_binding_descriptions(&[]);
        let viewport_state_info = ash::vk::PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports);
        let graphics_pipeline_create_info = ash::vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(self.pipeline_layout.unwrap())
            .render_pass(self.render_pass.unwrap())
            .build();
        let graphics_pipeline = unsafe {
            self.device.create_graphics_pipelines(
                ash::vk::PipelineCache::null(),
                &[graphics_pipeline_create_info],
                None,
            )
        }
        .unwrap();
        graphics_pipeline[0]
    }
}

impl Drop for CommandBufferAsh {
    fn drop(&mut self) {
        // unsafe{ self.device.destroy_descriptor_pool(pool, None) };

        // レンダーパス
        if let Some(render_pass) = self.render_pass {
            unsafe { self.device.destroy_render_pass(render_pass, None) }
        }

        // フレームバッファ
        if let Some(frame_buffer) = self.framebuffer {
            unsafe { self.device.destroy_framebuffer(frame_buffer, None) }
        }

        // パイプライン
        if let Some(pipeline) = self.pipeline {
            unsafe { self.device.destroy_pipeline(pipeline, None) };
        }

        unsafe {
            self.device
                .free_command_buffers(self.command_pool, &[self.command_buffer])
        };
        unsafe { self.device.destroy_command_pool(self.command_pool, None) };
    }
}

#[cfg(test)]
mod tests {
    use sjgfx_interface::{CommandBufferInfo, DeviceInfo};

    use crate::{CommandBufferAsh, DeviceAsh};

    #[test]
    fn new() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let _command_buffer = CommandBufferAsh::new(&device, &CommandBufferInfo::new());
    }

    #[test]
    fn begin_end() {
        let device = DeviceAsh::new(&DeviceInfo::new());
        let mut command_buffer = CommandBufferAsh::new(&device, &CommandBufferInfo::new());

        command_buffer.begin();
        command_buffer.end();
    }
}
