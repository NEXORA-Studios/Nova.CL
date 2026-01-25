use tauri::{AppHandle, Manager};

use crate::ipc::{CallResponse, IpcError};
use crate::lifecycle::{CommandInput, CommandOutput, LifecycleManager};
use crate::services::system::ram::RamInfo;

/// 获取系统内存信息
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
#[tauri::command]
pub async fn get_ram_info(app_handle: AppHandle, id: u64) -> CallResponse<RamInfo> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("get_ram_info", CommandInput::Args(vec![])).await {
        Ok(output) => match output {
            CommandOutput::Json(v) => match serde_json::from_value::<RamInfo>(v) {
                Ok(ram_info) => CallResponse::ok(id, "get_ram_info".to_string(), ram_info),
                Err(e) => {
                    let error = IpcError {
                        code: 3001,
                        module: "system".to_string(),
                        user_message: "解析内存信息失败".to_string(),
                        dev_message: format!("Failed to parse ram info: {}", e),
                        detail: None,
                        retryable: false,
                    };
                    CallResponse::error(id, "get_ram_info".to_string(), error)
                }
            },
            _ => {
                let error = IpcError {
                    code: 3002,
                    module: "system".to_string(),
                    user_message: "无效的响应类型".to_string(),
                    dev_message: "Invalid response type from get_ram_info command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, "get_ram_info".to_string(), error)
            }
        },
        Err(e) => {
            let error = IpcError {
                code: 3003,
                module: "system".to_string(),
                user_message: "获取内存信息失败".to_string(),
                dev_message: format!("Failed to invoke get_ram_info: {:?}", e),
                detail: None,
                retryable: false,
            };
            CallResponse::error(id, "get_ram_info".to_string(), error)
        }
    }
}
