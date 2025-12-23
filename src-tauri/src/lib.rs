mod http;
mod toml;

use crate::http::command::*;
use crate::toml::command::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // 初始化配置系统
    if let Err(e) = toml::init_config() {
        eprintln!("Failed to initialize config: {}", e);
    }
    
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
            http_patch,
            get_global_config_cmd,
            save_global_config_cmd,
            get_profile_config_cmd,
            save_profile_config_cmd,
            get_instance_config_cmd,
            save_instance_config_cmd,
            delete_instance_config_cmd,
            list_instance_configs_cmd,
            decrypt_string_cmd,
            encrypt_string_cmd,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
