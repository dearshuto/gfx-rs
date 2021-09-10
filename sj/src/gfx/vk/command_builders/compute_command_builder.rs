use crate::gfx::common::command_builder::IComputeCommandBuilder;

pub struct ComputeCommandBuilder {}

impl<'a> IComputeCommandBuilder<'a> for ComputeCommandBuilder {
    fn build(&self) {
        todo!()
    }

    fn set_pipeline(&mut self, pipeline: &'a crate::gfx::Pipeline<'a>) {
        todo!()
    }

    fn set_constant_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: usize,
    ) {
        todo!()
    }

    fn set_unordered_access_buffer(
        &mut self,
        slot: i32,
        stage: crate::gfx::ShaderStage,
        gpu_address: &crate::gfx::GpuAddress,
        size: u64,
    ) {
        todo!()
    }

    fn dispatch(&mut self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        todo!()
    }
}
