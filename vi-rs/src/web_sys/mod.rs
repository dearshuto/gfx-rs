use std::collections::HashMap;

use wasm_bindgen::JsCast;

use crate::{IDisplay, IDisplayEventListener, IInstance};

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
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let id = DisplayId { id: canvas.id() };
        let display = Display { canvas, context };
        self.display_table.insert(id.clone(), display);
        id
    }

    pub fn try_get_display(&self, id: &DisplayId) -> Option<&Display> {
        self.display_table.get(id)
    }
}

impl IInstance for Instance {
    type DisplayId = DisplayId;
    type Display = Display;

    fn new() -> Self {
        Self::new()
    }

    fn create_display(&mut self) -> Self::DisplayId {
        self.create_display()
    }

    fn try_get_display(&self, id: &Self::DisplayId) -> Option<&Self::Display> {
        self.try_get_display(id)
    }

    fn try_update(&mut self) -> bool {
        false
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
pub struct DisplayId {
    id: String,
}

pub struct Display {
    pub canvas: web_sys::HtmlCanvasElement,
    pub context: web_sys::WebGl2RenderingContext,
}

impl IDisplay for Display {
    fn is_redraw_requested(&self) -> bool {
        true
    }

    fn listen<TListener: IDisplayEventListener>(&self, _listener: &mut TListener) {
        // todo!()
    }

    fn get_scale_factor(&self) -> f64 {
        1.0
    }
}
