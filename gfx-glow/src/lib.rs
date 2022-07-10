use winit::event_loop::EventLoop;

mod buffer_glow;
mod command_buffer_glow;
mod device_glow;
mod queue_glow;
mod shader_glow;
pub mod vi;

pub use buffer_glow::BufferGlow;
pub use command_buffer_glow::CommandBufferGlow;
pub use device_glow::DeviceGlow;
pub use queue_glow::QueueGlow;
pub use shader_glow::ShaderGlow;

static mut GLOW_STATIC_DATA: Option<StaticData> = None;

pub fn initialize() {
    unsafe { GLOW_STATIC_DATA = Some(StaticData::new()) }
}

pub fn finalize() {}

struct StaticData {
    pub event_loop: EventLoop<()>,
}

impl StaticData {
    pub fn new() -> Self {
        Self {
            event_loop: EventLoop::new(),
        }
    }
}

unsafe impl Sync for StaticData {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
