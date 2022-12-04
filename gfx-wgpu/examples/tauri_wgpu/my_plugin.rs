use tauri_runtime::UserEvent;
use tauri_runtime_wry::{Plugin, PluginBuilder};

pub struct MyPluginBuilder {}

impl MyPluginBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: UserEvent> PluginBuilder<T> for MyPluginBuilder {
    type Plugin = MyPlugin;

    fn build(self, _context: tauri_runtime_wry::Context<T>) -> Self::Plugin {
        MyPlugin::new()
    }
}

pub struct MyPlugin {}

impl MyPlugin {
    pub fn new() -> Self {
        Self {}
    }
}

impl<T: UserEvent> Plugin<T> for MyPlugin {
    fn on_event(
        &mut self,
        _event: &tauri_runtime_wry::wry::application::event::Event<tauri_runtime_wry::Message<T>>,
        _event_loop: &tauri_runtime_wry::wry::application::event_loop::EventLoopWindowTarget<
            tauri_runtime_wry::Message<T>,
        >,
        _proxy: &tauri_runtime_wry::wry::application::event_loop::EventLoopProxy<
            tauri_runtime_wry::Message<T>,
        >,
        _control_flow: &mut tauri_runtime_wry::wry::application::event_loop::ControlFlow,
        _context: tauri_runtime_wry::EventLoopIterationContext<'_, T>,
        _web_context: &tauri_runtime_wry::WebContextStore,
    ) -> bool {
        false
    }
}
