#![feature(derive_from)]

mod commands;
mod ipc;
mod lifecycle;
mod services;
mod r#static;

use std::sync::Arc;
use tauri::{Manager, WindowEvent};

use crate::commands::*;
use crate::lifecycle::LifecycleManager;
use crate::r#static::*;

#[allow(unused_imports)]
use crate::services::env::{DotEnvProvider, EnvProvider, EnvVarProvider, HardcodedProvider};
use crate::services::{ConfigService, EnvService, HttpServerService, HttpService, LogService, SystemService}; // 导入所有数据生成器

pub fn run() {
    // 生成 Tauri 上下文
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let _ = app.get_webview_window("main").expect("no main window").set_focus();
        }))
        .manage(LifecycleManager::new())
        .plugin(tauri_plugin_keyring::init())
        .invoke_handler(tauri::generate_handler![
            // 日志命令
            log_trace,
            log_debug,
            log_info,
            log_warn,
            log_error,
            // HTTP客户端命令
            http_request,
            http_get,
            http_post,
            http_put,
            http_delete,
            http_patch,
            // HTTP服务器命令
            http_server_start,
            http_server_stop,
            http_server_status,
            // 系统命令
            get_ram_info,
            // 配置命令
            get_config,
            set_config
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let manager = app.state::<LifecycleManager>().inner().clone();
            // 注册服务
            manager.register(Arc::new(LogService::new()));
            manager.register(Arc::new(EnvService::new((*ENV_PROVIDERS.clone()).to_vec())));
            manager.register(Arc::new(HttpService::new()));
            manager.register(Arc::new(SystemService::new()));
            manager.register(Arc::new(HttpServerService::new()));
            manager.register(Arc::new(ConfigService::new(app_handle.clone(), (*CONFIG_PROVIDERS.clone()).to_vec(), &*CONFIG_METADATA)));
            // 拉起生命周期管理
            tauri::async_runtime::spawn(async move {
                manager.startup(&app_handle).await;
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                // 先阻止默认关闭行为
                api.prevent_close();
                let window = window.clone();
                let app_handle = window.app_handle().clone(); // AppHandle 是 Clone + Send + 'static
                let manager = app_handle.state::<LifecycleManager>().inner().clone(); // Arc 本身就是 Clone + Send + 'static
                tauri::async_runtime::spawn(async move {
                    // 处理 Lifecycle 关闭
                    manager.shutdown(&app_handle).await;
                    // 安全退出
                    app_handle.exit(0);
                });
            }
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .run(context)
        .expect("error while running tauri application");
}
