use sjgfx_interface::QueueInfo;

pub fn main() {
    sjgfx_ash::initialize();

    {
        let device = sjgfx_ash::DeviceAsh::new(&sjgfx_interface::DeviceInfo::new());
        let _queue = sjgfx_ash::QueueAsh::new(&device, &QueueInfo::new());
    }
    sjgfx_ash::finalize();
}
