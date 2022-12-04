mod my_plugin;
use tauri::RunEvent;

use my_plugin::MyPluginBuilder;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            app.wry_plugin(MyPluginBuilder::new());
            Ok(())
        })
        .build(tauri::generate_context!(
            "examples/tauri_wgpu/tauri.conf.json"
        ))
        .unwrap()
        .run(move |_app, event| {
            if let RunEvent::WindowEvent { label, event, .. } = event {
                println!("{} {:?}", label, event);
            }
        });
}
