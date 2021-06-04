use super::{AttributeFormat, Device};

pub struct VertexAttributeStateInfo {
    _format: AttributeFormat,
    _slot: i32,
    _buffer_index: i32,
    _offset: i64,
}

impl VertexAttributeStateInfo {
    pub fn new() -> Self {
        Self {
            _format: AttributeFormat::Float32_32,
            _slot: -1,
            _buffer_index: -1,
            _offset: -1,
        }
    }

    pub fn get_format(&self) -> &AttributeFormat {
        &self._format
    }

    pub fn set_format(mut self, format: AttributeFormat) -> Self {
        self._format = format;
        self
    }

    pub fn get_slot(&self) -> i32 {
        self._slot
    }

    pub fn set_slot(mut self, slot: i32) -> Self {
        self._slot = slot;
        self
    }

    pub fn get_buffer_index(&self) -> i32 {
        self._buffer_index
    }

    pub fn set_buffer_index(mut self, buffer_index: i32) -> Self {
        self._buffer_index = buffer_index;
        self
    }

    pub fn get_offset(&self) -> i64 {
        self._offset
    }

    pub fn set_offset(mut self, offset: i64) -> Self {
        self._offset = offset;
        self
    }
}

pub struct VertexBufferStateInfo {
    _stride: i64,
    _divisor: i64,
}

impl VertexBufferStateInfo {
    pub fn new() -> Self {
        Self {
            _stride: 0,
            _divisor: 1,
        }
    }

    pub fn get_stride(&self) -> i64 {
        self._stride
    }

    pub fn set_stride(mut self, stride: i64) -> Self {
        self._stride = stride;
        self
    }

    pub fn get_divisor(&self) -> i64 {
        self._divisor
    }

    pub fn set_divisor(mut self, divisor: i64) -> Self {
        self._divisor = divisor;
        self
    }
}

pub struct VertexStateInfo<'a> {
    _attribute_state_info_array: &'a [VertexAttributeStateInfo],
    _buffer_state_info_array: &'a [VertexBufferStateInfo],
}

impl<'a> VertexStateInfo<'a> {
    pub fn new() -> Self {
        Self {
            _attribute_state_info_array: &[],
            _buffer_state_info_array: &[],
        }
    }

    pub fn get_attribute_state_info_array(&self) -> &'a [VertexAttributeStateInfo] {
        self._attribute_state_info_array
    }

    pub fn set_attribute_state_info_array(
        mut self,
        attribute_state_info_array: &'a [VertexAttributeStateInfo],
    ) -> Self {
        self._attribute_state_info_array = attribute_state_info_array;
        self
    }

    pub fn get_buffer_state_info_array(&self) -> &'a [VertexBufferStateInfo] {
        self._buffer_state_info_array
    }

    pub fn set_buffer_state_info_array(
        mut self,
        buffer_state_info_array: &'a [VertexBufferStateInfo],
    ) -> Self {
        self._buffer_state_info_array = buffer_state_info_array;
        self
    }
}

pub trait IVertexState {
    fn new(device: &Device, info: &VertexStateInfo) -> Self;
}

pub struct TVertexState<T: IVertexState> {
    _impl: T,
}

impl<T: IVertexState> TVertexState<T> {
    pub fn new(device: &Device, info: &VertexStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
