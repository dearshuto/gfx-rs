use crate::gfx::{
    common::command_builder::IGraphicsCommandBuilder,
    wgpu::viewport_scissor_state_wgpu::ViewportScissorStateWgpu, ColorTargetView, DepthStencilView,
    Device, Pipeline, ViewportScissorState,
};
use std::sync::Arc;

pub struct GraphicsPassCommandBuilder<'a> {
    _device: &'a Device,
    _pipeline: &'a Pipeline<'a>,
    _vertex_shader_module: Arc<wgpu::ShaderModule>,
    _pixel_shader_module: Arc<wgpu::ShaderModule>,
    _render_pipeline: Option<wgpu::RenderPipeline>,
    _render_targets: Option<Vec<Arc<wgpu::TextureView>>>,
    _viewport_scissor_state: Option<ViewportScissorStateWgpu>,
    _vertex_buffers: [Option<Arc<wgpu::Buffer>>; 2],
    _constant_buffers: [Option<Arc<wgpu::Buffer>>; 32],
    _bind_group: Option<wgpu::BindGroup>,
    _vertex_offset: u32,
    _vertrex_count: u32,
    _draw_commands: Vec<DrawCommand>,
}

impl<'a> GraphicsPassCommandBuilder<'a> {
    pub fn new(device: &'a Device, pipeline: &'a Pipeline<'a>) -> Self {
        Self {
            _device: device,
            _pipeline: pipeline,
            _vertex_shader_module: pipeline.to_data().clone_vertex_shader_module(),
            _pixel_shader_module: pipeline.to_data().clone_pixel_shader_module(),
            _render_pipeline: None,
            _render_targets: None,
            _viewport_scissor_state: None,
            _vertex_buffers: std::default::Default::default(),
            _constant_buffers: std::default::Default::default(),
            _bind_group: None,
            _vertex_offset: 0,
            _vertrex_count: 0,
            _draw_commands: Vec::new(),
        }
    }

    pub fn push_command(&self, command_encoder: &mut wgpu::CommandEncoder) {
        // ImageView から変換する
        let render_pass_color_attachment: Vec<wgpu::RenderPassColorAttachment> = self
            ._render_targets
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| wgpu::RenderPassColorAttachment {
                view: x,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: true,
                },
            })
            .collect();
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &render_pass_color_attachment,
            depth_stencil_attachment: None,
        };
        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);

        // パイプライン
        render_pass.set_pipeline(&self._render_pipeline.as_ref().unwrap());

        // ビューポートシザー
        self._viewport_scissor_state
            .as_ref()
            .unwrap()
            .push(&mut render_pass);

        // デスクリプタたちをセット
        render_pass.set_bind_group(0, self._bind_group.as_ref().unwrap(), &[]);

        // 頂点バッファ
        for (index, vertex_buffer_opt) in self._vertex_buffers.iter().enumerate() {
            if let Some(vertex_buffer) = vertex_buffer_opt {
                render_pass.set_vertex_buffer(index as u32, vertex_buffer.slice(..));
            }
        }

        // 描画コマンド
        for draw_command in &self._draw_commands {
            draw_command.push_draw_command(&mut render_pass);
        }
    }

    // fn create_bind_group(&self) -> wgpu::BindGroup {
    // 	todo!()
    // let device_wgpu = self._device.to_data().get_device();
    // let bind_group_layout = self
    //     ._pipeline
    //     .to_data()
    //     .get_shader()
    //     .to_data()
    //     .get_bind_group_layout();

    // let slice = self._constant_buffers[0]
    //     .as_ref()
    //     .unwrap()
    //     .to_data()
    //     .get_buffer()
    //     .get_buffer()
    //     .slice(..);

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

    // device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
    //     label: None,
    //     layout: bind_group_layout,
    //     entries: &entrices,
    // })
    //}
}

impl<'a> IGraphicsCommandBuilder<'a> for GraphicsPassCommandBuilder<'a> {
    fn build(&mut self) {
        let render_pipeline_descriptor = wgpu::RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: wgpu::VertexState {
                module: &self._vertex_shader_module,
                entry_point: "main",
                buffers: &[],
            },
            fragment: Some(wgpu::FragmentState {
                module: &self._pixel_shader_module,
                entry_point: "main",
                targets: &[],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
        };
        self._render_pipeline = Some(
            self._device
                .to_data()
                .get_device()
                .create_render_pipeline(&render_pipeline_descriptor),
        );
    }

    fn set_viewport_scissor_state(&mut self, _viewport_scissor_state: &'a ViewportScissorState) {
        self._viewport_scissor_state = Some(*viewport_scissor_state.to_data());
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        _stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        _size: usize,
    ) {
        self._constant_buffers[slot as usize] = Some(gpu_address.to_data().clone_buffer());
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
        color_target_views: &[&ColorTargetView],
        _depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        self._render_targets = Some(
            color_target_views
                .iter()
                .map(|x| x.to_data().clone_texture_view())
                .collect(),
        );
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &crate::gfx::GpuAddress) {
        self._vertex_buffers[buffer_index as usize] = Some(gpu_address.to_data().clone_buffer());
    }

    fn draw(
        &mut self,
        primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self.draw_instanced(primitive_topology, vertex_count, vertex_offset, 1, 0);
    }

    fn draw_instanced(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        let vertices = (vertex_offset as u32)..(vertex_count as u32);
        let instances = (base_instance as u32)..(instance_count as u32);
        let command = DrawCommand::Simple(vertices, instances);
        self._draw_commands.push(command);
    }

    fn draw_indexed(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _index_format: crate::gfx::IndexFormat,
        _gpu_address: &crate::gfx::GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
    ) {
        todo!()
    }

    fn draw_indexed_instanced(
        &mut self,
        _primitive_topology: crate::gfx::PrimitiveTopology,
        _index_format: crate::gfx::IndexFormat,
        _gpu_address: &crate::gfx::GpuAddress,
        _index_count: i32,
        _base_vertex: i32,
        _instance_count: i32,
        _base_instance: i32,
    ) {
        todo!()
    }
}

enum DrawCommand {
    Simple(std::ops::Range<u32>, std::ops::Range<u32>),
}

impl DrawCommand {
    pub fn push_draw_command(&self, render_pass: &mut wgpu::RenderPass) {
        match self {
            Self::Simple(vertices, instances) => {
                render_pass.draw(vertices.clone(), instances.clone())
            }
        }
    }
}
