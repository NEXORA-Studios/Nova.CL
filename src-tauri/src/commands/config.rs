use log::trace;
use tauri::{AppHandle, Manager};

use crate::ipc::{CallRequest, CallResponse, IpcError};
use crate::lifecycle::{CommandInput, CommandOutput, LifecycleManager};
use crate::r#static::ErrCodes;

/// 获取配置值
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和配置项名称
#[tauri::command]
pub async fn get_config(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<String> {
    // 从 request 中提取 ID 和参数
    let id = request.id;
    let topic = request.topic;

    // 解析配置项名称
    let key = match request.payload {
        Some(payload) => {
            // 创建 payload 的副本用于操作
            let payload_copy = payload.clone();

            // 尝试从 payload 中获取 key 字段
            if let serde_json::Value::Object(obj) = payload {
                if let Some(serde_json::Value::String(key)) = obj.get("key") {
                    key.clone()
                } else {
                    // 尝试将整个 payload 作为 key
                    payload_copy.to_string()
                }
            } else {
                // 尝试将整个 payload 作为 key
                payload.to_string()
            }
        }
        None => {
            let error = IpcError {
                code: ErrCodes::ConfigGetConfigMissingParamKey,
                module: "config".to_string(),
                user_message: "获取配置失败".to_string(),
                dev_message: "Missing required parameter: key".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("get_config", CommandInput::Args(vec![key.clone()])).await {
        Ok(output) => match output {
            CommandOutput::Text(v) => {
                trace!("get_config key: {}, value: {}", key, v);
                CallResponse::ok(id, topic, v)
            }
            _ => {
                let error = IpcError {
                    code: ErrCodes::ConfigGetConfigInvalidResponse,
                    module: "config".to_string(),
                    user_message: "获取配置失败".to_string(),
                    dev_message: "Invalid response type from get_config command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, topic, error)
            }
        },
        Err(e) => {
            let error = IpcError {
                code: ErrCodes::ConfigGetConfigInvokeError,
                module: "config".to_string(),
                user_message: "获取配置失败".to_string(),
                dev_message: format!("Failed to invoke get_config: {:?}", e),
                detail: None,
                retryable: false,
            };
            CallResponse::error(id, topic, error)
        }
    }
}

/// 设置配置值
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、配置项名称和值
#[tauri::command]
pub async fn set_config(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<String> {
    // 从 request 中提取 ID 和参数
    let id = request.id;
    let topic = request.topic;

    // 解析配置项名称和值
    let (key, value) = match request.payload {
        Some(payload) => {
            // 尝试从 payload 中获取 key 和 value 字段
            if let serde_json::Value::Object(obj) = payload {
                let key = match obj.get("key") {
                    Some(serde_json::Value::String(s)) => vec![s.clone()],
                    Some(serde_json::Value::Array(arr)) => arr.iter().filter_map(|v| v.as_str().map(|s| s.to_string())).collect(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::ConfigSetConfigMssingParamKey,
                            module: "config".to_string(),
                            user_message: "设置配置失败".to_string(),
                            dev_message: "Missing required parameter: key".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let value = if let Some(value) = obj.get("value") {
                    value.to_string()
                } else {
                    let error = IpcError {
                        code: ErrCodes::ConfigSetConfigMssingParamValue,
                        module: "config".to_string(),
                        user_message: "设置配置失败".to_string(),
                        dev_message: "Missing required parameter: value".to_string(),
                        detail: None,
                        retryable: false,
                    };
                    return CallResponse::error(id, topic, error);
                };

                (key, value)
            } else {
                let error = IpcError {
                    code: ErrCodes::ConfigSetConfigInvalidPayloadFormat,
                    module: "config".to_string(),
                    user_message: "设置配置失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        }
        None => {
            let error = IpcError {
                code: ErrCodes::ConfigSetConfigMssingParamKeyAndValue,
                module: "config".to_string(),
                user_message: "设置配置失败".to_string(),
                dev_message: "Missing required parameters: key and value".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("set_config", CommandInput::Args(vec![key.clone().join("."), value.clone()])).await {
        Ok(output) => match output {
            CommandOutput::Text(v) => CallResponse::ok(id, topic, v),
            _ => {
                let error = IpcError {
                    code: ErrCodes::ConfigSetConfigInvalidResponse,
                    module: "config".to_string(),
                    user_message: "设置配置失败".to_string(),
                    dev_message: "Invalid response type from set_config command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, topic, error)
            }
        },
        Err(e) => {
            let error = IpcError {
                code: ErrCodes::ConfigSetConfigInvokeError,
                module: "config".to_string(),
                user_message: "设置配置失败".to_string(),
                dev_message: format!("Failed to invoke set_config: {:?}", e),
                detail: None,
                retryable: false,
            };
            CallResponse::error(id, topic, error)
        }
    }
}
