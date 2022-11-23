pub trait IDisplay {
    fn is_redraw_requested(&self) -> bool;

    fn listen<TListener: IDisplayEventListener>(&self, listener: &mut TListener);

    fn get_scale_factor(&self) -> f64;
}

pub trait IDisplayEventListener {
    fn on_resized(&mut self, _width: u32, _height: u32) {}
}
