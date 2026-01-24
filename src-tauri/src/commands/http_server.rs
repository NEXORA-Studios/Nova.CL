use tauri::{AppHandle, Manager}; 

use crate::ipc::{CallResponse, IpcError};
use crate::lifecycle::{CommandInput, HttpStartArgs};

/// HTTP 服务器启动结果
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HttpServerStartResult {
    pub status: String,
    pub message: String,
    pub port: u16,
}

/// HTTP 服务器停止结果
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HttpServerStopResult {
    pub status: String,
    pub message: String,
}

/// HTTP 服务器状态结果
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct HttpServerStatusResult {
    pub status: String,
    pub port: Option<u16>,
}

/// 启动 HTTP 服务器
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `port`: 服务器端口
/// - `lang`: 服务器语言
#[tauri::command]
pub async fn http_server_start(app_handle: AppHandle, id: u64, port: u16, lang: String) -> CallResponse<HttpServerStartResult> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 构建命令参数
    let start_args = HttpStartArgs {
        app_handle: app_handle.clone(),
        port,
        lang,
    };

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_server_start", CommandInput::HttpServerStart(start_args))
        .await
    {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStartResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, "http_server_start".to_string(), result)
                    } else {
                        let error = IpcError {
                            code: 2001,
                            module: "http_server".to_string(),
                            user_message: "解析服务器启动响应失败".to_string(),
                            dev_message: "Failed to parse http_server_start response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, "http_server_start".to_string(), error)
                    }
                },
                _ => {
                    let error = IpcError {
                        code: 2002,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器启动响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_start command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, "http_server_start".to_string(), error)
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_start: {:?}", e);
            let error = IpcError {
                code: 2003,
                module: "http_server".to_string(),
                user_message: "启动服务器失败".to_string(),
                dev_message: format!("Failed to invoke http_server_start: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, "http_server_start".to_string(), error)
        },
    }
}

/// 停止 HTTP 服务器
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
#[tauri::command]
pub async fn http_server_stop(app_handle: AppHandle, id: u64) -> CallResponse<HttpServerStopResult> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_server_stop", CommandInput::Args(vec![]))
        .await
    {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStopResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, "http_server_stop".to_string(), result)
                    } else {
                        let error = IpcError {
                            code: 2004,
                            module: "http_server".to_string(),
                            user_message: "解析服务器停止响应失败".to_string(),
                            dev_message: "Failed to parse http_server_stop response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, "http_server_stop".to_string(), error)
                    }
                },
                _ => {
                    let error = IpcError {
                        code: 2005,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器停止响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_stop command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, "http_server_stop".to_string(), error)
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_stop: {:?}", e);
            let error = IpcError {
                code: 2006,
                module: "http_server".to_string(),
                user_message: "停止服务器失败".to_string(),
                dev_message: format!("Failed to invoke http_server_stop: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, "http_server_stop".to_string(), error)
        },
    }
}

/// 获取 HTTP 服务器状态
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
#[tauri::command]
pub async fn http_server_get_status(app_handle: AppHandle, id: u64) -> CallResponse<HttpServerStatusResult> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_server_get_status", CommandInput::Args(vec![]))
        .await
    {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStatusResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, "http_server_get_status".to_string(), result)
                    } else {
                        let error = IpcError {
                            code: 2007,
                            module: "http_server".to_string(),
                            user_message: "解析服务器状态响应失败".to_string(),
                            dev_message: "Failed to parse http_server_get_status response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, "http_server_get_status".to_string(), error)
                    }
                },
                _ => {
                    let error = IpcError {
                        code: 2008,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器状态响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_get_status command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, "http_server_get_status".to_string(), error)
                },
            }
        }
        Err(e) => {
            eprintln!("Failed to invoke http_server_get_status: {:?}", e);
            let error = IpcError {
                code: 2009,
                module: "http_server".to_string(),
                user_message: "获取服务器状态失败".to_string(),
                dev_message: format!("Failed to invoke http_server_get_status: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, "http_server_get_status".to_string(), error)
        },
    }
}
