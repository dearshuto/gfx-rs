use super::Device;

pub struct ViewportScissorStateInfo<'a> {
    _viewport_state_info_array: &'a [ViewportStateInfo],
    _scissor_state_info_array: &'a [ScissorStateInfo],
}

impl<'a> ViewportScissorStateInfo<'a> {
    pub fn new() -> Self {
        Self {
            _viewport_state_info_array: &[],
            _scissor_state_info_array: &[],
        }
    }

    pub fn get_viewport_state_info_array(&self) -> &[ViewportStateInfo] {
        self._viewport_state_info_array
    }

    pub fn set_viewport_state_info_array(
        mut self,
        viewport_state_info_array: &'a [ViewportStateInfo],
    ) -> Self {
        self._viewport_state_info_array = viewport_state_info_array;
        self
    }

    pub fn get_scissor_state_info_array(&self) -> &[ScissorStateInfo] {
        self._scissor_state_info_array
    }

    pub fn set_scissor_state_info_array(
        mut self,
        scissor_state_info_array: &'a [ScissorStateInfo],
    ) -> Self {
        self._scissor_state_info_array = scissor_state_info_array;
        self
    }
}

pub struct ViewportStateInfo {
    _origin_x: f32,
    _origin_y: f32,
    _width: f32,
    _height: f32,
}

impl ViewportStateInfo {
    pub fn new() -> Self {
        Self {
            _origin_x: 0.0,
            _origin_y: 0.0,
            _width: 0.0,
            _height: 0.0,
        }
    }

    pub fn get_origin_x(&self) -> f32 {
        self._origin_x
    }

    pub fn set_origin_x(mut self, origin_x: f32) -> Self {
        self._origin_x = origin_x;
        self
    }

    pub fn get_origin_y(&self) -> f32 {
        self._origin_y
    }

    pub fn set_origin_y(mut self, origin_y: f32) -> Self {
        self._origin_y = origin_y;
        self
    }

    pub fn get_width(&self) -> f32 {
        self._width
    }

    pub fn set_width(mut self, width: f32) -> Self {
        self._width = width;
        self
    }

    pub fn get_height(&self) -> f32 {
        self._height
    }

    pub fn set_height(mut self, height: f32) -> Self {
        self._height = height;
        self
    }
}

#[derive(Debug, Copy, Clone)]
pub struct ScissorStateInfo {
    _origin_x: i32,
    _origin_y: i32,
    _width: i32,
    _height: i32,
}

impl ScissorStateInfo {
    pub fn new() -> Self {
        Self {
            _origin_x: 0,
            _origin_y: 0,
            _width: 0,
            _height: 0,
        }
    }

    pub fn get_origin_x(&self) -> i32 {
        self._origin_x
    }

    pub fn set_origin_x(mut self, origin_x: i32) -> Self {
        self._origin_x = origin_x;
        self
    }

    pub fn get_origin_y(&self) -> i32 {
        self._origin_y
    }

    pub fn set_origin_y(mut self, origin_y: i32) -> Self {
        self._origin_y = origin_y;
        self
    }

    pub fn get_width(&self) -> i32 {
        self._width
    }

    pub fn set_width(mut self, width: i32) -> Self {
        self._width = width;
        self
    }

    pub fn get_height(&self) -> i32 {
        self._height
    }

    pub fn set_height(mut self, height: i32) -> Self {
        self._height = height;
        self
    }
}

pub trait IViewportScissorState<'a> {
    fn new(device: &'a Device, info: &ViewportScissorStateInfo) -> Self;
}

pub struct TViewportScissorState<'a, T: IViewportScissorState<'a>> {
    _impl: T,
    _marker: std::marker::PhantomData<&'a ()>,
}

impl<'a, T: IViewportScissorState<'a>> TViewportScissorState<'a, T> {
    pub fn new(device: &'a Device, info: &ViewportScissorStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
            _marker: std::marker::PhantomData,
        }
    }

    pub fn to_data(&self) -> &T {
        &self._impl
    }
}

#[cfg(test)]
mod tests {
    use crate::gfx::{
        Device, DeviceInfo, ScissorStateInfo, ViewportScissorState, ViewportScissorStateInfo,
        ViewportStateInfo,
    };

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let viewport_state_info = ViewportStateInfo::new()
            .set_origin_x(0.0)
            .set_origin_y(0.0)
            .set_width(640.0)
            .set_height(480.0);
        let scissor_state_info = ScissorStateInfo::new()
            .set_origin_x(0)
            .set_origin_y(0)
            .set_width(640)
            .set_height(480);
        let _viewport_scissor_state = ViewportScissorState::new(
            &device,
            &ViewportScissorStateInfo::new()
                .set_viewport_state_info_array(&[viewport_state_info])
                .set_scissor_state_info_array(&[scissor_state_info]),
        );
    }
}
