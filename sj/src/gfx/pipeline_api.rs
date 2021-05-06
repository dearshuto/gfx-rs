use super::Device;
use super::Shader;
use std::marker::PhantomData;

pub struct ComputePipelineInfo<'a> {
    _shader: Option<&'a Shader<'a>>,
}

impl<'a> ComputePipelineInfo<'a> {
    pub fn new() -> Self {
        ComputePipelineInfo { _shader: None }
    }

    pub fn get_shader(&self) -> &'a Shader<'a> {
        self._shader.unwrap()
    }

    pub fn set_shader(mut self, shader: &'a Shader<'a>) -> Self {
        self._shader = Some(shader);
        self
    }
}

pub trait IPipelineImpl<'a> {
    fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self;
}

pub struct TPipelineInterface<'a, T: 'a>
where
    T: IPipelineImpl<'a>,
{
    pipeline_impl: T,
    _marker: PhantomData<&'a T>,
}

impl<'a, T: IPipelineImpl<'a>> TPipelineInterface<'a, T> {
    pub fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        Self {
            pipeline_impl: T::new_as_compute(device, info),
            _marker: PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self.pipeline_impl
    }
}
