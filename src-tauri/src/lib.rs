mod http;

use crate::http::command::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_window_state::Builder::new().build())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            http_request,
            http_get,
            http_post,
            http_put,
            http_delete,
            http_patch
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
