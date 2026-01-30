use log::{debug, error, info, trace, warn};
use tauri::{AppHandle, Manager};

use crate::ipc::{CallRequest, CallResponse};
use crate::lifecycle::CommandInput;

/// 记录 TRACE 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、日志分类和日志内容
#[tauri::command]
pub async fn log_trace(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<()> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析日志参数
    let (category, message) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let category = obj.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                let message = obj.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (category, message)
            } else {
                ("default".to_string(), payload.to_string())
            }
        },
        None => ("default".to_string(), "".to_string()),
    };

    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_trace", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, topic, ()),
        Err(e) => {
            eprintln!("Failed to invoke log_trace: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            trace!(target: &target, "{}", message);
            CallResponse::ok(id, topic, ())
        }
    }
}

/// 记录 DEBUG 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、日志分类和日志内容
#[tauri::command]
pub async fn log_debug(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<()> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析日志参数
    let (category, message) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let category = obj.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                let message = obj.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (category, message)
            } else {
                ("default".to_string(), payload.to_string())
            }
        },
        None => ("default".to_string(), "".to_string()),
    };

    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_debug", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, topic, ()),
        Err(e) => {
            eprintln!("Failed to invoke log_debug: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            debug!(target: &target, "{}", message);
            CallResponse::ok(id, topic, ())
        }
    }
}

/// 记录 INFO 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、日志分类和日志内容
#[tauri::command]
pub async fn log_info(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<()> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析日志参数
    let (category, message) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let category = obj.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                let message = obj.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (category, message)
            } else {
                ("default".to_string(), payload.to_string())
            }
        },
        None => ("default".to_string(), "".to_string()),
    };

    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_info", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, topic, ()),
        Err(e) => {
            eprintln!("Failed to invoke log_info: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            info!(target: &target, "{}", message);
            CallResponse::ok(id, topic, ())
        }
    }
}

/// 记录 WARN 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、日志分类和日志内容
#[tauri::command]
pub async fn log_warn(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<()> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析日志参数
    let (category, message) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let category = obj.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                let message = obj.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (category, message)
            } else {
                ("default".to_string(), payload.to_string())
            }
        },
        None => ("default".to_string(), "".to_string()),
    };

    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_warn", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, topic, ()),
        Err(e) => {
            eprintln!("Failed to invoke log_warn: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            warn!(target: &target, "{}", message);
            CallResponse::ok(id, topic, ())
        }
    }
}

/// 记录 ERROR 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `request`: 前端发送的 CallRequest 结构体，包含请求 ID、日志分类和日志内容
#[tauri::command]
pub async fn log_error(app_handle: AppHandle, request: CallRequest<serde_json::Value>) -> CallResponse<()> {
    // 从 request 中提取 ID 和 topic
    let id = request.id;
    let topic = request.topic;

    // 解析日志参数
    let (category, message) = match request.payload {
        Some(payload) => {
            if let serde_json::Value::Object(obj) = payload {
                let category = obj.get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("default")
                    .to_string();
                let message = obj.get("message")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                (category, message)
            } else {
                ("default".to_string(), payload.to_string())
            }
        },
        None => ("default".to_string(), "".to_string()),
    };

    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_error", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, topic, ()),
        Err(e) => {
            eprintln!("Failed to invoke log_error: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            error!(target: &target, "{}", message);
            CallResponse::ok(id, topic, ())
        }
    }
}
