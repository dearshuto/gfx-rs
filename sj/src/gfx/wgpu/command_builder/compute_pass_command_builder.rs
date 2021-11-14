use crate::gfx::{Device, GpuAddress, Pipeline, ShaderStage};
use std::sync::Arc;

enum BufferType {
    UnorderedAccess(std::sync::Arc<wgpu::Buffer>),
}

pub struct ComputePassCommandBuilder<'a> {
    _device: &'a Device,
    _pipeline: Option<&'a Pipeline<'a>>,
    _unordered_access_buffers: [Option<std::sync::Arc<wgpu::Buffer>>; 32],
    _bind_group: Option<wgpu::BindGroup>,
    _dispatch_count_x: u32,
    _dispatch_count_y: u32,
    _dispatch_count_z: u32,
    _is_end: bool,
}

impl<'a> ComputePassCommandBuilder<'a> {
    pub fn new(device: &'a Device) -> Self {
        Self {
            _device: device,
            _pipeline: None,
            _unordered_access_buffers: std::default::Default::default(),
            _bind_group: None,
            _dispatch_count_x: 0,
            _dispatch_count_y: 0,
            _dispatch_count_z: 0,
            _is_end: false,
        }
    }

    pub fn end(&mut self) {}

    pub fn build(&mut self) {
        let pipeline_impl = self._pipeline.unwrap().to_data();
        let shader_impl = pipeline_impl.get_shader().to_data();
        let compute_pipeline = pipeline_impl.get_compute_pipeline().unwrap();

        let entries = self.create_entries();
        let bind_group =
            self._device
                .to_data()
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: shader_impl.get_bind_group_layout(),
                    entries: &entries,
                });

        // let mut compute_pass = command_encoder.begin_compute_pass(&wgpu::ComputePassDescriptor{ label: None });
        // compute_pass.set_pipeline(&compute_pipeline);
        // compute_pass.set_bind_group(0, &bind_group, &[]);
        // compute_pass.dispatch(
        //     self._dispatch_count_x,
        //     self._dispatch_count_y,
        //     self._dispatch_count_z,
        // );

        self._bind_group = Some(bind_group);
    }

    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        self._pipeline = Some(pipeline);
    }

    pub fn set_constant_buffer(
        &mut self,
        _slot: i32,
        _stage: ShaderStage,
        _gpu_address: GpuAddress<'a>,
        _size: usize,
    ) {
        //self._buffers[slot as usize] = Some(gpu_address);

        // let slice = gpu_address
        //     .to_data()
        //     .get_buffer()
        //     .to_data()
        //     .get_buffer()
        //     .slice(..);

        // self._buffers[slot as usize] = Some(wgpu::BindingResource::Buffer(slice));
    }

    pub fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        _stage: ShaderStage,
        gpu_address: &GpuAddress,
        _size: u64,
    ) {
        self._unordered_access_buffers[slot as usize] =
            Some(gpu_address.to_data().get_buffer().clone_buffer());
    }

    pub fn set_vertex_buffer(&mut self, _buffer_index: i32, _gpu_address: &GpuAddress) {
        assert!(false);
    }

    pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        self._dispatch_count_x = group_count_x;
        self._dispatch_count_y = group_count_y;
        self._dispatch_count_z = group_count_z;
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        self._bind_group.as_ref().unwrap()
    }

    pub fn get_dispatch_count(&self) -> (u32, u32, u32) {
        (
            self._dispatch_count_x,
            self._dispatch_count_y,
            self._dispatch_count_z,
        )
    }

    fn create_entries(&self) -> Vec<wgpu::BindGroupEntry> {
        let mut entries = Vec::new();

        // TODO
        for index in 0..self._unordered_access_buffers.len() {
            if let Some(buffer) = &self._unordered_access_buffers[index] {
                let entry = wgpu::BindGroupEntry {
                    binding: index as u32,
                    resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                        buffer: &buffer,
                        offset: 0,
                        size: None,
                    }),
                };

                entries.push(entry);
            }
        }

        entries
    }
}
