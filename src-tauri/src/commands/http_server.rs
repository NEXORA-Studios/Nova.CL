use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::lifecycle::{CommandInput, CommandOutput, HttpStartArgs, LifecycleManager};

/// HTTP 服务器启动结果
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpServerStartResult {
    pub status: String,
    pub message: String,
    pub port: u16,
}

/// HTTP 服务器停止结果
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpServerStopResult {
    pub status: String,
    pub message: String,
}

/// HTTP 服务器状态结果
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpServerStatusResult {
    pub status: String,
    pub port: Option<u16>,
}

/// HTTP 错误
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpServerError {
    pub message: String,
    pub code: Option<u16>,
}

/// 启动 HTTP 服务器
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `port`: 服务器端口
/// - `lang`: 服务器语言
#[tauri::command]
pub async fn http_server_start(
    app_handle: AppHandle,
    port: u16,
    lang: String,
) -> Result<HttpServerStartResult, HttpServerError> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 构建命令参数
    let start_args = CommandInput::HttpServerStart(HttpStartArgs {
        app_handle: app_handle.clone(),
        port,
        lang,
    });

    // 调用生命周期管理器的命令
    match manager
        .invoke_command(
            "http_server_start",
            start_args,
        )
        .await
    {
        Ok(output) => {
            match output {
                CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStartResult
                    if let Ok(result) = serde_json::from_value(json) {
                        Ok(result)
                    } else {
                        Err(HttpServerError {
                            message: "Failed to parse http_server_start response".to_string(),
                            code: None,
                        })
                    }
                }
                _ => Err(HttpServerError {
                    message: "Invalid response type from http_server_start command".to_string(),
                    code: None,
                }),
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_start: {:?}", e);
            Err(HttpServerError {
                message: format!("Failed to invoke http_server_start: {:?}", e),
                code: None,
            })
        }
    }
}

/// 停止 HTTP 服务器
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
#[tauri::command]
pub async fn http_server_stop(
    app_handle: AppHandle,
) -> Result<HttpServerStopResult, HttpServerError> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_server_stop", CommandInput::Args(vec![]))
        .await
    {
        Ok(output) => {
            match output {
                CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStopResult
                    if let Ok(result) = serde_json::from_value(json) {
                        Ok(result)
                    } else {
                        Err(HttpServerError {
                            message: "Failed to parse http_server_stop response".to_string(),
                            code: None,
                        })
                    }
                }
                _ => Err(HttpServerError {
                    message: "Invalid response type from http_server_stop command".to_string(),
                    code: None,
                }),
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_stop: {:?}", e);
            Err(HttpServerError {
                message: format!("Failed to invoke http_server_stop: {:?}", e),
                code: None,
            })
        }
    }
}

/// 获取 HTTP 服务器状态
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
#[tauri::command]
pub async fn http_server_get_status(
    app_handle: AppHandle,
) -> Result<HttpServerStatusResult, HttpServerError> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_server_get_status", CommandInput::Args(vec![]))
        .await
    {
        Ok(output) => {
            match output {
                CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStatusResult
                    if let Ok(result) = serde_json::from_value(json) {
                        Ok(result)
                    } else {
                        Err(HttpServerError {
                            message: "Failed to parse http_server_get_status response".to_string(),
                            code: None,
                        })
                    }
                }
                _ => Err(HttpServerError {
                    message: "Invalid response type from http_server_get_status command"
                        .to_string(),
                    code: None,
                }),
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_get_status: {:?}", e);
            Err(HttpServerError {
                message: format!("Failed to invoke http_server_get_status: {:?}", e),
                code: None,
            })
        }
    }
}
