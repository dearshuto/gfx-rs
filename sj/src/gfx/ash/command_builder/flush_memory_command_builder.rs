use crate::gfx::{Device, GpuAccess};
use ash::version::DeviceV1_0;

pub struct FlushMemoryCommandBuilder<'a> {
    _device: &'a Device,
    _command_buffer: ash::vk::CommandBuffer,
    _src_access_mask: ash::vk::AccessFlags,
    _src_stage_mask: ash::vk::PipelineStageFlags,
}

impl<'a> FlushMemoryCommandBuilder<'a> {
    pub fn new(
        device: &'a Device,
        command_buffer: ash::vk::CommandBuffer,
        gpu_access: &GpuAccess,
    ) -> Self {
        let (src_access_mask, src_stage_mask) = gpu_access.to_ash();

        Self {
            _device: device,
            _command_buffer: command_buffer,
            _src_stage_mask: src_stage_mask,
            _src_access_mask: src_access_mask,
        }
    }

    pub fn build(&self) {
        let device_ash = self._device.to_data().get_device();
        let memory_barrier = ash::vk::MemoryBarrier::builder()
            .src_access_mask(self._src_access_mask)
            .dst_access_mask(ash::vk::AccessFlags::empty())
            .build();

        unsafe {
            device_ash.cmd_pipeline_barrier(
                self._command_buffer,
                self._src_stage_mask,
                ash::vk::PipelineStageFlags::TOP_OF_PIPE,
                ash::vk::DependencyFlags::empty(),
                &[memory_barrier],
                &[],
                &[],
            );
        }
    }
}

impl GpuAccess {
    pub fn to_ash(&self) -> (ash::vk::AccessFlags, ash::vk::PipelineStageFlags) {
        let mut src_access_mask = ash::vk::AccessFlags::empty();
        let mut src_stage_mask = ash::vk::PipelineStageFlags::empty();

        if self.contains(GpuAccess::READ) {
            src_stage_mask |= ash::vk::PipelineStageFlags::TRANSFER;
        }

        if self.contains(GpuAccess::WRITE) {
            src_access_mask |= ash::vk::AccessFlags::TRANSFER_WRITE;
            src_stage_mask |= ash::vk::PipelineStageFlags::TRANSFER;
        }

        if self.contains(GpuAccess::TEXTURE) {
            src_stage_mask |= ash::vk::PipelineStageFlags::VERTEX_SHADER
                // | ash::vk::PipelineStageFlags::GEOMETRY_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
                | ash::vk::PipelineStageFlags::FRAGMENT_SHADER
                | ash::vk::PipelineStageFlags::COMPUTE_SHADER;
        }

        if self.contains(GpuAccess::UNORDERED_ACCESS_BUFFER) {
            src_access_mask |= ash::vk::AccessFlags::SHADER_WRITE;
            src_stage_mask |= ash::vk::PipelineStageFlags::VERTEX_SHADER
                // | ash::vk::PipelineStageFlags::GEOMETRY_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
                | ash::vk::PipelineStageFlags::FRAGMENT_SHADER
                | ash::vk::PipelineStageFlags::COMPUTE_SHADER;
        }

        if self.contains(GpuAccess::COLOR_BUFFER) {
            src_access_mask |= ash::vk::AccessFlags::COLOR_ATTACHMENT_WRITE;
            src_stage_mask |= ash::vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT;
        }

        if self.contains(GpuAccess::DEPTH_STENCIL) {
            src_access_mask |= ash::vk::AccessFlags::DEPTH_STENCIL_ATTACHMENT_WRITE;
            src_stage_mask |= ash::vk::PipelineStageFlags::LATE_FRAGMENT_TESTS;
        }

        if self.contains(GpuAccess::IMAGE) {
            src_access_mask |= ash::vk::AccessFlags::SHADER_WRITE;
            src_stage_mask |= ash::vk::PipelineStageFlags::VERTEX_SHADER
                // | ash::vk::PipelineStageFlags::GEOMETRY_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_CONTROL_SHADER
                // | ash::vk::PipelineStageFlags::TESSELLATION_EVALUATION_SHADER
                | ash::vk::PipelineStageFlags::FRAGMENT_SHADER
                | ash::vk::PipelineStageFlags::COMPUTE_SHADER;
        }

        (src_access_mask, src_stage_mask)
    }
}
