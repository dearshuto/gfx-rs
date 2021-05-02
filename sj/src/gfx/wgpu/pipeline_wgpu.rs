use std::marker::PhantomData;
use super::super::Device;
use super::super::pipeline_api::{IPipelineImpl, PipelineInfo};

pub struct Pipeline<'a>
{
	render_pipeline_impl: Option<wgpu::RenderPipeline>,
	compute_pipeline_impl: wgpu::ComputePipeline,
	bind_group: wgpu::BindGroup,
	_marker: PhantomData<&'a i32>
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a>{
	fn new(device: &'a Device, info: &PipelineInfo) -> Self
	{
		let shader = info.get_shader().to_data().get_impl();
		let compute_pipeline = device.to_data().get_device().create_compute_pipeline(
			&wgpu::ComputePipelineDescriptor{
				label: None,
				layout: None,
				module: shader,
				entry_point: "main"
			}
		);		
		
		let bind_group_layout = compute_pipeline.get_bind_group_layout(0);
		let bind_group = device.to_data().get_device().create_bind_group(&wgpu::BindGroupDescriptor {
			label: None,
			layout: &bind_group_layout,
			entries: &[],
		});
		let a: wgpu::BindGroup;
		
		Self
		{
			render_pipeline_impl: None,
			compute_pipeline_impl: compute_pipeline,
			bind_group,
			_marker: PhantomData,
		}
	}
}

impl<'a> Pipeline<'a>
{
	pub fn get_render_pipeline(&self) -> Option<&wgpu::RenderPipeline>
	{
		self.render_pipeline_impl.as_ref()
	}
	
	pub fn get_compute_pipeline(&'a self) -> Option<&wgpu::ComputePipeline>
	{
		Some(&self.compute_pipeline_impl)
	}

	pub fn get_bind_group(&self) -> &wgpu::BindGroup
	{
		&self.bind_group
	}
}
