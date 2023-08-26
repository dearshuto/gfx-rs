pub fn main() {
    sjgfx_ash::initialize();

    {
        let _device = sjgfx_ash::DeviceAsh::new(&sjgfx_interface::DeviceInfo::new());
    }
    sjgfx_ash::finalize();
}
