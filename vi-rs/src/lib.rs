// #[cfg(any(target_arch = "wasm32"))]
// pub mod glutin;

pub mod winit;

pub mod web_sys;
use sjgfx_interface::IDisplay;

pub trait IInstance {
    type DisplayId: Eq + PartialEq + Clone;
    type Display: IDisplay;

    fn new() -> Self;

    fn create_display(&mut self) -> Self::DisplayId;

    fn try_get_display(&self, id: &Self::DisplayId) -> Option<&Self::Display>;

    fn try_update(&mut self) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
