use crate::gfx::{
    wgpu::viewport_scissor_state_wgpu::ViewportScissorStateWgpu, ColorTargetView, DepthStencilView,
    Device, GpuAddress, IndexFormat, Pipeline, PrimitiveTopology, ShaderStage,
    ViewportScissorState,
};

pub struct GraphicsPassCommandBuilder<'a> {
    _device: &'a Device,
    _pipeline: &'a Pipeline<'a>,
    _viewport_scissor_state: Option<ViewportScissorStateWgpu>,
    _vertex_buffers: [Option<wgpu::BufferSlice<'a>>; 8],
    _vertex_offset: u32,
    _vertrex_count: u32,
}

impl<'a> GraphicsPassCommandBuilder<'a> {
    pub fn new(device: &'a Device, pipeline: &'a Pipeline<'a>) -> Self {
        Self {
            _device: device,
            _pipeline: pipeline,
            _viewport_scissor_state: None,
            _vertex_buffers: std::default::Default::default(),
            _vertex_offset: 0,
            _vertrex_count: 0,
        }
    }

    pub fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {
        let render_pass_descriptor = wgpu::RenderPassDescriptor {
            color_attachments: &[],
            depth_stencil_attachment: None,
        };
        let mut render_pass = command_encoder.begin_render_pass(&render_pass_descriptor);

        // ビューポートシザー
        self._viewport_scissor_state
            .as_ref()
            .unwrap()
            .push(&mut render_pass);

        render_pass.draw(
            std::ops::Range {
                start: self._vertex_offset,
                end: self._vertrex_count,
            },
            std::ops::Range { start: 0, end: 1 },
        );
    }

    pub fn set_viewport_scissor_state(&mut self, viewport_scissor_state: &'a ViewportScissorState) {
        self._viewport_scissor_state = Some(*viewport_scissor_state.to_data());
    }

    pub fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: GpuAddress<'a>,
        size: usize,
    ) {
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: ShaderStage,
        gpu_address: &GpuAddress,
        size: u64,
    ) {
    }

    pub fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
    }

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress<'a>) {
        // let slice = gpu_address
        //     .to_data()
        //     .get_buffer()
        //     .to_data()
        //     .get_buffer()
        //     .slice(..);
        // self._vertex_buffers[buffer_index as usize] = Some(slice);
    }

    pub fn draw(
        &mut self,
        _primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        self._vertrex_count = vertex_count as u32;
        self._vertex_offset = vertex_offset as u32;
    }

    pub fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
    }

    pub fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
    }

    pub fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
    }

    pub fn draw_indirect(&mut self, gpu_address: &GpuAddress) {}

    fn create_bind_group(&self) -> wgpu::BindGroup {
        let device_wgpu = self._device.to_data().get_device();
        let bind_group_layout = self
            ._pipeline
            .to_data()
            .get_shader()
            .to_data()
            .get_bind_group_layout();

        device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: bind_group_layout,
            entries: &[],
        })
    }
}
