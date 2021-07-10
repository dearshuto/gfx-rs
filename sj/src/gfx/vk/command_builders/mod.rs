mod clear_color_command_builder;
mod clear_depth_stencil_command_builder;
mod copy_image_command_builder;
mod copy_image_to_buffer_command_builder;
mod dispatch_command_builder;
mod draw_indexed_instanced_command_builder;
mod draw_instanced_command_builder;
mod set_constant_buffer_command_builder;
mod set_pipeline_command_builder;
mod set_render_targets_command_builder;
mod set_unordered_access_buffer_command_builder;
mod set_vertex_buffer_command_builder;
mod set_viewport_scissor_state_command_builder;

use vulkano::pipeline::vertex::VertexDefinition;

pub use self::clear_color_command_builder::ClearColorCommandBuilder;
pub use self::clear_depth_stencil_command_builder::ClearDepthStencilCommandBuilder;
pub use self::copy_image_command_builder::CopyImageCommandBuilder;
pub use self::copy_image_to_buffer_command_builder::CopyImageToBufferCommandBuilder;
pub use self::dispatch_command_builder::DispatchCommandBuilder;
pub use self::draw_indexed_instanced_command_builder::DrawIndexedInstancedCommandBuilder;
pub use self::draw_instanced_command_builder::DrawInstancedCommandBuilder;
pub use self::set_constant_buffer_command_builder::SetConstnatBufferCommandBuilder;
pub use self::set_pipeline_command_builder::SetPipelineCommandBuilder;
pub use self::set_render_targets_command_builder::SetRenderTargetsCommandBuilder;
pub use self::set_unordered_access_buffer_command_builder::SetUnorderedAccessBufferCommandBuilder;
pub use self::set_vertex_buffer_command_builder::SetVertexBufferCommandBuilder;
pub use self::set_viewport_scissor_state_command_builder::SetViewportScissorStateBuilder;

pub type VkAutoCommandBufferBuilder = vulkano::command_buffer::AutoCommandBufferBuilder<
    vulkano::command_buffer::PrimaryAutoCommandBuffer<
        vulkano::command_buffer::pool::standard::StandardCommandPoolAlloc,
    >,
>;

pub enum Command {
    SetViewportScissorState(SetViewportScissorStateBuilder),
    SetPipeline(SetPipelineCommandBuilder),
    SetConstantBuffer(SetConstnatBufferCommandBuilder),
    SetUnorderedAccessBuffer(SetUnorderedAccessBufferCommandBuilder),
    DrawInstanced(DrawInstancedCommandBuilder),
    DrawIndexedInstanced(DrawIndexedInstancedCommandBuilder),
    Dispatch(DispatchCommandBuilder),
    ClearColor(ClearColorCommandBuilder),
    ClearDepthStencil(ClearDepthStencilCommandBuilder),
    CopyImage(CopyImageCommandBuilder),
    CopyImageToBuffer(CopyImageToBufferCommandBuilder),
    SetRenderTargets(SetRenderTargetsCommandBuilder),
    SetVertexBuffer(SetVertexBufferCommandBuilder),
}

impl Command {
    pub fn build(&self, command_builder: VkAutoCommandBufferBuilder) -> VkAutoCommandBufferBuilder {
        match &self {
            Command::SetViewportScissorState(ref builder) => builder.build(command_builder),
            Command::SetPipeline(ref builder) => builder.build(command_builder),
            Command::SetConstantBuffer(ref builder) => builder.build(command_builder),
            Command::SetUnorderedAccessBuffer(ref builder) => builder.build(command_builder),
            Command::ClearColor(ref builder) => builder.build(command_builder),
            Command::ClearDepthStencil(ref builder) => builder.build(command_builder),
            Command::CopyImage(ref builder) => builder.build(command_builder),
            Command::CopyImageToBuffer(ref builder) => builder.build(command_builder),
            Command::DrawInstanced(ref builder) => builder.build(command_builder),
            Command::DrawIndexedInstanced(ref builder) => builder.build(command_builder),
            Command::Dispatch(ref builder) => builder.build(command_builder),
            Command::SetRenderTargets(ref builder) => builder.build(command_builder),
            Command::SetVertexBuffer(ref builder) => builder.build(command_builder),
        }
    }
}
