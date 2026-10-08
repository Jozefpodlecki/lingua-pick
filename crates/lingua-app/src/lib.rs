mod setup;
mod commands;

use crate::commands::generate_handlers;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let mut builder = tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(setup::setup)
        .invoke_handler(generate_handlers());

    builder = setup::register_dependencies(builder);

    let app = builder
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    app.run(|_, _| {});
}
