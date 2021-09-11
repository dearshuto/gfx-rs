use vulkano::{buffer::BufferAccess, command_buffer::{DynamicState, PrimaryAutoCommandBuffer, SubpassContents, pool::CommandPoolBuilderAlloc}, image::{view::ImageView, AttachmentImage, ImageUsage}, pipeline::{
        vertex::VertexSource,
        viewport::{Scissor, Viewport},
        GraphicsPipelineAbstract,
    }, render_pass::{Framebuffer, FramebufferAbstract, RenderPass, Subpass}};

use crate::gfx::{
    common::command_builder::IGraphicsCommandBuilder, DepthStencilStateInfo, Pipeline,
    PrimitiveTopology, RasterizerStateInfo, ScissorStateInfo, ViewportStateInfo,
};

use super::draw_command::{
    DrawCommand, IndexedDrawCommandInfo, IndexedInstancedDrawInfo, InstancedDrawCommandInfo,
    SimpleDrawCommandInfo,
};

pub struct GraphicsCommandBuilder<'a> {
    _device: std::sync::Arc<vulkano::device::Device>,
    _pipeline: &'a Pipeline<'a>,
    _rasterizer_state_info: RasterizerStateInfo,
    _depth_stencil_state_info: DepthStencilStateInfo,
    _viewport_state_info_array: Option<Vec<ViewportStateInfo>>,
    _scissor_state_info_array: Option<Vec<ScissorStateInfo>>,
    _primitive_topology: Option<PrimitiveTopology>,
    _render_pass: Option<std::sync::Arc<RenderPass>>,
    _frame_buffer: Option<std::sync::Arc<dyn FramebufferAbstract>>,
    _vertex_buffer: [Option<std::sync::Arc<dyn BufferAccess>>; 8],
    _draw_command: Vec<DrawCommand>,
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
            _pipeline: pipeline,
            _rasterizer_state_info: *pipeline.to_data().get_rasterizer_state_info(),
            _depth_stencil_state_info: *pipeline.to_data().get_depth_stencil_state_info(),
            _viewport_state_info_array: None,
            _scissor_state_info_array: None,
            _primitive_topology: None,
            _render_pass: None,
            _frame_buffer: None,
            _vertex_buffer: std::default::Default::default(),
            _draw_command: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build<P>(
        &self,
        command_builder: &mut vulkano::command_buffer::AutoCommandBufferBuilder<PrimaryAutoCommandBuffer<P::Alloc>, P>,
    )
	where
		P: CommandPoolBuilderAlloc,
	{
        let graphics_pipeline_builder = GraphicsPipelineBuilder(
            vulkano::pipeline::GraphicsPipeline::start()
                .vertex_shader(self._pipeline.to_data().clone_vertex_entry_point(), ())
				.fragment_shader(self._pipeline.to_data().clone_pixel_entry_point(), ())
                .render_pass(Subpass::from(self._render_pass.as_ref().unwrap().clone(), 0).unwrap())
        )
        .push_rasterizer_state(&self._rasterizer_state_info)
        .push_depth_stencil_state(&self._depth_stencil_state_info)
        .push_primitive_topology(&self._primitive_topology.as_ref().unwrap())
        .push_viewport_scissors(
            self._viewport_state_info_array.as_ref().unwrap(),
            self._scissor_state_info_array.as_ref().unwrap(),
        );
        let pipeline = std::sync::Arc::new(
            graphics_pipeline_builder
                .0
                .build(self._device.clone())
                .unwrap(),
        );
        let vertex_buffers: Vec<std::sync::Arc<dyn BufferAccess>> = self
            ._vertex_buffer
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.as_ref().unwrap().clone())
            .collect();
        let dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };

        command_builder
			.begin_render_pass(self._frame_buffer.unwrap().clone(), SubpassContents::Inline, [])
            //.draw(pipeline, &dynamic_state, vertex_buffers, (), ())
            .unwrap();
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
        _slot: i32,
        _stage: crate::gfx::ShaderStage,
        _gpu_address: &crate::gfx::GpuAddress,
        _size: usize,
    ) {
        todo!()
    }

    fn set_unordered_access_buffer(
        &mut self,
        _slot: i32,
        _stage: crate::gfx::ShaderStage,
        _gpu_address: &crate::gfx::GpuAddress,
        _size: u64,
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
        let buffer = gpu_address.to_data().clone_buffer_access();
        self._vertex_buffer[buffer_index as usize] = Some(buffer);
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        _vertex_count: i32,
        _vertex_offset: i32,
    ) {
        self._primitive_topology = Some(primitive_topology);
        //     self._draw_command.push(
        // 		DrawCommand::Simple(SimpleDrawCommandInfo::new(primitive_topology, vertex_count, vertex_offset)),
        // 	);
    }

    fn draw_instanced(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self._draw_command
            .push(DrawCommand::Instanced(InstancedDrawCommandInfo::new(
                primitive_topology,
                vertex_count,
                vertex_offset,
                instance_count,
                base_instance,
            )));
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        index_format: crate::gfx::IndexFormat,
        gpu_address: &crate::gfx::GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        let info = IndexedDrawCommandInfo::new(
            primitive_topology,
            index_format,
            gpu_address.to_data().clone_buffer_access(),
            index_count,
            base_vertex,
        );
        let command = DrawCommand::Indexed(info);
        self._draw_command.push(command);
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
        let info = IndexedInstancedDrawInfo::new(
            primitive_topology,
            index_format,
            gpu_address.to_data().clone_buffer_access(),
            index_count,
            base_vertex,
            instance_count,
            base_instance,
        );
        let command = DrawCommand::IndexedInstancing(info);
        self._draw_command.push(command);
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

    fn push_draw_simple_command<V, Gp>(
        mut self,
        info: &SimpleDrawCommandInfo,
        graphics_pipeline: Gp,
        vertex_buffers: V,
    ) where
        Gp: GraphicsPipelineAbstract + VertexSource<V> + Send + Sync + 'static + Clone,
    {
        let mut dynamic_state = DynamicState {
            line_width: None,
            viewports: None,
            scissors: None,
            compare_mask: None,
            write_mask: None,
            reference: None,
        };
        //self.0.draw(graphics_pipeline, &dynamic_state, vertex_buffers, (), ()).unwrap();
    }
}

struct AutoCommandBufferBuilder<T, P>(vulkano::command_buffer::AutoCommandBufferBuilder<T, P>);

impl<T, P> AutoCommandBufferBuilder<T, P> {
    pub fn push_draw_command<V, Gp>(
        self,
        draw_command: &DrawCommand,
        graphics_pipeline: std::sync::Arc<Gp>,
    ) -> Self
    where
        Gp: GraphicsPipelineAbstract + VertexSource<V> + Send + Sync + 'static + Clone,
    {
        match draw_command {
            &DrawCommand::Simple(ref info) => panic!(),
            &DrawCommand::Instanced(ref info) => self.push_draw_instanced_command(info),
            &DrawCommand::Indexed(ref info) => self.push_draw_indexed_command(info),
            &DrawCommand::IndexedInstancing(ref info) => {
                self.push_draw_indexed_instanced_command(info)
            }
        }
    }

    fn push_draw_instanced_command(self, info: &InstancedDrawCommandInfo) -> Self {
        self
    }

    fn push_draw_indexed_command(self, info: &IndexedDrawCommandInfo) -> Self {
        self
    }

    fn push_draw_indexed_instanced_command(self, info: &IndexedInstancedDrawInfo) -> Self {
        self
    }
}
