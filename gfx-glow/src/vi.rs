use glutin::dpi::PhysicalSize;
use glutin::event_loop::ControlFlow;
use glutin::window::Window;
use glutin::{ContextWrapper, PossiblyCurrent};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use uuid::Uuid;
use winit::event::Event::{MainEventsCleared, RedrawRequested};
use winit::platform::run_return::EventLoopExtRunReturn;

pub struct Display {
    pub window: ContextWrapper<PossiblyCurrent, Window>,
}

impl Display {
    pub fn new() -> Self {
        let event_loop = unsafe { &crate::GLOW_STATIC_DATA.as_ref().unwrap().event_loop };
        let window_builder =
            winit::window::WindowBuilder::new().with_inner_size(PhysicalSize::new(640, 480));
        let window = unsafe {
            glutin::ContextBuilder::new()
                // .with_shared_lists(other)
                // .with_depth_buffer(native_options.depth_buffer)
                // .with_multisampling(native_options.multisampling)
                // .with_srgb(true)
                // .with_stencil_buffer(native_options.stencil_buffer)
                // .with_vsync(native_options.vsync)
                .build_windowed(window_builder, &event_loop)
                .unwrap()
                .make_current()
                .unwrap()
        };

        Self { window }
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)]
pub struct DisplayId {
    id: Uuid,
}

impl DisplayId {
    pub fn new() -> Self {
        Self { id: Uuid::new_v4() }
    }
}

pub struct Instance {
    display_table: HashMap<DisplayId, Display>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            display_table: HashMap::new(),
        }
    }

    pub fn create_display(&mut self) -> DisplayId {
        let id = DisplayId::new();
        let display = Display::new();
        self.display_table.insert(id.clone(), display);
        id
    }

    pub fn try_get_display(&self, id: DisplayId) -> Option<&Display> {
        self.display_table.get(&id)
    }

    pub fn make_current(&mut self, id: DisplayId) {
        let display = self.display_table.remove(&id).unwrap();
        let display = Display {
            window: unsafe { display.window.make_current() }.unwrap(),
        };
        self.display_table.insert(id, display);
    }

    pub fn should_update(&mut self) -> bool {
        for display in self.display_table.values_mut() {
            display.window.swap_buffers().unwrap();
        }

        let event_loop = unsafe { &mut crate::GLOW_STATIC_DATA.as_mut().unwrap().event_loop };
        event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                RedrawRequested(_window_id) => {}
                MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {}
            }
        });

        sleep(Duration::from_millis(16));
        return true;
    }
}
