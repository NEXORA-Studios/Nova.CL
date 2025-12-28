use std::env;
use tauri::command;

/// 获取指定名称的环境变量值
#[command]
pub fn get_env_var(key: &str) -> Option<String> {
    env::var(key).ok()
}

/// 获取所有环境变量
#[command]
pub fn get_all_env_vars() -> Vec<(String, String)> {
    env::vars().collect()
}

/// 检查环境变量是否存在
#[command]
pub fn has_env_var(key: &str) -> bool {
    env::var(key).is_ok()
}
