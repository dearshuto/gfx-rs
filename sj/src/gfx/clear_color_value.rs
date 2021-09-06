pub struct ClearColorValue {
    _value_float: [f32; 4],
    _value_int: [i32; 4],
    _value_uint: [u32; 4],
}

impl ClearColorValue {
    pub fn get_value_float(&self) -> &[f32; 4] {
        &self._value_float
    }

    pub fn get_value_flaot_mut(&mut self) -> &mut [f32; 4] {
        &mut self._value_float
    }

    pub fn get_value_int(&self) -> &[i32; 4] {
        &self._value_int
    }

    pub fn get_value_int_mut(&mut self) -> &mut [i32; 4] {
        &mut self._value_int
    }

    pub fn get_value_uint(&self) -> &[u32; 4] {
        &self._value_uint
    }

    pub fn get_value_uint_mut(&mut self) -> &mut [u32; 4] {
        &mut self._value_uint
    }
}
