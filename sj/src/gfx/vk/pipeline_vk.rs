use crate::gfx::pipeline_api::{ComputePipelineInfo, GraphicsPipelineInfo, IPipelineImpl};
use crate::gfx::Device;
//use vulkano::pipeline::shader::SpecializationConstants;
//use vulkano::pipeline::vertex::SingleBufferDefinition;

type VkGraphicsPipeline<T> = vulkano::pipeline::GraphicsPipeline<T>;
use super::common::Float3232;

pub struct PipelineVk {
    //_graphics_pipeline:
    //Option<std::sync::Arc<VkGraphicsPipeline<SingleBufferDefinition<Float3232>>>>,
    _pipeline: Option<Box<dyn vulkano::pipeline::GraphicsPipelineAbstract>>,
    //_compute_pipeline: Option<std::sync::Arc<vulkano::pipeline::ComputePipeline>>,
}

impl<'a> IPipelineImpl<'a> for PipelineVk {
    fn new_as_graphics(device: &'a Device, info: &'a GraphicsPipelineInfo) -> Self {
        // let device_vk = device.to_data().get_device_impl();

        // // This definition describes the layout of this stage.
        // let vertex_layout = vulkano::pipeline::layout::PipelineLayoutDesc::new(
        //     // No descriptor sets.
        //     vec![],
        //     // No push constants.
        //     vec![],
        // )
        // .unwrap();
        // let vertex_input = unsafe {
        //     vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
        //         vulkano::pipeline::shader::ShaderInterfaceEntry {
        //             location: 1..2,
        //             format: vulkano::format::Format::R32G32B32Sfloat,
        //             name: Some(std::borrow::Cow::Borrowed("color")),
        //         },
        //     ])
        // };
        // let vertex_output = unsafe {
        //     vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
        //         vulkano::pipeline::shader::ShaderInterfaceEntry {
        //             location: 0..1,
        //             format: vulkano::format::Format::R32G32B32Sfloat,
        //             name: Some(std::borrow::Cow::Borrowed("v_color")),
        //         },
        //     ])
        // };
        // let vert_main = unsafe {
        //     info.get_shader()
        //         .to_data()
        //         .get_vertex_shader_module()
        //         .graphics_entry_point(
        //             std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0"),
        //             vertex_layout,
        //             <()>::descriptors(),
        //             vertex_input,
        //             vertex_output,
        //             vulkano::pipeline::shader::GraphicsShaderType::Vertex,
        //         )
        // };

        // // Same as with our vertex shader, but for fragment one instead.
        // let fragment_input = unsafe {
        //     vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
        //         vulkano::pipeline::shader::ShaderInterfaceEntry {
        //             location: 0..1,
        //             format: vulkano::format::Format::R32G32B32Sfloat,
        //             name: Some(std::borrow::Cow::Borrowed("v_color")),
        //         },
        //     ])
        // };

        // // Note that color fragment color entry will be determined
        // // automatically by Vulkano.
        // let fragment_output = unsafe {
        //     vulkano::pipeline::shader::ShaderInterface::new_unchecked(vec![
        //         vulkano::pipeline::shader::ShaderInterfaceEntry {
        //             location: 0..1,
        //             format: vulkano::format::Format::R32G32B32A32Sfloat,
        //             name: Some(std::borrow::Cow::Borrowed("f_color")),
        //         },
        //     ])
        // };

        // // Layout same as with vertex shader.
        // let fragment_layout = vulkano::pipeline::layout::PipelineLayoutDesc::new(
        //     // No descriptor sets.
        //     vec![],
        //     // No push constants.
        //     vec![],
        // )
        // .unwrap();
        // let frag_main = unsafe {
        //     info.get_shader()
        //         .to_data()
        //         .get_pixel_shader_module()
        //         .graphics_entry_point(
        //             std::ffi::CStr::from_bytes_with_nul_unchecked(b"main\0"),
        //             fragment_layout,
        //             <()>::descriptors(),
        //             fragment_input,
        //             fragment_output,
        //             vulkano::pipeline::shader::GraphicsShaderType::Fragment,
        //         )
        // };
        // let render_pass = std::sync::Arc::new(
        //     vulkano::single_pass_renderpass!(
        //         device_vk.clone(),
        //         attachments: {
        //             color: {
        //                 load: Clear,
        //                 store: Store,
        //                 format: vulkano::format::Format::R8Unorm, // TODO
        //                 samples: 1,
        //             }
        //         },
        //         pass: {
        //             color: [color],
        //             depth_stencil: {}
        //         }
        //     )
        //     .unwrap(),
        // );
        // let graphics_pipeline = std::sync::Arc::new(
        //     vulkano::pipeline::GraphicsPipeline::start()
        //         //.vertex_input(SingleBufferDefinition::<Vertex>::new())
        //         .vertex_shader(vert_main, ())
        //         .triangle_list()
        //         .viewports_dynamic_scissors_irrelevant(1)
        //         .fragment_shader(frag_main, ())
        //         .cull_mode_front()
        //         .front_face_counter_clockwise()
        //         .depth_stencil_disabled()
        //         .render_pass(vulkano::render_pass::Subpass::from(render_pass.clone(), 0).unwrap())
        //         .build(device_vk.clone())
        //         .unwrap(),
        // );

        Self {
            _pipeline: None,
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
