use crate::http::client::HttpClient;
use crate::http::types::{HttpError, HttpRequest, HttpResponse};
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
    HTTP_CLIENT.get(&url, headers).await
}

#[tauri::command]
pub async fn http_post(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    HTTP_CLIENT.post(&url, headers, body).await
}

#[tauri::command]
pub async fn http_put(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    HTTP_CLIENT.put(&url, headers, body).await
}

#[tauri::command]
pub async fn http_delete(
    url: String,
    headers: Option<HashMap<String, String>>,
) -> Result<HttpResponse, HttpError> {
    HTTP_CLIENT.delete(&url, headers).await
}

#[tauri::command]
pub async fn http_patch(
    url: String,
    headers: Option<HashMap<String, String>>,
    body: Option<serde_json::Value>,
) -> Result<HttpResponse, HttpError> {
    HTTP_CLIENT.patch(&url, headers, body).await
}
