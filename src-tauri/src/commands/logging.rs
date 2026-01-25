use log::{debug, error, info, trace, warn};
use tauri::{AppHandle, Manager};

use crate::ipc::CallResponse;
use crate::lifecycle::CommandInput;

/// 记录 TRACE 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub async fn log_trace(app_handle: AppHandle, id: u64, category: String, message: String) -> CallResponse<()> {
    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_trace", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, "log_trace".to_string(), ()),
        Err(e) => {
            eprintln!("Failed to invoke log_trace: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            trace!(target: &target, "{}", message);
            CallResponse::ok(id, "log_trace".to_string(), ())
        }
    }
}

/// 记录 DEBUG 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub async fn log_debug(app_handle: AppHandle, id: u64, category: String, message: String) -> CallResponse<()> {
    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_debug", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, "log_debug".to_string(), ()),
        Err(e) => {
            eprintln!("Failed to invoke log_debug: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            debug!(target: &target, "{}", message);
            CallResponse::ok(id, "log_debug".to_string(), ())
        }
    }
}

/// 记录 INFO 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub async fn log_info(app_handle: AppHandle, id: u64, category: String, message: String) -> CallResponse<()> {
    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_info", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, "log_info".to_string(), ()),
        Err(e) => {
            eprintln!("Failed to invoke log_info: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            info!(target: &target, "{}", message);
            CallResponse::ok(id, "log_info".to_string(), ())
        }
    }
}

/// 记录 WARN 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub async fn log_warn(app_handle: AppHandle, id: u64, category: String, message: String) -> CallResponse<()> {
    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_warn", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, "log_warn".to_string(), ()),
        Err(e) => {
            eprintln!("Failed to invoke log_warn: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            warn!(target: &target, "{}", message);
            CallResponse::ok(id, "log_warn".to_string(), ())
        }
    }
}

/// 记录 ERROR 级别的日志
///
/// # 参数
/// - `app_handle`: Tauri 应用句柄，用于获取生命周期管理器
/// - `id`: 请求 ID，用于匹配响应
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub async fn log_error(app_handle: AppHandle, id: u64, category: String, message: String) -> CallResponse<()> {
    // 构建命令参数
    let args = vec![category.clone(), message.clone()];

    // 从应用状态中获取生命周期管理器
    let manager = app_handle.state::<crate::lifecycle::LifecycleManager>();
    let manager = manager.inner().clone();

    // 异步调用日志命令
    match manager.invoke_command("log_error", CommandInput::Args(args)).await {
        Ok(_) => CallResponse::ok(id, "log_error".to_string(), ()),
        Err(e) => {
            eprintln!("Failed to invoke log_error: {:?}", e);
            // 降级到直接日志记录
            let target = format!("ts::{}", category);
            error!(target: &target, "{}", message);
            CallResponse::ok(id, "log_error".to_string(), ())
        }
    }
}
