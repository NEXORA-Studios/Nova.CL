mod commands;
mod ipc;
mod lifecycle;
mod services;

use std::sync::Arc;
use tauri::{Manager, WindowEvent};

use crate::commands::*;
use crate::lifecycle::LifecycleManager;
#[allow(unused_imports)]
use crate::services::env::{DotEnvProvider, EnvVarProvider, HardcodedProvider};
use crate::services::{EnvService, HttpServerService, HttpService, LogService, SystemService};

#[tokio::main]
pub async fn run() {
    // 生成 Tauri 上下文
    let context = tauri::generate_context!();

    tauri::Builder::default()
        .manage(LifecycleManager::new())
        // 注册日志、HTTP客户端和HTTP服务器命令
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
            http_server_get_status,
            // 系统命令
            get_ram_info
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            let manager = app.state::<LifecycleManager>().inner().clone();

            // 根据模式注册不同的加载源
            let env_providers = {
                let mut providers: Vec<Arc<dyn crate::services::env::EnvProvider>> = Vec::new();
                // 硬编码字符串加载源 - 总是启用
                let hardcoded_provider = Arc::new(HardcodedProvider::from_pairs(&[
                    ("NOVA_VERSION", "{{NOVA_VERSION}}"),
                    ("NOVA_CHANNEL", "{{NOVA_CHANNEL}}"),
                    ("APP_ENCRYPTION_KEY", "{{APP_ENCRYPTION_KEY}}"),
                    ("OAUTH_REDIRECT_URI_BASE", "{{OAUTH_REDIRECT_URI_BASE}}"),
                    ("OAUTH_MS_CLIENT_ID", "{{OAUTH_MS_CLIENT_ID}}"),
                    ("OAUTH_MS_CLIENT_SECRET", "{{OAUTH_MS_CLIENT_SECRET}}"),
                    ("OAUTH_MS_REDIRECT_URI", "{{OAUTH_MS_REDIRECT_URI}}"),
                ]));
                providers.push(hardcoded_provider);
                // Debug 模式：启用 dotEnv 文件加载源
                #[cfg(debug_assertions)]
                {
                    let dotenv_provider = Arc::new(DotEnvProvider::new("../.env"));
                    providers.push(dotenv_provider);
                }
                // Release 模式：启用环境变量加载源
                #[cfg(not(debug_assertions))]
                {
                    let envvar_provider = Arc::new(EnvVarProvider);
                    providers.push(envvar_provider);
                }
                providers
            };

            // 注册服务
            manager.register(Arc::new(LogService::new()));
            manager.register(Arc::new(EnvService::new(env_providers)));
            manager.register(Arc::new(HttpService::new()));
            manager.register(Arc::new(SystemService::new()));
            manager.register(Arc::new(HttpServerService::new()));

            // 拉起生命周期管理
            tauri::async_runtime::spawn(async move {
                manager.startup(&app_handle).await;
            });

            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();

                // 提前 clone 所有需要跨 .await 使用的值
                let window = window.clone(); // 用于最后 close
                let app_handle = window.app_handle().clone(); // AppHandle 是 Clone + Send + 'static
                let manager = app_handle.state::<Arc<LifecycleManager>>().inner().clone(); // Arc 本身就是 Clone + Send + 'static

                tokio::spawn(async move {
                    // 这里三个变量都是 'static + Send 的
                    manager.shutdown(&app_handle).await;
                    // 异步安全关闭窗口
                    if let Err(e) = window.close() {
                        eprintln!("窗口关闭失败: {:?}", e);
                    }
                });
            }
        })
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .run(context)
        .expect("error while running tauri application");
}
