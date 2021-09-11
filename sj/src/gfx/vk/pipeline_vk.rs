use crate::gfx::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use crate::gfx::{DepthStencilStateInfo, Device, RasterizerStateInfo};
use vulkano::pipeline::shader::{GraphicsEntryPoint, SpecializationConstants};
//use vulkano::pipeline::vertex::SingleBufferDefinition;

type VkGraphicsPipeline<T> = vulkano::pipeline::GraphicsPipeline<T>;
//use super::common::Float3232;

pub struct PipelineVk<'a> {
    _rasterizer_state_info: Option<RasterizerStateInfo>,
    _depth_stencil_state_info: Option<DepthStencilStateInfo>,
    _vertex_entry_point: Option<GraphicsEntryPoint<'a>>,
	_pixel_entry_point: Option<GraphicsEntryPoint<'a>>,
    //Option<std::sync::Arc<VkGraphicsPipeline<SingleBufferDefinition<Float3232>>>>,
    //_compute_pipeline: Option<std::sync::Arc<vulkano::pipeline::ComputePipeline>>,	
    _is_graphics: bool,
}

impl<'a> IPipelineImpl<'a> for PipelineVk<'a> {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        let device_vk = device.to_data().get_device_impl();

        // This definition describes the layout of this stage.
        let vertex_layout = vulkano::pipeline::layout::PipelineLayout::new(
            device_vk.clone(),
            // No descriptor sets.
            vec![],
            // No push constants.
            vec![],
        )
        .unwrap();
        let vertex_input = unsafe {
            vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
                vulkano::pipeline::shader::ShaderInterfaceEntry {
                    location: 1..2,
                    format: vulkano::format::Format::R32G32B32Sfloat,
                    name: Some(std::borrow::Cow::Borrowed("color")),
                },
            ])
        };
        let vertex_output = unsafe {
            vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
                vulkano::pipeline::shader::ShaderInterfaceEntry {
                    location: 0..1,
                    format: vulkano::format::Format::R32G32B32Sfloat,
                    name: Some(std::borrow::Cow::Borrowed("v_color")),
                },
            ])
        };
        let vert_main = unsafe {
            info.get_shader()
                .to_data()
                .get_vertex_shader_module()
                .graphics_entry_point(
                    std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0"),
                    [],
                    None,
                    <()>::descriptors(),
                    vertex_input,
                    vertex_output,
                    vulkano::pipeline::shader::GraphicsShaderType::Vertex,
                )
        };

        // Same as with our vertex shader, but for fragment one instead.
        let fragment_input = unsafe {
            vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
                vulkano::pipeline::shader::ShaderInterfaceEntry {
                    location: 0..1,
                    format: vulkano::format::Format::R32G32B32Sfloat,
                    name: Some(std::borrow::Cow::Borrowed("v_color")),
                },
            ])
        };

        // Note that color fragment color entry will be determined
        // automatically by Vulkano.
        let fragment_output = unsafe {
            vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
                vulkano::pipeline::shader::ShaderInterfaceEntry {
                    location: 0..1,
                    format: vulkano::format::Format::R32G32B32A32Sfloat,
                    name: Some(std::borrow::Cow::Borrowed("f_color")),
                },
            ])
        };

        // Layout same as with vertex shader.
        let frag_main = unsafe {
            info.get_shader()
                .to_data()
                .get_pixel_shader_module()
                .graphics_entry_point(
                    std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0"),
                    [],
					None,
                    <()>::descriptors(),
                    fragment_input,
                    fragment_output,
                    vulkano::pipeline::shader::GraphicsShaderType::Fragment,
                )
        };

        Self {
            _rasterizer_state_info: Some(*info.get_rasterizer_state()),
            _depth_stencil_state_info: Some(*info.get_depth_stencil_state()),
            _vertex_entry_point: Some(vert_main),
			_pixel_entry_point: Some(frag_main),
            _is_graphics: true,
            //_graphics_pipeline: None, //Some(graphics_pipeline),
            //_compute_pipeline: None,
        }
    }

    fn new_as_compute(device: &'a Device, info: ComputePipelineInfo<'a>) -> Self {
        todo!();
        // let device_vk = device.to_data().get_device_impl();
        // // Layout same as with vertex shader.
        // let compute_layout = vulkano::pipeline::layout::PipelineLayout::new(
        //     // No descriptor sets.
        //     vec![],
        //     // No push constants.
        //     vec![],
        // )
        // .unwrap();
        // let entry_point = unsafe {
        //     info.get_shader()
        //         .to_data()
        //         .get_compute_shader_module()
        //         .compute_entry_point(
        //             std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0"),
        //             compute_layout,
        //             <()>::descriptors(),
        //         )
        // };
        // let compute_pipeline =
        //     vulkano::pipeline::ComputePipeline::new(device_vk.clone(), &entry_point, &(), None)
        //         .unwrap();

        // Self {
        //     _graphics_pipeline: None,
        //     _compute_pipeline: Some(std::sync::Arc::new(compute_pipeline)),
        // }
    }
}

impl<'a> PipelineVk<'a> {
    pub fn is_graphics(&self) -> bool {
        self._is_graphics
    }

    pub fn get_rasterizer_state_info(&self) -> &RasterizerStateInfo {
        &self._rasterizer_state_info.as_ref().unwrap()
    }

    pub fn get_depth_stencil_state_info(&self) -> &DepthStencilStateInfo {
        &self._depth_stencil_state_info.as_ref().unwrap()
    }

	pub fn clone_vertex_entry_point(&self) -> GraphicsEntryPoint {
		self._vertex_entry_point.as_ref().unwrap().clone()
	}

	pub fn clone_pixel_entry_point(&self) -> GraphicsEntryPoint {
		self._pixel_entry_point.as_ref().unwrap().clone()
	}	
}
