use super::{
    BlendStateInfo, DepthStencilStateInfo, Device, RasterizerStateInfo, Shader, VertexStateInfo,
};
use std::marker::PhantomData;

pub struct GraphicsPipelineInfo<'a> {
    _shader: Option<&'a Shader<'a>>,
    _vertex_state_info: Option<&'a VertexStateInfo<'a>>,
    _blend_state_info: Option<&'a BlendStateInfo<'a>>,
    _depth_stencil_state_info: Option<&'a DepthStencilStateInfo>,
    _rasterizer_state_info: Option<&'a RasterizerStateInfo>,
}

impl<'a> GraphicsPipelineInfo<'a> {
    pub fn new() -> Self {
        Self {
            _shader: None,
            _vertex_state_info: None,
            _blend_state_info: None,
            _depth_stencil_state_info: None,
            _rasterizer_state_info: None,
        }
    }

    pub fn get_shader(&self) -> &'a Shader<'a> {
        self._shader.unwrap()
    }

    pub fn set_shader(mut self, shader: &'a Shader<'a>) -> Self {
        self._shader = Some(shader);
        self
    }

    pub fn get_vertex_state_info(&self) -> &'a VertexStateInfo {
        self._vertex_state_info.unwrap()
    }

    pub fn set_vertex_state_info(mut self, vertex_state_info: &'a VertexStateInfo) -> Self {
        self._vertex_state_info = Some(vertex_state_info);
        self
    }

    pub fn get_blend_state_info(&self) -> &'a BlendStateInfo {
        self._blend_state_info.unwrap()
    }

    pub fn set_blend_stae(mut self, blend_state: &'a BlendStateInfo) -> Self {
        self._blend_state_info = Some(blend_state);
        self
    }

    pub fn get_depth_stencil_state(&self) -> &'a DepthStencilStateInfo {
        self._depth_stencil_state_info.unwrap()
    }

    pub fn set_depth_stencil_state(
        mut self,
        depth_stencil_state: &'a DepthStencilStateInfo,
    ) -> Self {
        self._depth_stencil_state_info = Some(depth_stencil_state);
        self
    }

    pub fn get_rasterizer_state(&self) -> &'a RasterizerStateInfo {
        self._rasterizer_state_info.unwrap()
    }

    pub fn set_rasterizer_state(mut self, rasterizer_state: &'a RasterizerStateInfo) -> Self {
        self._rasterizer_state_info = Some(rasterizer_state);
        self
    }
}

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
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self;

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
    pub fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        Self {
            pipeline_impl: T::new_as_graphics(device, info),
            _marker: PhantomData,
        }
    }

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
