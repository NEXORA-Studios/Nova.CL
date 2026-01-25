use tauri::{AppHandle, Manager};

use crate::ipc::{CallRequest, CallResponse, IpcError};
use crate::r#static::ErrCodes;
use crate::lifecycle::CommandInput;
use crate::services::http::types::{HttpRequest, HttpResponse};

/// 发送 HTTP 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 HTTP 请求对象
#[tauri::command]
pub async fn http_request(app_handle: AppHandle, request: CallRequest<HttpRequest>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID、topic 和请求对象
    let id = request.id;
    let topic = request.topic;
    let req = match request.payload {
        Some(req) => req,
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientRequestMissingParamReq,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameter: req".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 调用生命周期管理器的命令
    match manager.invoke_command("http_request", CommandInput::HttpClientReq(req)).await {
        Ok(output) => match output {
            crate::lifecycle::CommandOutput::HttpClientResp(resp) => CallResponse::ok(id, topic, resp),
            _ => {
                let error = IpcError {
                    code: ErrCodes::HttpClientRequestInvalidResponse,
                    module: "http".to_string(),
                    user_message: "无效的响应类型".to_string(),
                    dev_message: "Invalid response type from http_request command".to_string(),
                    detail: None,
                    retryable: false,
                };
                CallResponse::error(id, topic, error)
            }
        },
        Err(e) => {
            eprintln!("Failed to invoke http_request: {:?}", e);
            let error = IpcError {
                code: ErrCodes::HttpClientRequestInvokeError,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: format!("Failed to invoke http_request: {:?}", e),
                detail: None,
                retryable: true,
            };
            CallResponse::error(id, topic, error)
        }
    }
}

/// 发送 GET 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 GET 请求参数
#[tauri::command]
pub async fn http_get(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (url, headers) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let url = match obj.get("url") {
                    Some(serde_json::Value::String(url)) => url.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpClientGetMissingParamUrl,
                            module: "http".to_string(),
                            user_message: "请求发送失败".to_string(),
                            dev_message: "Missing required parameter: url".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let headers = match obj.get("headers") {
                    Some(serde_json::Value::Object(headers_obj)) => {
                        let mut headers = std::collections::HashMap::new();
                        for (key, value) in headers_obj {
                            if let serde_json::Value::String(value) = value {
                                headers.insert(key.clone(), value.clone());
                            }
                        }
                        Some(headers)
                    },
                    _ => None,
                };

                (url, headers)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpClientGetInvalidPayloadFormat,
                    module: "http".to_string(),
                    user_message: "请求发送失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        },
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientGetMissingParamUrl,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameters: url".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 构建 HttpRequest 对象
    let req = HttpRequest {
        method: "GET".to_string(),
        url,
        headers,
        body: None,
    };

    // 创建 CallRequest 并调用 http_request
    let request = CallRequest {
        version: crate::ipc::IPC_VERSION as u8,
        id,
        topic: "http_request".to_string(),
        payload: Some(req),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    http_request(app_handle, request).await
}

/// 发送 POST 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 POST 请求参数
#[tauri::command]
pub async fn http_post(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (url, headers, body) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let url = match obj.get("url") {
                    Some(serde_json::Value::String(url)) => url.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpClientPostMissingParamUrl,
                            module: "http".to_string(),
                            user_message: "请求发送失败".to_string(),
                            dev_message: "Missing required parameter: url".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let headers = match obj.get("headers") {
                    Some(serde_json::Value::Object(headers_obj)) => {
                        let mut headers = std::collections::HashMap::new();
                        for (key, value) in headers_obj {
                            if let serde_json::Value::String(value) = value {
                                headers.insert(key.clone(), value.clone());
                            }
                        }
                        Some(headers)
                    },
                    _ => None,
                };

                let body = obj.get("body").cloned();

                (url, headers, body)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpClientPostInvalidPayloadFormat,
                    module: "http".to_string(),
                    user_message: "请求发送失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        },
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientPostMissingParamUrl,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameters: url".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 构建 HttpRequest 对象
    let req = HttpRequest {
        method: "POST".to_string(),
        url,
        headers,
        body,
    };

    // 创建 CallRequest 并调用 http_request
    let request = CallRequest {
        version: crate::ipc::IPC_VERSION as u8,
        id,
        topic: "http_request".to_string(),
        payload: Some(req),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    http_request(app_handle, request).await
}

/// 发送 PUT 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 PUT 请求参数
#[tauri::command]
pub async fn http_put(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (url, headers, body) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let url = match obj.get("url") {
                    Some(serde_json::Value::String(url)) => url.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpClientPutMissingParamUrl,
                            module: "http".to_string(),
                            user_message: "请求发送失败".to_string(),
                            dev_message: "Missing required parameter: url".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let headers = match obj.get("headers") {
                    Some(serde_json::Value::Object(headers_obj)) => {
                        let mut headers = std::collections::HashMap::new();
                        for (key, value) in headers_obj {
                            if let serde_json::Value::String(value) = value {
                                headers.insert(key.clone(), value.clone());
                            }
                        }
                        Some(headers)
                    },
                    _ => None,
                };

                let body = obj.get("body").cloned();

                (url, headers, body)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpClientPutInvalidPayloadFormat,
                    module: "http".to_string(),
                    user_message: "请求发送失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        },
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientPutMissingParamUrl,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameters: url".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 构建 HttpRequest 对象
    let req = HttpRequest {
        method: "PUT".to_string(),
        url,
        headers,
        body,
    };

    // 创建 CallRequest 并调用 http_request
    let request = CallRequest {
        version: crate::ipc::IPC_VERSION as u8,
        id,
        topic: "http_request".to_string(),
        payload: Some(req),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    http_request(app_handle, request).await
}

/// 发送 DELETE 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 DELETE 请求参数
#[tauri::command]
pub async fn http_delete(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (url, headers) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let url = match obj.get("url") {
                    Some(serde_json::Value::String(url)) => url.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpClientDeleteMissingParamUrl,
                            module: "http".to_string(),
                            user_message: "请求发送失败".to_string(),
                            dev_message: "Missing required parameter: url".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let headers = match obj.get("headers") {
                    Some(serde_json::Value::Object(headers_obj)) => {
                        let mut headers = std::collections::HashMap::new();
                        for (key, value) in headers_obj {
                            if let serde_json::Value::String(value) = value {
                                headers.insert(key.clone(), value.clone());
                            }
                        }
                        Some(headers)
                    },
                    _ => None,
                };

                (url, headers)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpClientDeleteInvalidPayloadFormat,
                    module: "http".to_string(),
                    user_message: "请求发送失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        },
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientDeleteMissingParamUrl,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameters: url".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 构建 HttpRequest 对象
    let req = HttpRequest {
        method: "DELETE".to_string(),
        url,
        headers,
        body: None,
    };

    // 创建 CallRequest 并调用 http_request
    let request = CallRequest {
        version: crate::ipc::IPC_VERSION as u8,
        id,
        topic: "http_request".to_string(),
        payload: Some(req),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    http_request(app_handle, request).await
}

/// 发送 PATCH 请求
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID 和 PATCH 请求参数
#[tauri::command]
pub async fn http_patch(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<HttpResponse> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析请求参数
    let (url, headers, body) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let url = match obj.get("url") {
                    Some(serde_json::Value::String(url)) => url.clone(),
                    _ => {
                        let error = IpcError {
                            code: ErrCodes::HttpClientPatchMissingParamUrl,
                            module: "http".to_string(),
                            user_message: "请求发送失败".to_string(),
                            dev_message: "Missing required parameter: url".to_string(),
                            detail: None,
                            retryable: false,
                        };
                        return CallResponse::error(id, topic, error);
                    }
                };

                let headers = match obj.get("headers") {
                    Some(serde_json::Value::Object(headers_obj)) => {
                        let mut headers = std::collections::HashMap::new();
                        for (key, value) in headers_obj {
                            if let serde_json::Value::String(value) = value {
                                headers.insert(key.clone(), value.clone());
                            }
                        }
                        Some(headers)
                    },
                    _ => None,
                };

                let body = obj.get("body").cloned();

                (url, headers, body)
            } else {
                let error = IpcError {
                    code: ErrCodes::HttpClientPatchInvalidPayloadFormat,
                    module: "http".to_string(),
                    user_message: "请求发送失败".to_string(),
                    dev_message: "Invalid payload format".to_string(),
                    detail: None,
                    retryable: false,
                };
                return CallResponse::error(id, topic, error);
            }
        },
        None => {
            let error = IpcError {
                code: ErrCodes::HttpClientPatchMissingParamUrl,
                module: "http".to_string(),
                user_message: "请求发送失败".to_string(),
                dev_message: "Missing required parameters: url".to_string(),
                detail: None,
                retryable: false,
            };
            return CallResponse::error(id, topic, error);
        }
    };

    // 构建 HttpRequest 对象
    let req = HttpRequest {
        method: "PATCH".to_string(),
        url,
        headers,
        body,
    };

    // 创建 CallRequest 并调用 http_request
    let request = CallRequest {
        version: crate::ipc::IPC_VERSION as u8,
        id,
        topic: "http_request".to_string(),
        payload: Some(req),
        timestamp: chrono::Utc::now().timestamp_millis() as u64,
    };

    http_request(app_handle, request).await
}
