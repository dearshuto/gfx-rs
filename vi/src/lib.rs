use std::{thread::sleep, time::Duration};

use winit::event::Event::{MainEventsCleared, RedrawRequested, WindowEvent};
use winit::event::WindowEvent::Resized;
use winit::{
    event_loop::{ControlFlow, EventLoop},
    platform::run_return::EventLoopExtRunReturn,
    window::{Window, WindowBuilder},
};

pub trait IDisplayEventListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}

pub struct Display<T: 'static> {
    pub window: Window,
    pub event_loop: EventLoop<T>,
    is_close_requested: bool,
    width: u32,
    height: u32,
}

impl<T> Display<T> {
    pub fn should_close(&self) -> bool {
        self.is_close_requested
    }

    pub fn update<TFunc: FnMut()>(&mut self, mut updater: TFunc) {
        self.event_loop.run_return(|event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            match event {
                RedrawRequested(..) => updater(),
                MainEventsCleared => {
                    *control_flow = ControlFlow::Exit;
                }
                WindowEvent { event, .. } => match event {
                    Resized(size) => {
                        self.width = size.width;
                        self.height = size.height;
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                        self.is_close_requested = true;
                    }
                    _ => {}
                },
                _ => {}
            }
        });

        self.window.request_redraw();
        sleep(Duration::from_millis(16));

    }

    pub fn listen<TListener: IDisplayEventListener>(&self, listener: &mut TListener) {
        listener.on_resized(self.width, self.height);
    }
}

pub fn create_display<T>(event_loop: EventLoop<T>) -> Display<T> {
    create_display_with_size(event_loop, 1280, 960)
}

pub fn create_display_with_size<T>(event_loop: EventLoop<T>, width: u32, height: u32) -> Display<T> {
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    Display {
        window,
        event_loop,
        is_close_requested: false,
        width,
        height
    }
}

struct DummyListener;
impl IDisplayEventListener for DummyListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
