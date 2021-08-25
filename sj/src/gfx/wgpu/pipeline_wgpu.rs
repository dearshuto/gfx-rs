use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::{Device, Shader};
use std::sync::Arc;

pub struct Pipeline {
	_vertex_shader_module: Option<Arc<wgpu::ShaderModule>>,
	_pixel_shader_module: Option<Arc<wgpu::ShaderModule>>,
	_vertex_bind_grpup: Option<wgpu::BindGroup>,
	_pixel_bind_grpup: Option<wgpu::BindGroup>,
	_compute_bind_group: Option<wgpu::BindGroup>,
    _compute_pipeline: Option<wgpu::ComputePipeline>,
}

impl<'a> IPipelineImpl<'a> for Pipeline {
    fn new_as_graphics(_device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
		Self {
			_vertex_shader_module: Some(info.get_shader().to_data().clone_vertex_shader_module()),
			_pixel_shader_module: Some(info.get_shader().to_data().clone_pixel_shader_module()),
			_vertex_bind_grpup: None,
			_pixel_bind_grpup: None,
			_compute_bind_group: None,
			_compute_pipeline: None,
		}
    }

    fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        let shader = info.get_shader().to_data().clone_vertex_shader_module();
        let compute_pipeline = device.to_data().get_device().create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                layout: None,
                label: None,
                module: &shader,
                entry_point: &"main".to_string(),
            },
        );

        let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
        let bind_group =
            device
                .to_data()
                .get_device()
                .create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &bind_group_layout,
                    entries: &[],
                });

        Self {
            _vertex_shader_module: None,
            _pixel_shader_module: None,
            _vertex_bind_grpup: None,
            _pixel_bind_grpup: None,
			_compute_bind_group: Some(bind_group),
            _compute_pipeline: Some(compute_pipeline),
        }
    }
}

impl Pipeline {
    pub fn is_compute(&self) -> bool {
        self._compute_pipeline.is_some()
    }

	pub fn get_compute_pipeline(&self) -> &wgpu::ComputePipeline {
		&self._compute_pipeline.as_ref().unwrap()
	}

	pub fn get_compute_bind_group(&self) -> &wgpu::BindGroup {
		&self._compute_bind_group.as_ref().unwrap()
	}
}
