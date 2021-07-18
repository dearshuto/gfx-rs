use crate::gfx::Shader;

use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::Device;

pub struct Pipeline<'a> {
    render_pipeline_impl: Option<wgpu::RenderPipeline>,
    compute_pipeline_impl: Option<wgpu::ComputePipeline>,
    _shader: &'a Shader<'a>,
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a> {
    fn new_as_graphics(_device: &'a Device, _info: &'a GraphicsPipelineInfo) -> Self {
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

        Self {
            render_pipeline_impl: None,
            compute_pipeline_impl: Some(compute_pipeline),
            _shader: info.get_shader(),
        }
    }
}

impl<'a> Pipeline<'a> {
    pub fn is_compute(&self) -> bool {
        self.compute_pipeline_impl.is_some()
    }

    pub fn get_render_pipeline(&self) -> Option<&wgpu::RenderPipeline> {
        self.render_pipeline_impl.as_ref()
    }

    pub fn get_compute_pipeline(&'a self) -> Option<&wgpu::ComputePipeline> {
        self.compute_pipeline_impl.as_ref()
    }

    pub fn get_shader(&self) -> &Shader {
        self._shader
    }
}
