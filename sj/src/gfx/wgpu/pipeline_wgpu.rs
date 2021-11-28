use crate::gfx::{BlendStateInfo, DepthStencilStateInfo, PrimitiveTopology, RasterizerStateInfo};

use super::super::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use super::super::{Device, Shader};

pub struct Pipeline<'a> {
    _device: &'a Device,
    _shader: &'a Shader<'a>,

    // Graphics
    _rasterizer_state_info_opt: Option<RasterizerStateInfo>,
    _depth_stencil_state_info_opt: Option<DepthStencilStateInfo>,
    _blend_state_opt: Option<wgpu::BlendState>,

    // Compute
    compute_pipeline_impl: Option<wgpu::ComputePipeline>,
}

impl<'a> IPipelineImpl<'a> for Pipeline<'a> {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        Self {
            _device: device,
            _rasterizer_state_info_opt: Some(info.get_rasterizer_state().clone()),
            _depth_stencil_state_info_opt: Some(info.get_depth_stencil_state().clone()),
            _blend_state_opt: Some(info.get_blend_state_info().to_wgpu()),
            compute_pipeline_impl: None,
            _shader: info.get_shader(),
        }
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
            _device: device,
            _rasterizer_state_info_opt: None,
            _depth_stencil_state_info_opt: None,
            _blend_state_opt: None,
            compute_pipeline_impl: Some(compute_pipeline),
            _shader: info.get_shader(),
        }
    }
}

impl<'a> Pipeline<'a> {
    pub fn create_primitive_state(&self, _index_format: wgpu::IndexFormat) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            ..Default::default()
        }
    }

    pub fn create_depth_stencil_state(
        &self,
        texture_format: wgpu::TextureFormat,
    ) -> wgpu::DepthStencilState {
        wgpu::DepthStencilState {
            format: texture_format,
            depth_write_enabled: self
                ._depth_stencil_state_info_opt
                .as_ref()
                .unwrap()
                .is_depth_write_enabled(),
            depth_compare: wgpu::CompareFunction::LessEqual,
            stencil: wgpu::StencilState::default(),
            bias: wgpu::DepthBiasState::default(),
        }
    }

    pub fn is_compute(&self) -> bool {
        self.compute_pipeline_impl.is_some()
    }

    pub fn get_compute_pipeline(&'a self) -> Option<&wgpu::ComputePipeline> {
        Some(self.compute_pipeline_impl.as_ref().unwrap())
    }

    pub fn get_shader(&self) -> &Shader {
        self._shader
    }
}

impl<'a> BlendStateInfo<'a> {
    pub fn to_wgpu(&self) -> wgpu::BlendState {
        wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::SrcAlpha,
                dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent::REPLACE,
        }
    }
}

impl PrimitiveTopology {
    pub fn to_wgpu(&self) -> wgpu::PrimitiveTopology {
        match &self {
            PrimitiveTopology::TriangleList => wgpu::PrimitiveTopology::TriangleList,
            PrimitiveTopology::PointList => wgpu::PrimitiveTopology::PointList,
        }
    }
}
