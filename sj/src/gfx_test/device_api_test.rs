#[cfg(test)]
mod tests {
    use super::super::super::gfx::{Device, DeviceInfo};

    #[test]
    fn initialize() {
        let _device = Device::new(&DeviceInfo::new());
    }
}
