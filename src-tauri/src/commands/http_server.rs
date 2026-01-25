use log::error;
use tauri::{AppHandle, Manager};

use crate::ipc::{CallRequest, CallResponse, IpcError};
use crate::lifecycle::{CommandInput, HttpStartArgs};
use crate::r#static::ErrCodes;

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
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和服务器启动参数
#[tauri::command]
pub async fn http_server_start(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpServerStartResult> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (port, lang) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let port = match obj.get("port") {
                    Some(serde_json::Value::Number(port)) => {
                        if let Some(port) = port.as_u64() {
                            port as u16
                        } else {
                            let error = IpcError {
                                code: ErrCodes::HttpServerStartInvalidPort,
                                module: "http_server".to_string(),
                                user_message: "启动服务器失败".to_string(),
                                dev_message: "Invalid port value".to_string(),
                                detail: None,
                                retryable: false,
                            };
                            return CallResponse::error(id, topic, error);
                        }
                    }
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpServerStartMissingParamPort,
                            module: "http_server".to_string(),
                            user_message: "启动服务器失败".to_string(),
                            dev_message: "Missing required parameter: port".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let lang = match obj.get("lang") {
                    Some(serde_json::Value::String(lang)) => lang.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpServerStartMissingParamLang,
                            module: "http_server".to_string(),
                            user_message: "启动服务器失败".to_string(),
                            dev_message: "Missing required parameter: lang".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                (port, lang)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpServerStartInvalidPayloadFormat,
                    module: "http_server".to_string(),
                    user_message: "启动服务器失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        }
        None => {
            let error = IpcError {
                code: ErrCodes::HttpServerStartMissingParamPortAndLang,
                module: "http_server".to_string(),
                user_message: "启动服务器失败".to_string(),
                dev_message: "Missing required parameters: port and lang".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

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
    match manager.invoke_command("http_server_start", CommandInput::HttpServerStart(start_args)).await {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStartResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, topic, result)
                    } else {
                        let error = IpcError {
                            code: ErrCodes::HttpServerStartParseResponseError,
                            module: "http_server".to_string(),
                            user_message: "解析服务器启动响应失败".to_string(),
                            dev_message: "Failed to parse http_server_start response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, topic, error)
                    }
                }
                _ => {
                    let error = IpcError {
                        code: ErrCodes::HttpServerStartInvalidResponse,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器启动响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_start command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, topic, error)
                }
            }
        }
        Err(e) => {
            error!("执行命令 http_server_start 失败: {:#?}", e);
            let error = IpcError {
                code: ErrCodes::HttpServerStartInvokeError,
                module: "http_server".to_string(),
                user_message: "启动服务器失败".to_string(),
                dev_message: format!("Failed to invoke http_server_start: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, topic, error)
        }
    }
}

/// 停止 HTTP 服务器
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID
#[tauri::command]
pub async fn http_server_stop(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpServerStopResult> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("http_server_stop", CommandInput::Args(vec![])).await {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStopResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, topic, result)
                    } else {
                        let error = IpcError {
                            code: ErrCodes::HttpServerStopParseResponseError,
                            module: "http_server".to_string(),
                            user_message: "解析服务器停止响应失败".to_string(),
                            dev_message: "Failed to parse http_server_stop response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, topic, error)
                    }
                }
                _ => {
                    let error = IpcError {
                        code: ErrCodes::HttpServerStopInvalidResponse,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器停止响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_stop command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, topic, error)
                }
            }
        }
        Err(e) => {
            error!("执行命令 http_server_stop 失败: {:#?}", e);
            let error = IpcError {
                code: ErrCodes::HttpServerStopInvokeError,
                module: "http_server".to_string(),
                user_message: "停止服务器失败".to_string(),
                dev_message: format!("Failed to invoke http_server_stop: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, topic, error)
        }
    }
}

/// 获取 HTTP 服务器状态
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID
#[tauri::command]
pub async fn http_server_status(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpServerStatusResult> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("http_server_status", CommandInput::Args(vec![])).await {
        Ok(output) => {
            match output {
                crate::lifecycle::CommandOutput::Json(json) => {
                    // 尝试解析为 HttpServerStatusResult
                    if let Ok(result) = serde_json::from_value(json) {
                        CallResponse::ok(id, topic, result)
                    } else {
                        let error = IpcError {
                            code: ErrCodes::HttpServerStatusParseResponseError,
                            module: "http_server".to_string(),
                            user_message: "解析服务器状态响应失败".to_string(),
                            dev_message: "Failed to parse http_server_status response".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        CallResponse::error(id, topic, error)
                    }
                }
                _ => {
                    let error = IpcError {
                        code: ErrCodes::HttpServerStatusInvalidResponse,
                        module: "http_server".to_string(),
                        user_message: "无效的服务器状态响应类型".to_string(),
                        dev_message: "Invalid response type from http_server_status command".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, topic, error)
                }
            }
        }
        Err(e) => {
            error!("执行命令 http_server_status 失败: {:#?}", e);
            let error = IpcError {
                code: ErrCodes::HttpServerStatusInvokeError,
                module: "http_server".to_string(),
                user_message: "获取服务器状态失败".to_string(),
                dev_message: format!("Failed to invoke http_server_status: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, topic, error)
        }
    }
}
