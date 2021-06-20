use super::{
    ClearColorCommandBuilder, CopyImageCommandBuilder, CopyImageToBufferCommandBuilder,
    DispatchParams, DrawCommandBuilder, EndRenderPassCommandBuilder, FlushMemoryCommandBuilder,
    SetConstantBufferCommandBuilder, SetPipelineParams, SetRenderTargetsCommandBuilder,
    SetTextureStateTransitionCommandBuilder, SetUnorderedAccessBufferParams,
    SetVertexBufferCommandBuilder, SetViewportScissorStateCommandBuilder,
};

pub enum Command<'a> {
    ClearColorCommand(ClearColorCommandBuilder<'a>),
    SetRenderTargets(SetRenderTargetsCommandBuilder<'a>),
    EndRenderTargets(EndRenderPassCommandBuilder<'a>),
    SetViewportScissorState(SetViewportScissorStateCommandBuilder<'a>),
    SetPipeline(SetPipelineParams<'a>),
    SetConstantBuffer(SetConstantBufferCommandBuilder<'a>),
    SetUnorderedAccessBuffer(SetUnorderedAccessBufferParams<'a>),
    SetVertexBuffer(SetVertexBufferCommandBuilder<'a>),
    Draw(DrawCommandBuilder<'a>),
    Dispatch(DispatchParams<'a>),
    SetTextureStateTransition(SetTextureStateTransitionCommandBuilder<'a>),
    CopyImage(CopyImageCommandBuilder<'a>),
    CopyImageToBuffer(CopyImageToBufferCommandBuilder<'a>),
    FlushMemory(FlushMemoryCommandBuilder<'a>),
}

impl<'a> Command<'a> {
    pub fn build(&self) {
        match *self {
            Self::ClearColorCommand(ref builder) => builder.build(),
            Self::SetRenderTargets(ref builder) => builder.build(),
            Self::EndRenderTargets(ref builder) => builder.buld(),
            Self::SetViewportScissorState(ref builder) => builder.build(),
            Self::SetPipeline(ref params) => params.build(),
            Self::SetConstantBuffer(ref builder) => builder.build(),
            Self::SetUnorderedAccessBuffer(ref params) => params.build(),
            Self::SetVertexBuffer(ref builder) => builder.build(),
            Self::Draw(ref builder) => builder.build(),
            Self::Dispatch(ref params) => params.build(),
            Self::CopyImage(ref builder) => builder.build(),
            Self::CopyImageToBuffer(ref builder) => builder.build(),
            Self::SetTextureStateTransition(ref builder) => builder.build(),
            Self::FlushMemory(ref builder) => builder.build(),
        }
    }
}
