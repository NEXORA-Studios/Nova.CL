use log::{debug, error, info, trace, warn};

/// 记录 TRACE 级别的日志
///
/// # 参数
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub fn log_trace(category: String, message: String) {
    let target = format!("ts::{}", category);
    trace!(target: &target, "{}", message);
}

/// 记录 DEBUG 级别的日志
///
/// # 参数
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub fn log_debug(category: String, message: String) {
    let target = format!("ts::{}", category);
    debug!(target: &target, "{}", message);
}

/// 记录 INFO 级别的日志
///
/// # 参数
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub fn log_info(category: String, message: String) {
    let target = format!("ts::{}", category);
    info!(target: &target, "{}", message);
}

/// 记录 WARN 级别的日志
///
/// # 参数
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub fn log_warn(category: String, message: String) {
    let target = format!("ts::{}", category);
    warn!(target: &target, "{}", message);
}

/// 记录 ERROR 级别的日志
///
/// # 参数
/// - `category`: 日志分类
/// - `message`: 日志内容
#[tauri::command]
pub fn log_error(category: String, message: String) {
    let target = format!("ts::{}", category);
    error!(target: &target, "{}", message);
}
