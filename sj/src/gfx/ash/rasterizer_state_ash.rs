use super::super::rasterizer_state_api::{IRasterizerStateImpl, RasterizerStateInfo};
use super::super::Device;

pub struct RasterizerStateImpl {
    _pipeline_rasterizer_state_create_info: ash::vk::PipelineRasterizationStateCreateInfo,
}

impl IRasterizerStateImpl for RasterizerStateImpl {
    fn new(_device: &Device, _info: RasterizerStateInfo) -> Self {
        let rasterization_state_create_info =
            ash::vk::PipelineRasterizationStateCreateInfo::builder()
                .front_face(ash::vk::FrontFace::COUNTER_CLOCKWISE)
                .line_width(1.0)
                .polygon_mode(ash::vk::PolygonMode::FILL)
                .cull_mode(ash::vk::CullModeFlags::NONE)
                .build();
        Self {
            _pipeline_rasterizer_state_create_info: rasterization_state_create_info,
        }
    }
}
