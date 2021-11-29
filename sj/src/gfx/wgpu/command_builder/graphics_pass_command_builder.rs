use std::ops::Range;

use wgpu::RenderPipeline;

use crate::gfx::{
    wgpu::viewport_scissor_state_wgpu::ViewportScissorStateWgpu, ColorTargetView, DepthStencilView,
    Device, GpuAddress, IndexFormat, Pipeline, PrimitiveTopology, ShaderStage,
    ViewportScissorState,
};

pub struct GraphicsPassCommandBuilder<'a> {
    _device: &'a Device,
    _pipeline: &'a Pipeline<'a>,
    _bind_group: Option<wgpu::BindGroup>,
    _render_pipeline: Option<RenderPipeline>,
    _color_attachment_descriptors: Vec<wgpu::RenderPassColorAttachment<'a>>,
    _viewport_scissor_state: Option<ViewportScissorStateWgpu>,
    // _unordered_access_buffers: [Option<std::sync::Arc<wgpu::Buffer>>; 32],
    _vertex_buffers: [Option<GpuAddress<'a>>; 2],
    //_constant_buffers: [Option<GpuAddress<'a>>; 32],
    // _vertex_offset: u32,
    // _vertrex_count: u32,

    // レンダーターゲット
    _render_targert_format: Option<wgpu::TextureFormat>,

    // 描画コマンド
    _primitive_topology: PrimitiveTopology,
    _vertex_count: u32,
    _vertex_offset: u32,
    _instance_count: u32,
    _base_instance: u32,
}

impl<'a> GraphicsPassCommandBuilder<'a> {
    pub fn new(device: &'a Device, pipeline: &'a Pipeline<'a>) -> Self {
        Self {
            _device: device,
            _pipeline: pipeline,
            _bind_group: None,
            _render_pipeline: None,
            _color_attachment_descriptors: Vec::new(),
            _viewport_scissor_state: None,
            _vertex_buffers: std::default::Default::default(),
            // _constant_buffers: std::default::Default::default(),
            // _vertex_offset: 0,
            // _vertrex_count: 0,
            _render_targert_format: None,
            _primitive_topology: PrimitiveTopology::TriangleList,
            _vertex_count: 0,
            _vertex_offset: 0,
            _instance_count: 0,
            _base_instance: 0,
        }
    }

    pub fn build(&mut self) {
        let pipeline_impl = self._pipeline.to_data();
        let shader_impl = self._pipeline.to_data().get_shader();
        let vertex_buffers = [wgpu::VertexBufferLayout {
            array_stride: (std::mem::size_of::<f32>() * 2) as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: shader_impl.to_data().get_vertex_attributes(),
        }];

        // バインドグループ
        self._bind_group = Some(self.create_bind_group());

        // 描画パイプライン
        let render_pipeline = self._device.to_data().get_device().create_render_pipeline(
            &wgpu::RenderPipelineDescriptor {
                label: None,
                layout: Some(
                    self._pipeline
                        .to_data()
                        .get_shader()
                        .to_data()
                        .get_pipeline_layout(),
                ),
                vertex: wgpu::VertexState {
                    module: shader_impl.to_data().get_vertex_shader_module(),
                    entry_point: "main",
                    buffers: &vertex_buffers,
                },
                primitive: pipeline_impl.create_primitive_state(wgpu::IndexFormat::Uint32),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: shader_impl.to_data().get_pixel_shader_module(),
                    entry_point: "main",
                    targets: &[wgpu::ColorTargetState {
                        format: self._render_targert_format.as_ref().unwrap().clone(),
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    }],
                }),
            },
        );
        self._render_pipeline = Some(render_pipeline);
    }

    pub fn set_viewport_scissor_state(
        &mut self,
        _viewport_scissor_state: &'a ViewportScissorState,
    ) {
        //self._viewport_scissor_state = Some(*viewport_scissor_state.to_data());
    }

    pub fn set_constant_buffer(
        &mut self,
        _slot: i32,
        _stage: ShaderStage,
        _gpu_address: GpuAddress<'a>,
        _size: usize,
    ) {
        todo!()
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        _slot: i32,
        _stage: ShaderStage,
        _gpu_address: &GpuAddress,
        _size: u64,
    ) {
        todo!()
    }

    pub fn set_scan_buffer_view_as_render_target(
        &mut self,
        render_target_format: wgpu::TextureFormat,
    ) {
        self._render_targert_format = Some(render_target_format);
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&'a ColorTargetView<'a>],
        _depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        // TODO: MRT
        let view = color_target_views[0].to_data().get_texture_view();
        let color_attachment_descriptor = wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                store: true,
            },
        };
        self._color_attachment_descriptors.clear();
        self._color_attachment_descriptors
            .push(color_attachment_descriptor);
    }

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: GpuAddress<'a>) {
        self._vertex_buffers[buffer_index as usize] = Some(gpu_address);
    }

    pub fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw_instanced(primitive_topology, vertex_count, vertex_offset, 1, 0);
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        self._primitive_topology = primitive_topology;
        self._vertex_count = vertex_count as u32;
        self._vertex_offset = vertex_offset as u32;
        self._instance_count = instance_count as u32;
        self._base_instance = base_instance as u32;
    }

    pub fn draw_indexed(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: IndexFormat,
        _gpu_address: &GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
    ) {
        todo!()
    }

    pub fn draw_indexed_instanced(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        _index_format: IndexFormat,
        _gpu_address: &GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!()
    }

    pub fn draw_indirect(&mut self, _gpu_address: &GpuAddress) {
        todo!()
    }

    pub fn get_vertex_buffer(&self) -> &wgpu::Buffer {
        self._vertex_buffers[0]
            .as_ref()
            .unwrap()
            .to_data()
            .get_buffer()
            .get_buffer()
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self._bind_group.as_ref().unwrap()
    }

    pub fn get_render_pipeline(&self) -> &wgpu::RenderPipeline {
        self._render_pipeline.as_ref().unwrap()
    }

    pub fn get_vertices_range(&self) -> Range<u32> {
        self._vertex_offset..(self._vertex_offset + self._vertex_count)
    }

    pub fn get_instance_range(&self) -> Range<u32> {
        self._base_instance..(self._base_instance + self._instance_count)
    }

    // pub fn get_primitive_topology(&self) -> wgpu::PrimitiveTopology {
    // 	match self._primitive_topology {
    // 		PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
    // 		PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
    // 	}
    // }

    // pub fn get_vertex_count(&self) -> u32 {
    // 	self._vertex_count
    // }

    // pub fn get_vertex_offset(&self) -> u32 {
    // 	self._vertex_offset
    // }

    // fn create_render_pass_descriptor(&self) -> wgpu::RenderPassDescriptor {
    //     wgpu::RenderPassDescriptor {
    //         color_attachments: &[],
    //         depth_stencil_attachment: None,
    //         label: None,
    //     }
    // }

    fn create_bind_group(&self) -> wgpu::BindGroup {
        let device_wgpu = self._device.to_data().get_device();
        let bind_group_layout = self
            ._pipeline
            .to_data()
            .get_shader()
            .to_data()
            .get_bind_group_layout();
        // let entrices = [
        //     wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: wgpu::BindingResource::Buffer(slice),
        //     },
        //     wgpu::BindGroupEntry {
        //         binding: 1,
        //         resource: wgpu::BindingResource::Buffer(slice),
        //     },
        // ];

        device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[],
        })
    }

    // fn push_vertex_buffers(
    //     render_pass: &mut wgpu::RenderPass<'a>,
    //     vertex_buffer_gpu_addresses: &'a [GpuAddress<'a>],
    // ) {
    //     let slice = vertex_buffer_gpu_addresses[0]
    //         .to_data()
    //         .get_buffer()
    //         .get_buffer()
    //         .slice(..);
    //     render_pass.set_vertex_buffer(0, slice);
    // }
}
