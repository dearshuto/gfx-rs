use std::marker::PhantomData;
use super::super::Device;
use super::super::pipeline_api::{IPipelineImpl, PipelineInfo};

pub struct Pipeline<'a>
{
	render_pipeline_impl: Option<wgpu::RenderPipeline>,
	compute_pipeline_impl: Option<wgpu::ComputePipeline>,
	_marker: PhantomData<&'a i32>
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a>{
	fn new(device: &'a mut Device, info: &PipelineInfo) -> Self
	{
		let shader = info.get_shader().to_data().get_impl();
		let compute_pipeline = Some(device.to_data().get_device().create_compute_pipeline(
			&wgpu::ComputePipelineDescriptor{
				label: None,
				layout: None,
				module: shader,
				entry_point: "main"
			}
		));

		Self
		{
			render_pipeline_impl: None,
			compute_pipeline_impl: None,
			_marker: PhantomData,
		}
	}
}
