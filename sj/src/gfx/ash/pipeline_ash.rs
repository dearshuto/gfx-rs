use super::super::pipeline_api::{ComputePipelineInfo, IPipelineImpl};
use super::super::{Device, Shader};

pub struct PipelineImpl<'a> {
    _shader: &'a Shader<'a>,
}

impl<'a> PipelineImpl<'a> {
    pub fn get_shader(&self) -> &'a Shader<'a> {
        self._shader
    }

    pub fn get_pipeline(&self) -> &ash::vk::Pipeline {
        self._shader.to_data().get_compute_pipeline()
    }
}

impl<'a> IPipelineImpl<'a> for PipelineImpl<'a> {
    fn new_as_compute<'b>(_device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        Self {
            _shader: info.get_shader(),
        }
    }
}
