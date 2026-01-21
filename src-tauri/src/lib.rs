mod app;
mod core;
mod plugin;
mod tauri_cmd;

pub fn run() {
    // 生成 Tauri 上下文
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .run(context)
        .expect("error while running tauri application");
}
