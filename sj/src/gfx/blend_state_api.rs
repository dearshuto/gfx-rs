use super::Device;

pub struct BlendStateInfo<'a> {
    _target_state_info: &'a [BlendTargetStateInfo],
}

impl<'a> BlendStateInfo<'a> {
    pub fn new() -> Self {
        Self {
            _target_state_info: &[],
        }
    }

    pub fn get_color_target_blend_state_info(&self) -> &'a [BlendTargetStateInfo] {
        self._target_state_info
    }

    pub fn get_target_state_info(&self) -> &'a [BlendTargetStateInfo] {
        self._target_state_info
    }

    pub fn set_target_state_info(mut self, target_state_info: &'a [BlendTargetStateInfo]) -> Self {
        self._target_state_info = target_state_info;
        self
    }
}

pub struct BlendTargetStateInfo {}

impl BlendTargetStateInfo {
    pub fn new() -> Self {
        Self {}
    }
}

pub trait IBlendStateImpl {
    fn new(device: &Device, info: &BlendStateInfo) -> Self;
}

pub struct TBlendState<T>
where
    T: IBlendStateImpl,
{
    _impl: T,
}

impl<T: IBlendStateImpl> TBlendState<T> {
    pub fn new(device: &Device, info: &BlendStateInfo) -> Self {
        Self {
            _impl: T::new(device, info),
        }
    }
}
