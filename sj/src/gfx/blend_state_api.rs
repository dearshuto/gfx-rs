use super::Device;

pub struct BlendStateInfo<'a> {
    _target_state_info: &'a [BlendTargetStateInfo],
}

pub struct BlendTargetStateInfo {}

impl<'a> BlendStateInfo<'a> {
    pub fn new() -> Self {
        Self {
            _target_state_info: &[],
        }
    }

    pub fn get_color_target_blend_state_info(&self) -> &'a [BlendTargetStateInfo] {
        self._target_state_info
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
