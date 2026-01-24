use tauri::{AppHandle, Manager};

use crate::ipc::{CallResponse, IpcError};
use crate::lifecycle::CommandInput;
use crate::services::http::types::{HttpRequest, HttpResponse};
use serde_json::Value;

/// 发送 HTTP 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `req`: HTTP 请求对象
#[tauri::command]
pub async fn http_request(
    app_handle: AppHandle,
    id: u64,
    req: HttpRequest,
) -> CallResponse<HttpResponse> {
    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager
        .invoke_command("http_request", CommandInput::HttpClientReq(req))
        .await
    {
        Ok(output) => match output {
            crate::lifecycle::CommandOutput::HttpClientResp(resp) => {
                CallResponse::ok(id, "http_request".to_string(), resp)
            }
            _ => {
                let error = IpcError {
                    code: 1001,
                    module: "http".to_string(),
                    user_message: "无效的响应类型".to_string(),
                    dev_message: "Invalid response type from http_request command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, "http_request".to_string(), error)
            }
        },
        Err(e) => {
            eprintln!("Failed to invoke http_request: {:?}", e);
            let error = IpcError {
                code: 1002,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: format!("Failed to invoke http_request: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, "http_request".to_string(), error)
        }
    }
}

/// 发送 GET 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `url`: 请求 URL
/// - `headers`: 请求头
#[tauri::command]
pub async fn http_get(
    app_handle: AppHandle,
    id: u64,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
) -> CallResponse<HttpResponse> {
    let req = HttpRequest {
        method: "GET".to_string(),
        url,
        headers,
        body: None,
    };
    http_request(app_handle, id, req).await
}

/// 发送 POST 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `url`: 请求 URL
/// - `headers`: 请求头
/// - `body`: 请求体
#[tauri::command]
pub async fn http_post(
    app_handle: AppHandle,
    id: u64,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<Value>,
) -> CallResponse<HttpResponse> {
    let req = HttpRequest {
        method: "POST".to_string(),
        url,
        headers,
        body,
    };
    http_request(app_handle, id, req).await
}

/// 发送 PUT 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `url`: 请求 URL
/// - `headers`: 请求头
/// - `body`: 请求体
#[tauri::command]
pub async fn http_put(
    app_handle: AppHandle,
    id: u64,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<Value>,
) -> CallResponse<HttpResponse> {
    let req = HttpRequest {
        method: "PUT".to_string(),
        url,
        headers,
        body,
    };
    http_request(app_handle, id, req).await
}

/// 发送 DELETE 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `url`: 请求 URL
/// - `headers`: 请求头
#[tauri::command]
pub async fn http_delete(
    app_handle: AppHandle,
    id: u64,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
) -> CallResponse<HttpResponse> {
    let req = HttpRequest {
        method: "DELETE".to_string(),
        url,
        headers,
        body: None,
    };
    http_request(app_handle, id, req).await
}

/// 发送 PATCH 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `url`: 请求 URL
/// - `headers`: 请求头
/// - `body`: 请求体
#[tauri::command]
pub async fn http_patch(
    app_handle: AppHandle,
    id: u64,
    url: String,
    headers: Option<std::collections::HashMap<String, String>>,
    body: Option<Value>,
) -> CallResponse<HttpResponse> {
    let req = HttpRequest {
        method: "PATCH".to_string(),
        url,
        headers,
        body,
    };
    http_request(app_handle, id, req).await
}
