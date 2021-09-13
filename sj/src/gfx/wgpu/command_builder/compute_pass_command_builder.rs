use crate::gfx::{Device, Pipeline};
use crate::gfx::common::command_builder::IComputeCommandBuilder;
use std::sync::Arc;

pub struct ComputePassCommandBuilder<'a> {
    _device: &'a Device,
    _pipeline: &'a Pipeline<'a>,
    _compute_pipeline: Option<&'a wgpu::ComputePipeline>,
    _bind_grpup: Option<wgpu::BindGroup>,
    _bind_grpup_layout: Option<wgpu::BindGroupLayout>,
    _buffer_binding: [Option<Arc<wgpu::Buffer>>; 8],
    _buffers: [Option<Arc<wgpu::Buffer>>; 32],
    _dispatch_count_x: u32,
    _dispatch_count_y: u32,
    _dispatch_count_z: u32,
    _is_end: bool,
}

impl<'a> ComputePassCommandBuilder<'a> {
    pub fn new(device: &'a Device, pipeline: &'a Pipeline<'a>) -> Self {
        Self {
            _device: device,
            _pipeline: pipeline,
            _compute_pipeline: None,
            _bind_grpup: None,
            _bind_grpup_layout: None,
            _buffer_binding: std::default::Default::default(),
            _buffers: std::default::Default::default(),
            _dispatch_count_x: 0,
            _dispatch_count_y: 0,
            _dispatch_count_z: 0,
            _is_end: false,
        }
    }

    pub fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {
		let compute_pipeline = self._pipeline.to_data().get_compute_pipeline();
		let bind_group = self._pipeline.to_data().get_compute_bind_group();
        let mut compute_pass =
            command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor { label: None });
		
        compute_pass.set_pipeline(&compute_pipeline);
        compute_pass.set_bind_group(0, &bind_group, &[]);
        compute_pass.dispatch(
            self._dispatch_count_x,
            self._dispatch_count_y,
            self._dispatch_count_z,
        );
    }
}

impl<'a> IComputeCommandBuilder<'a> for ComputePassCommandBuilder<'a> {
    fn build(&mut self) {
        todo!()
    }

    fn set_pipeline(&mut self, _pipeline: &'a Pipeline<'a>) {
        todo!()
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        _stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        _size: usize,
    ) {
        self._buffers[slot as usize] = Some(gpu_address.to_data().clone_buffer());
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        _stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        _size: u64,
    ) {
        assert!(0 <= slot);
		self._buffer_binding[slot as usize] = Some(gpu_address.to_data().clone_buffer());
    }

    fn dispatch(&mut self, _group_count_x: u32, _group_count_y: u32, _group_count_z: u32) {
        // let device_wgpu = self._device.to_data().get_device();
        // let bind_group_layout =
        //     device_wgpu.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
        //         label: None,
        //         entries: &[wgpu::BindGroupLayoutEntry {
        //             binding: 0,
        //             visibility: wgpu::ShaderStage::VERTEX,
        //             ty: wgpu::BindingType::UniformBuffer {
        //                 dynamic: false,
        //                 min_binding_size: wgpu::BufferSize::new(64),
        //             },
        //             count: None,
        //         }],
        //     });
        // let slice = self._buffers[0]
        //     .as_ref()
        //     .unwrap()
        //     .to_data()
        //     .get_buffer()
        //     .get_buffer()
        //     .slice(..);
        // let bind_group = device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
        //     label: None,
        //     layout: &bind_group_layout,
        //     entries: &[wgpu::BindGroupEntry {
        //         binding: 0,
        //         resource: wgpu::BindingResource::Buffer(slice),
        //     }],
        // });
        // self._bind_grpup = Some(bind_group);
        // self._bind_grpup_layout = Some(bind_group_layout);
        // self._dispatch_count_x = group_count_x;
        // self._dispatch_count_y = group_count_y;
        // self._dispatch_count_z = group_count_z;
        // self._is_end = true;
    }
}
