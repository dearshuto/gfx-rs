use sjgfx_interface::{
    IVertexState, VertexAttributeStateInfo, VertexBufferStateInfo, VertexStateInfo,
};

use crate::api::IApi;

pub struct TVertexStateBuilder<T: IApi> {
    info: VertexStateInfo,
    _marker: std::marker::PhantomData<T>,
}

impl<T: IApi> TVertexStateBuilder<T> {
    pub fn new() -> Self {
        Self {
            info: VertexStateInfo::new(),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn build(&self, device: &T::Device) -> T::VertexState {
        T::VertexState::new(device, &self.info)
    }

    pub fn set_vertex_attribute_states<TAttributes>(
        self,
        vertex_attribute_state_infos: TAttributes,
    ) -> Self
    where
        TAttributes: Iterator<Item = VertexAttributeStateInfo>,
    {
        Self {
            info: self
                .info
                .set_attribute_state_info_array(vertex_attribute_state_infos),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn set_vertex_buffer_states<TAttributes>(
        self,
        vertex_buffer_state_infos: TAttributes,
    ) -> Self
    where
        TAttributes: Iterator<Item = VertexBufferStateInfo>,
    {
        Self {
            info: self
                .info
                .set_buffer_state_info_array(vertex_buffer_state_infos),
            _marker: std::marker::PhantomData,
        }
    }
}
