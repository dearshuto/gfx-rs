use crate::gfx::{Device, GpuAddress, Pipeline, ShaderStage};

pub struct ComputePassCommandBuilder<'a> {
    _device: &'a Device,
    _compute_pipeline: Option<&'a wgpu::ComputePipeline>,
    _bind_grpup: Option<wgpu::BindGroup>,
    _bind_grpup_layout: Option<wgpu::BindGroupLayout>,
    _buffers: [Option<GpuAddress<'a>>; 32],
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
            _buffers: std::default::Default::default(),
            _dispatch_count_x: 0,
            _dispatch_count_y: 0,
            _dispatch_count_z: 0,
            _is_end: false,
        }
    }

    pub fn build(&self, command_encoder: &mut wgpu::CommandEncoder) {
        let mut compute_pass = command_encoder.begin_compute_pass();
        compute_pass.set_pipeline(self._compute_pipeline.unwrap());
        compute_pass.set_bind_group(0, self._bind_grpup.as_ref().unwrap(), &[]);
        compute_pass.dispatch(
            self._dispatch_count_x,
            self._dispatch_count_y,
            self._dispatch_count_z,
        );
    }

    pub fn set_pipeline(&mut self, pipeline: &'a Pipeline<'a>) {
        self._compute_pipeline = pipeline.to_data().get_compute_pipeline();
    }

    pub fn set_constant_buffer(
        &mut self,
        slot: i32,
        _stage: ShaderStage,
        gpu_address: GpuAddress<'a>,
        _size: usize,
    ) {
        self._buffers[slot as usize] = Some(gpu_address);

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

    pub fn set_vertex_buffer(&mut self, buffer_index: i32, gpu_address: &GpuAddress) {
        assert!(false);
    }

    pub fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
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
        let slice = self._buffers[0]
            .as_ref()
            .unwrap()
            .to_data()
            .get_buffer()
            .get_buffer()
            .slice(..);
        let bind_group = device_wgpu.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(slice),
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
