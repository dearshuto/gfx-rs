use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::{Device, Shader};
use std::marker::PhantomData;

pub struct Pipeline<'a> {
    render_pipeline_impl: Option<wgpu::RenderPipeline>,
    compute_pipeline_impl: Option<wgpu::ComputePipeline>,
    _shader: &'a Shader<'a>,
    _marker: PhantomData<&'a i32>,
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a> {
    fn new_as_graphics(_device: &'a Device, _info: &'a GraphicsPipelineInfo) -> Self {
        todo!()
    }

    fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        let shader_impl = info.get_shader().to_data();
        let shader_module = shader_impl.clone_compute_shader_module();
        let pipeline_layout = info.get_shader().to_data().get_pipeline_layout();
        let compute_pipeline = device.to_data().get_device().create_compute_pipeline(
            &wgpu::ComputePipelineDescriptor {
                layout: Some(pipeline_layout),
                label: None,
                module: &shader_module,
                entry_point: &"main".to_string(),
            },
        );

        Self {
            render_pipeline_impl: None,
            compute_pipeline_impl: Some(compute_pipeline),
            _shader: info.get_shader(),
            _marker: PhantomData,
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
        Some(self.compute_pipeline_impl.as_ref().unwrap())
    }

    pub fn get_shader(&self) -> &Shader {
        self._shader
    }
}
