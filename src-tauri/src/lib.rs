mod env;
mod http;
mod logging;
mod system;
mod toml;

use crate::env::command::*;
use crate::http::command::*;
use crate::logging::command::*;
use crate::logging::logging::init_logging;
use crate::system::command::*;
use crate::toml::command::*;

pub fn run() {
    // 生成 Tauri 上下文
    let context = tauri::generate_context!();

    // 初始化日志系统
    if let Err(e) = init_logging(context.config()) {
        eprintln!("Failed to initialize logging: {}", e);
    }

    // 加载 .env 文件
    if let Err(e) = env::init_env() {
        log::error!(target: "env", "Failed to load .env file: {}", e);
    }

    // 初始化配置系统
    if let Err(e) = toml::init_config() {
        log::error!(target: "toml", "Failed to initialize config: {}", e);
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            http_request,
            http_get,
            http_post,
            http_put,
            http_delete,
            http_patch,
            http_server_start,
            http_server_stop,
            http_server_status,
            get_global_config_cmd,
            save_global_config_cmd,
            get_profile_config_cmd,
            save_profile_config_cmd,
            get_collection_config_cmd,
            save_collection_config_cmd,
            get_instance_config_cmd,
            save_instance_config_cmd,
            delete_instance_config_cmd,
            decrypt_string_cmd,
            encrypt_string_cmd,
            get_ram_info,
            // 环境变量相关命令
            get_env_var,
            get_all_env_vars,
            has_env_var,
            // 日志相关命令
            log_trace,
            log_debug,
            log_info,
            log_warn,
            log_error,
        ])
        .run(context)
        .expect("error while running tauri application");
}
