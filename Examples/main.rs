extern crate sjgfx_rs;

fn main() {
    let device_info = sjgfx_rs::gfx::DeviceInfo {};

    // OpenGL
    let mut device_gl = sjgfx_rs::gfx::DeviceGl {};
    device_gl.initialize(&device_info);
    device_gl.finalize();

    // Vulkan
    let mut device_vk = sjgfx_rs::gfx::DeviceVk {};
    device_vk.initialize(&device_info);
    device_vk.finalize();
}
