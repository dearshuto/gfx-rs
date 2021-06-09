use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::Device;
use std::marker::PhantomData;

pub struct Pipeline<'a> {
    render_pipeline_impl: Option<wgpu::RenderPipeline>,
    compute_pipeline_impl: wgpu::ComputePipeline,
    bind_group: wgpu::BindGroup,
    _marker: PhantomData<&'a i32>,
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a> {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        todo!()
    }

    fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        let shader = info.get_shader().to_data().get_impl();
        let compute_pipeline = device.to_data().get_device().create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                layout: None,
                label: None,
                compute_stage: wgpu::ProgrammableStageDescriptor {
                    module: shader,
                    entry_point: "main",
                },
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
                    bindings: (),
                });
        let a: wgpu::BindGroup;

        Self {
            render_pipeline_impl: None,
            compute_pipeline_impl: compute_pipeline,
            bind_group,
            _marker: PhantomData,
        }
    }
}

impl<'a> Pipeline<'a> {
    pub fn get_render_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.render_pipeline_impl.as_ref()
    }

    pub fn get_compute_pipeline(&'a self) -> Option<&wgpu::ComputePipeline> {
        Some(&self.compute_pipeline_impl)
    }

    pub fn get_bind_group(&self) -> &wgpu::BindGroup {
        &self.bind_group
    }
}
