use crate::http::client::HttpClient;
use crate::http::server::{get_server_status, start_server, stop_server};
use crate::http::types::{HttpError, HttpRequest, HttpResponse};
use log::trace;
use std::collections::HashMap;

// 创建一个全局的 HTTP 客户端实例
lazy_static::lazy_static! {
    static ref HTTP_CLIENT: HttpClient = HttpClient::new();
}

#[tauri::command]
pub async fn http_request(req: HttpRequest) -> Result<HttpResponse, HttpError> {
    HTTP_CLIENT.request(&req).await
}

#[tauri::command]
pub async fn http_get(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    trace!("http_get: url={}, headers={:?}", url, headers);
    HTTP_CLIENT.get(&url, headers).await
}

#[tauri::command]
pub async fn http_post(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    trace!("http_post: url={}, headers={:?}, body={:?}", url, headers, body);
    HTTP_CLIENT.post(&url, headers, body).await
}

#[tauri::command]
pub async fn http_put(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    trace!("http_put: url={}, headers={:?}, body={:?}", url, headers, body);
    HTTP_CLIENT.put(&url, headers, body).await
}

#[tauri::command]
pub async fn http_delete(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    trace!("http_delete: url={}, headers={:?}", url, headers);
    HTTP_CLIENT.delete(&url, headers).await
}

#[tauri::command]
pub async fn http_patch(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    trace!("http_patch: url={}, headers={:?}, body={:?}", url, headers, body);
    HTTP_CLIENT.patch(&url, headers, body).await
}

#[tauri::command]
pub async fn http_server_start(
    app_handle: tauri::AppHandle,
    port: u16,
) -> Result<serde_json::Value, String> {
    // 启动服务器，获取授权代码接收器
    let auth_code_rx = start_server(app_handle, port).await?;

    // 等待授权代码到达
    match auth_code_rx.await {
        Ok(params) => {
            // 服务器会自动关闭，返回授权代码和其他参数
            Ok(
                serde_json::json!({"status": "ok", "message": "Authorization code received", "params": params}),
            )
        }
        Err(e) => {
            // 发生错误，关闭服务器
            let _ = stop_server().await;
            Err(format!("Failed to receive authorization code: {}", e))

        }
    }
}

#[tauri::command]
pub async fn http_server_stop() -> Result<serde_json::Value, String> {
    stop_server().await?;
    Ok(serde_json::json!({"status": "ok", "message": "Server stopped"}))
}

#[tauri::command]
pub async fn http_server_status() -> Result<serde_json::Value, String> {
    match get_server_status().await {
        Some(port) => Ok(serde_json::json!({"status": "running", "port": port})),
        None => Ok(serde_json::json!({"status": "stopped"})),
    }
}
