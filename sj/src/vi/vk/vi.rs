extern crate winit;

pub struct Display
{
    event_loop: winit::event_loop::EventLoop<()>,
    _window : winit::window::Window,
}

impl Display
{
    pub fn new() -> Display
    {
	let event_loop = winit::event_loop::EventLoop::new();
	let window = winit::window::WindowBuilder::new()
            .with_title("A fantastic window!")
            .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
            .build(&event_loop)
            .unwrap();
	Display{event_loop, _window: window}
    }

    pub fn get_event_loop(&self) -> &winit::event_loop::EventLoop<()>
    {
	&self.event_loop
    }
}

pub struct Layer<'a>
{
    event_loop: &'a winit::event_loop::EventLoop<()>,
}

impl <'a> Layer<'a>
{
    pub fn get_event_loop(&self) -> &winit::event_loop::EventLoop<()>
    {
	&self.event_loop
    }
}

pub fn create_display() -> Display
{
    Display::new()
}

pub fn create_layer(display: &Display) -> Layer
{
    Layer{
	event_loop: display.get_event_loop()
    }
}

