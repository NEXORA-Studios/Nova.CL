use crate::http::client::HttpClient;
use crate::http::server::{get_server_status, start_server, stop_server};
use crate::http::types::{HttpError, HttpRequest, HttpResponse};
use log::trace;
use serde_json::json;
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
    trace!(
        "http_post: url={}, headers={:?}, body={:?}",
        url,
        headers,
        body
    );
    HTTP_CLIENT.post(&url, headers, body).await
}

#[tauri::command]
pub async fn http_put(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    trace!(
        "http_put: url={}, headers={:?}, body={:?}",
        url,
        headers,
        body
    );
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
    trace!(
        "http_patch: url={}, headers={:?}, body={:?}",
        url,
        headers,
        body
    );
    HTTP_CLIENT.patch(&url, headers, body).await
}

#[tauri::command]
pub async fn http_server_start(
    app_handle: tauri::AppHandle,
    port: u16,
    lang: Option<String>, // 可选语言参数，前端不传则默认 "en"
) -> Result<serde_json::Value, String> {
    let lang = lang.unwrap_or_else(|| "en".to_string());

    // 启动服务器
    match start_server(app_handle, port, lang).await {
        Ok(_) => {
            // start_server 成功后会返回一个无用的 oneshot::Receiver
            // 我们不需要等待它，直接返回成功
            Ok(json!({
                "status": "ok",
                "message": "OAuth server started",
                "port": port
            }))
        }
        Err(e) => {
            // 启动失败（比如端口被占用、服务器已在运行等）
            Err(format!("Failed to start server: {}", e))
        }
    }
}

#[tauri::command]
pub async fn http_server_stop() -> Result<serde_json::Value, String> {
    match stop_server().await {
        Ok(()) => Ok(json!({
            "status": "ok",
            "message": "OAuth server stopped successfully"
        })),
        Err(e) => Err(e), // stop_server 已经返回友好错误信息
    }
}

#[tauri::command]
pub async fn http_server_status() -> Result<serde_json::Value, String> {
    match get_server_status().await {
        Some(port) => Ok(json!({
            "status": "running",
            "port": port
        })),
        None => Ok(json!({
            "status": "stopped"
        })),
    }
}
