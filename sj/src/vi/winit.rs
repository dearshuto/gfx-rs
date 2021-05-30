pub struct Display {
}

impl Display {
    pub fn new() -> Self {
		Self{}
    }
}

pub struct Layer {
	_event_loop: winit::event_loop::EventLoop<()>,
    _window: winit::window::Window,
}

impl Layer {
    pub fn get_event_loop(&self) -> &winit::event_loop::EventLoop<()> {
        &self._event_loop
    }

    pub fn get_event_loop_mut(&mut self) -> &mut winit::event_loop::EventLoop<()> {
        &mut self._event_loop
    }

	pub fn get_window(&self) -> &winit::window::Window {
		&self._window
	}
}

pub fn create_display() -> Display {
    Display::new()
}

pub fn create_layer(_display: &mut Display) -> Layer {
	let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
    // .with_title("Hello Window")
    // .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();
	
    Layer {		
        _event_loop: event_loop,
		_window: window,
    }
}
