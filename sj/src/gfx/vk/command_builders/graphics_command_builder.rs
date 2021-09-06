use vulkano::{
    command_buffer::AutoCommandBufferBuilder,
    image::{view::ImageView, AttachmentImage, ImageUsage},
    pipeline::viewport::{Scissor, Viewport},
    render_pass::{Framebuffer, FramebufferAbstract, RenderPass},
};

use crate::gfx::{
    common::command_builder::IGraphicsCommandBuilder, DepthStencilStateInfo, Pipeline,
    PrimitiveTopology, RasterizerStateInfo, ScissorStateInfo, ViewportStateInfo,
};

pub struct GraphicsCommandBuilder<'a> {
    _device: std::sync::Arc<vulkano::device::Device>,
    _rasterizer_state_info: RasterizerStateInfo,
    _depth_stencil_state_info: DepthStencilStateInfo,
    _viewport_state_info_array: Option<Vec<ViewportStateInfo>>,
    _scissor_state_info_array: Option<Vec<ScissorStateInfo>>,
    _primitive_topology: Option<PrimitiveTopology>,
    _render_pass: Option<std::sync::Arc<RenderPass>>,
    _frame_buffer: Option<std::sync::Arc<dyn FramebufferAbstract>>,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a> GraphicsCommandBuilder<'a> {
    pub fn new(
        device: std::sync::Arc<vulkano::device::Device>,
        pipeline: &'a Pipeline<'a>,
    ) -> GraphicsCommandBuilder {
        assert!(pipeline.to_data().is_graphics());

        Self {
            _device: device,
            _rasterizer_state_info: *pipeline.to_data().get_rasterizer_state_info(),
            _depth_stencil_state_info: *pipeline.to_data().get_depth_stencil_state_info(),
            _viewport_state_info_array: None,
            _scissor_state_info_array: None,
            _primitive_topology: None,
            _render_pass: None,
            _frame_buffer: None,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build<L, P>(
        &self,
        command_builder: AutoCommandBufferBuilder<L, P>,
    ) -> AutoCommandBufferBuilder<L, P> {
        let _graphics_pipeline_builder = vulkano::pipeline::GraphicsPipeline::start()
            .viewports_dynamic_scissors_irrelevant(1)
            .cull_mode_front()
            .depth_write(true)
            //.vertex_shader()
            .depth_stencil_disabled();

        let _graphics_command_builder_wrap =
            GraphicsPipelineBuilder(vulkano::pipeline::GraphicsPipeline::start())
                .push_rasterizer_state(&self._rasterizer_state_info)
                .push_depth_stencil_state(&self._depth_stencil_state_info)
                .push_primitive_topology(&self._primitive_topology.as_ref().unwrap())
                .push_viewport_scissors(
                    self._viewport_state_info_array.as_ref().unwrap(),
                    self._scissor_state_info_array.as_ref().unwrap(),
                );

        // let render_pass = std::sync::Arc::new(
        //     vulkano::single_pass_renderpass!(
        //         device_vk.clone(),
        //         attachments: {
        //             color: {
        //                 load: Clear,
        //                 store: Store,
        //                 format: vulkano::format::Format::R8Unorm, // TODO
        //                 samples: 1,
        //             }
        //         },
        //         pass: {
        //             color: [color],
        //             depth_stencil: {}
        //         }
        //     )
        //     .unwrap(),
        // );
        command_builder
    }
}

impl<'a> IGraphicsCommandBuilder<'a> for GraphicsCommandBuilder<'a> {
    fn build(&self) {
        todo!()
    }

    fn set_viewport_scissor_state(
        &mut self,
        viewport_scissor_state: &'a crate::gfx::ViewportScissorState,
    ) {
        self._viewport_state_info_array = Some(
            viewport_scissor_state
                .to_data()
                .get_viewport_state_info_array()
                .to_vec(),
        );
        self._scissor_state_info_array = Some(
            viewport_scissor_state
                .to_data()
                .get_scissor_state_info_array()
                .to_vec(),
        );
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    ) {
        todo!()
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        todo!()
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&crate::gfx::ColorTargetView],
        _depth_stencil_state_view: Option<&crate::gfx::DepthStencilView>,
    ) {
        let render_pass = std::sync::Arc::new(
            vulkano::single_pass_renderpass!(
                self._device.clone(),
                attachments: {
                    // `color` is a custom name we give to the first and only attachment.
                    color: {
                        // `load: Clear` means that we ask the GPU to clear the content of this
                        // attachment at the start of the drawing.
                        load: Clear,
                        // `store: Store` means that we ask the GPU to store the output of the draw
                        // in the actual image. We could also ask it to discard the result.
                        store: Store,
                        // `format: <ty>` indicates the type of the format of the image. This has to
                        // be one of the types of the `vulkano::format` module (or alternatively one
                        // of your structs that implements the `FormatDesc` trait). Here we use the
                        // same format as the swapchain.
                        format: vulkano::format::Format::R8G8B8A8Unorm,
                        // TODO:
                        samples: 1,
                    }
                },
                pass: {
                    color: [color],
                    depth_stencil: {}
                }
            )
            .unwrap(),
        );

        let image = color_target_views[0].to_data().clone_image();
        let dimension = image.dimensions();
        let image_view = ImageView::new(
            AttachmentImage::with_usage(
                self._device.clone(),
                [dimension.width(), dimension.height()],
                vulkano::format::Format::R8G8B8A8Unorm,
                ImageUsage {
                    transient_attachment: true,
                    input_attachment: true,
                    ..ImageUsage::none()
                },
            )
            .unwrap(),
        )
        .unwrap();

        let frame_buffer = std::sync::Arc::new(
            Framebuffer::start(render_pass.clone())
                .add(image_view)
                .unwrap()
                .build()
                .unwrap(),
        );

        self._render_pass = Some(render_pass);
        self._frame_buffer = Some(frame_buffer);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        todo!()
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        todo!()
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _vertex_count: i32,
        _vertex_offset: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!()
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        todo!()
    }

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        todo!()
    }
}

impl<'vs, 'tcs, 'tes, 'gs, 'fs, Vdef, Vss, Tcss, Tess, Gss, Fss>
    GraphicsPipelineBuilder<'vs, 'tcs, 'tes, 'gs, 'fs, Vdef, Vss, Tcss, Tess, Gss, Fss>
{
}

struct GraphicsPipelineBuilder<'vs, 'tcs, 'tes, 'gs, 'fs, Vdef, Vss, Tcss, Tess, Gss, Fss>(
    vulkano::pipeline::GraphicsPipelineBuilder<
        'vs,
        'tcs,
        'tes,
        'gs,
        'fs,
        Vdef,
        Vss,
        Tcss,
        Tess,
        Gss,
        Fss,
    >,
);

impl<'vs, 'tcs, 'tes, 'gs, 'fs, Vdef, Vss, Tcss, Tess, Gss, Fss>
    GraphicsPipelineBuilder<'vs, 'tcs, 'tes, 'gs, 'fs, Vdef, Vss, Tcss, Tess, Gss, Fss>
{
    pub fn push_rasterizer_state(self, _rasterizer_state_info: &RasterizerStateInfo) -> Self {
        Self(self.0.cull_mode_disabled())
    }

    pub fn push_depth_stencil_state(
        self,
        depth_stencil_state_info: &DepthStencilStateInfo,
    ) -> Self {
        let mut result = Self(
            self.0
                .depth_write(depth_stencil_state_info.is_depth_write_enabled()),
        );
        result = Self(result.0.depth_stencil_disabled());
        result
    }

    pub fn push_primitive_topology(self, primitive_topology: &PrimitiveTopology) -> Self {
        match primitive_topology {
            PrimitiveTopology::PointList => Self(self.0.point_list()),
            PrimitiveTopology::TriangleList => Self(self.0.triangle_list()),
        }
    }

    pub fn push_viewport_scissors(
        self,
        viewport_state_info_array: &[ViewportStateInfo],
        scissor_state_info_array: &[ScissorStateInfo],
    ) -> Self {
        let viewport_scissors: Vec<(Viewport, Scissor)> = viewport_state_info_array
            .iter()
            .zip(scissor_state_info_array)
            .map(|x| (x.0.to_vk(), x.1.to_vk()))
            .collect();
        Self(self.0.viewports_scissors(viewport_scissors))
    }

    // pub fn push_viewport_scissor_state(self, viewport_state_info: &ViewportStateInfo, scissor_state_info: &ScissorStateInfo) -> Self {

    // 	let viewport = vulkano::pipeline::viewport::Viewport{
    // 		origin: [viewport_state_info.get_origin_x(), viewport_state_info.get_origin_y()],
    // 		dimensions: [viewport_state_info.get_width(), viewport_state_info.get_height()],
    // 		depth_range: 0.0..1.0,
    // 	};
    // 	let scissor = vulkano::pipeline::viewport::Scissor{
    // 		origin: [scissor_state_info.get_origin_x(), scissor_state_info.get_origin_y()],
    // 		dimensions: [scissor_state_info.get_width() as u32, scissor_state_info.get_height() as u32],
    // 	};
    // 	Self(self.0.viewports_scissors([(viewport, scissor)]))
    // }
}