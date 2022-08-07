use sjgfx_interface::{CommandBufferInfo, DeviceInfo, ICommandBuffer, IDevice, IQueue, QueueInfo};
use sjgfx_wsys::{CommandBufferWsys, DeviceWsys, QueueWsys};

fn main() {
    let mut instance = sjvi::web_sys::Instance::new();
    let id = instance.create_display();
    let display = instance.try_get_display(&id).unwrap();

    let mut device = DeviceWsys::new_with_surface(&DeviceInfo::new(), &display);
    let mut queue = QueueWsys::new(&mut device, &QueueInfo::new());
    let mut command_buffer = CommandBufferWsys::new(&mut device, &CommandBufferInfo::new());

    command_buffer.begin();
    command_buffer.end();

    queue.execute(&command_buffer);
}
