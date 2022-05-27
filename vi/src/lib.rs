use std::{thread::sleep, time::Duration};

use winit::dpi::PhysicalSize;
use winit::event::Event::{MainEventsCleared, RedrawRequested, WindowEvent};
use winit::event::WindowEvent::Resized;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub trait IResizeEventListener
{
    fn on_resized(&mut self);
}

pub struct Display<T: 'static> {
    pub window: Window,
    pub event_loop: EventLoop<T>,
    is_close_requested: bool,
    event_callbacks: [Option<dyn IResizeEventListener>; 8],
}

impl<T> Display<T> {
    pub fn should_close(&self) -> bool {
        self.is_close_requested
    }

    pub fn update<TFunc: FnMut()>(&mut self, mut updater: TFunc) {
        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            if let WindowEvent { event, .. } = &event {
                // Print only Window events to reduce noise
                println!("{:?}", event);
            }
            match event {
                RedrawRequested(..) => updater(),
                MainEventsCleared => {
                    self.window.request_redraw();
                }
                WindowEvent { event, .. } => match event {
                    Resized(_size) => {}
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        self.is_close_requested = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        sleep(Duration::from_millis(16));
    }
}

pub fn create_display<T>(event_loop: EventLoop<T>) -> Display<T> {
    let window = WindowBuilder::new()
    .with_inner_size(PhysicalSize::new(1280, 960))
    .build(&event_loop)
    .unwrap();

    Display {
        window,
        event_loop,
        is_close_requested: false,
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
