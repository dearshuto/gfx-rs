use crate::{enums::AttributeFormat, IDevice};

#[derive(Clone)]
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

#[derive(Clone)]
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

pub struct VertexStateInfo {
    _attribute_state_info_array: Vec<VertexAttributeStateInfo>,
    _buffer_state_info_array: Vec<VertexBufferStateInfo>,
}

impl VertexStateInfo {
    pub fn new() -> Self {
        Self {
            _attribute_state_info_array: Vec::new(),
            _buffer_state_info_array: Vec::new(),
        }
    }

    pub fn get_attribute_state_info_array(&self) -> &[VertexAttributeStateInfo] {
        &self._attribute_state_info_array
    }

    pub fn set_attribute_state_info_array<TIterator>(
        mut self,
        attribute_state_infos: TIterator,
    ) -> Self
    where
        TIterator: IntoIterator<Item = VertexAttributeStateInfo>,
    {
        self._attribute_state_info_array.clear();
        self._attribute_state_info_array
            .extend(attribute_state_infos);
        self
    }

    pub fn get_buffer_state_info_array(&self) -> &[VertexBufferStateInfo] {
        &self._buffer_state_info_array
    }

    pub fn set_buffer_state_info_array<TIterator>(mut self, buffer_state_infos: TIterator) -> Self
    where
        TIterator: IntoIterator<Item = VertexBufferStateInfo>,
    {
        self._buffer_state_info_array.clear();
        self._buffer_state_info_array.extend(buffer_state_infos);
        self
    }
}

pub trait IVertexState<'a> {
    type DeviceType: IDevice;

    fn new(device: &'a Self::DeviceType, info: &VertexStateInfo) -> Self;
}
