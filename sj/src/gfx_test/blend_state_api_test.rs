#[cfg(test)]
mod tests {
    use super::super::super::gfx::{
        BlendState, BlendStateInfo, BlendTargetStateInfo, Device, DeviceInfo,
    };

    #[test]
    fn initialize() {
        let device = Device::new(&DeviceInfo::new());
        let blend_target_state_info_array = [BlendTargetStateInfo::new()];
        let blend_state_info =
            BlendStateInfo::new().set_target_state_info(&blend_target_state_info_array);
        let _blend_state = BlendState::new(&device, &blend_state_info);
    }
}
