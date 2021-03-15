pub mod gfx {
    pub struct DeviceInfo {}
    pub trait TDevice {}

    pub struct DeviceGl {}

    impl DeviceGl {
        pub fn initialize(&mut self, _info: &DeviceInfo) {}
        pub fn finalize(&mut self) {}
    }

    pub struct DeviceVk {}

    impl DeviceVk {
        pub fn initialize(&mut self, _info: &DeviceInfo) {}
        pub fn finalize(&mut self) {}
    }
}
