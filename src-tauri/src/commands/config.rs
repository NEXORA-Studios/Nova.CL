use tauri::{AppHandle, Manager};

use crate::ipc::{CallResponse, IpcError};
use crate::lifecycle::{CommandInput, CommandOutput, LifecycleManager};

/// 获取配置值
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `key`: 配置项名称
#[tauri::command]
pub async fn get_config(app_handle: AppHandle, id: u64, key: String) -> CallResponse<String> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("get_config", CommandInput::Args(vec![key.clone()])).await {
        Ok(output) => match output {
            CommandOutput::Text(v) => CallResponse::ok(id, "get_config".to_string(), v),
            _ => {
                let error = IpcError {
                    code: 2001,
                    module: "config".to_string(),
                    user_message: "获取配置失败".to_string(),
                    dev_message: "Invalid response type from get_config command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, "get_config".to_string(), error)
            }
        },
        Err(e) => {
            let error = IpcError {
                code: 2002,
                module: "config".to_string(),
                user_message: "获取配置失败".to_string(),
                dev_message: format!("Failed to invoke get_config: {:?}", e),
                detail: None,
                retryable: false,
            };
            CallResponse::error(id, "get_config".to_string(), error)
        }
    }
}

/// 设置配置值
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `key`: 配置项名称
/// - `value`: 配置值
#[tauri::command]
pub async fn set_config(app_handle: AppHandle, id: u64, key: String, value: String) -> CallResponse<String> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("set_config", CommandInput::Args(vec![key.clone(), value.clone()])).await {
        Ok(output) => match output {
            CommandOutput::Text(v) => CallResponse::ok(id, "set_config".to_string(), v),
            _ => {
                let error = IpcError {
                    code: 2003,
                    module: "config".to_string(),
                    user_message: "设置配置失败".to_string(),
                    dev_message: "Invalid response type from set_config command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, "set_config".to_string(), error)
            }
        },
        Err(e) => {
            let error = IpcError {
                code: 2004,
                module: "config".to_string(),
                user_message: "设置配置失败".to_string(),
                dev_message: format!("Failed to invoke set_config: {:?}", e),
                detail: None,
                retryable: false,
            };
            CallResponse::error(id, "set_config".to_string(), error)
        }
    }
}
