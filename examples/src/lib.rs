use std::sync::{Arc, Mutex};

use winit::{event_loop::{EventLoop, ControlFlow}, window::WindowBuilder, platform::run_return::EventLoopExtRunReturn, event::{Event, WindowEvent}};

pub fn test() {
    println!("LIB");
}

pub trait IAppLogic<'a> {
    fn new() -> Self;
    fn update(&mut self);
    fn draw(&'a mut self);
}

pub struct App;

impl App {
    pub fn run<'a, T: 'a + IAppLogic<'a>>() {
        let instance = Arc::new(Mutex::new(T::new()));

        let mut event_loop = EventLoop::new();
        let _window = WindowBuilder::new().build(&event_loop);

        let mut should_close = false;
        while !should_close {
            event_loop.run_return(|event, _, control_flow| {
                *control_flow = ControlFlow::Wait;

                // instance.update();
                // logic.update();
            match event {
                Event::RedrawRequested(_) => {
                    // instance.clone().lock().unwrap().draw();
                    // logic.draw();
                }
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    should_close = true;
                    *control_flow = ControlFlow::Exit
                }
                _ => {}
            }
            });
        }
    }
}
