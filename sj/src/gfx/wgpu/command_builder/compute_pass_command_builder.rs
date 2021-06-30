use crate::gfx::{
    Buffer, ColorTargetView, DepthStencilView, Device, GpuAddress, IndexFormat, Pipeline,
    PrimitiveTopology, ShaderStage, ViewportScissorState,
};

use super::command_builder::ICommandBuilder;

pub struct ComputePassCommandBuilder<'a> {
    _device: &'a Device,
    _compute_pipeline: Option<&'a wgpu::ComputePipeline>,
    _bind_grpup: Option<wgpu::BindGroup>,
    _bind_grpup_layout: Option<wgpu::BindGroupLayout>,
    _buffers: Vec<Option<wgpu::BindingResource<'a>>>,
    _dispatch_count_x: u32,
    _dispatch_count_y: u32,
    _dispatch_count_z: u32,
    _is_end: bool,
}

impl<'a> ComputePassCommandBuilder<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            _device: device,
            _compute_pipeline: None,
            _bind_grpup: None,
            _bind_grpup_layout: None,
            _buffers: Vec::new(),
            _dispatch_count_x: 0,
            _dispatch_count_y: 0,
            _dispatch_count_z: 0,
            _is_end: false,
        }
    }
}

impl<'a> ICommandBuilder<'a> for ComputePassCommandBuilder<'a> {
    fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {
        let mut compute_pass = command_encoder.begin_compute_pass();
        compute_pass.set_pipeline(self._compute_pipeline.unwrap());
        compute_pass.set_bind_group(0, self._bind_grpup.as_ref().unwrap(), &[]);
        compute_pass.dispatch(
            self._dispatch_count_x,
            self._dispatch_count_y,
            self._dispatch_count_z,
        );
    }

    fn is_end(&self) -> bool {
        self._is_end
    }

    fn set_viewport_scissor_state(&mut self, _viewport_scissor_state: &'a ViewportScissorState) {
        assert!(false);
    }

    fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        self._compute_pipeline = pipeline.to_data().get_compute_pipeline();
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        _stage: ShaderStage,
        gpu_address: &GpuAddress,
        _size: usize,
    ) {
        if (slot as usize) < self._buffers.len() {
            self._buffers.resize(slot as usize, None);
        }

        let a = gpu_address.to_data();
        let slice = a.get_buffer().to_data().get_buffer().slice(..);
        self._buffers[0] = Some(wgpu::BindingResource::Buffer(slice));

        // let slice = gpu_address
        //     .to_data()
        //     .get_buffer()
        //     .to_data()
        //     .get_buffer()
        //     .slice(..);

        // self._buffers[slot as usize] = Some(wgpu::BindingResource::Buffer(slice));
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        _stage: ShaderStage,
        gpu_address: &GpuAddress,
        _size: u64,
    ) {
        // self._buffers.resize(slot);
        // self._buffers[slot] = wgpu::BindingResource::Buffer(
        //     gpu_address
        //         .to_data()
        //         .get_buffer()
        //         .to_data()
        //         .get_buffer()
        //         .slice(..),
        // );
    }

    fn set_render_targets(
        &mut self,
        color_target_views: &[&ColorTargetView],
        depth_stencil_state_view: Option<&DepthStencilView>,
    ) {
        assert!(false);
    }

    fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        assert!(false);
    }

    fn draw(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
    ) {
        assert!(false);
    }

    fn draw_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        vertex_count: i32,
        vertex_offset: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        assert!(false);
    }

    fn draw_indexed(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
    ) {
        assert!(false);
    }

    fn draw_indexed_instanced(
        &mut self,
        primitive_topology: PrimitiveTopology,
        index_format: IndexFormat,
        gpu_address: &GpuAddress,
        index_count: i32,
        base_vertex: i32,
        instance_count: i32,
        base_instance: i32,
    ) {
        assert!(false);
    }

    fn draw_indirect(&mut self, gpu_address: &GpuAddress) {
        assert!(false);
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let device_wgpu = self._device.to_data().get_device();
        let bind_group_layout =
            device_wgpu.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: None,
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: wgpu::BufferSize::new(64),
                    },
                    count: None,
                }],
            });
        let bind_group = device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(
                    self._buffers[0].to_data().get_buffer().slice(..),
                ),
            }],
        });
        self._bind_grpup = Some(bind_group);
        self._bind_grpup_layout = Some(bind_group_layout);
        self._dispatch_count_x = group_count_x;
        self._dispatch_count_y = group_count_y;
        self._dispatch_count_z = group_count_z;
        self._is_end = true;
    }
}
