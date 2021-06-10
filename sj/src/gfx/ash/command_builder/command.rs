use super::{
    ClearColorCommandBuilder, CopyImageToBufferCommandBuilder, DispatchParams, DrawCommandBuilder,
    EndRenderPassCommandBuilder, SetPipelineParams, SetRenderTargetsCommandBuilder,
    SetUnorderedAccessBufferParams, SetVertexBufferCommandBuilder,
    SetViewportScissorStateCommandBuilder,
};

pub enum Command<'a> {
    ClearColorCommand(ClearColorCommandBuilder<'a>),
    SetRenderTargets(SetRenderTargetsCommandBuilder<'a>),
    EndRenderTargets(EndRenderPassCommandBuilder<'a>),
    SetViewportScissorState(SetViewportScissorStateCommandBuilder<'a>),
    SetPipeline(SetPipelineParams<'a>),
    SetUnorderedAccessBuffer(SetUnorderedAccessBufferParams<'a>),
    SetVertexBuffer(SetVertexBufferCommandBuilder<'a>),
    Draw(DrawCommandBuilder<'a>),
    Dispatch(DispatchParams<'a>),
    CopyImageToBuffer(CopyImageToBufferCommandBuilder<'a>),
}

impl<'a> Command<'a> {
    pub fn build(&self) {
        match *self {
            Self::ClearColorCommand(ref builder) => builder.build(),
            Self::SetRenderTargets(ref builder) => builder.build(),
            Self::EndRenderTargets(ref builder) => builder.buld(),
            Self::SetViewportScissorState(ref builder) => builder.build(),
            Self::SetPipeline(ref params) => params.build(),
            Self::SetUnorderedAccessBuffer(ref params) => params.build(),
            Self::SetVertexBuffer(ref builder) => builder.build(),
            Self::Draw(ref builder) => builder.build(),
            Self::Dispatch(ref params) => params.build(),
            Self::CopyImageToBuffer(ref builder) => builder.build(),
        }
    }
}
