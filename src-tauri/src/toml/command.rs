use tauri::command;

use crate::toml::crypto::{decrypt_string, encrypt_string};
use crate::toml::manager::{
    delete_instance_config, get_global_config, get_instance_config, get_profile_config,
    list_instance_configs, save_global_config, save_instance_config, save_profile_config,
};
use crate::toml::types::{GlobalConfig, InstanceConfig, ProfileConfig};
use crate::toml::utils::generate_instance_id;

/// 获取全局配置
#[command]
pub fn get_global_config_cmd() -> Result<GlobalConfig, String> {
    get_global_config().map_err(|e| format!("Failed to get global config: {}", e))
}

/// 保存全局配置
#[command]
pub fn save_global_config_cmd(config: GlobalConfig) -> Result<(), String> {
    save_global_config(&config).map_err(|e| format!("Failed to save global config: {}", e))
}

/// 获取账户配置
#[command]
pub fn get_profile_config_cmd() -> Result<ProfileConfig, String> {
    get_profile_config().map_err(|e| format!("Failed to get profile config: {}", e))
}

/// 保存账户配置
#[command]
pub fn save_profile_config_cmd(config: ProfileConfig) -> Result<(), String> {
    save_profile_config(&config).map_err(|e| format!("Failed to save profile config: {}", e))
}

/// 获取实例配置
#[command]
pub fn get_instance_config_cmd(instance_id: String) -> Result<InstanceConfig, String> {
    get_instance_config(&instance_id).map_err(|e| format!("Failed to get instance config: {}", e))
}

/// 保存实例配置
#[command]
pub fn save_instance_config_cmd(
    instance_id: Option<String>,
    config: InstanceConfig,
) -> Result<String, String> {
    // 如果没有提供实例 ID，生成一个新的
    let instance_id = instance_id.unwrap_or_else(generate_instance_id);

    save_instance_config(&instance_id, &config)
        .map(|_| instance_id)
        .map_err(|e| format!("Failed to save instance config: {}", e))
}

/// 删除实例配置
#[command]
pub fn delete_instance_config_cmd(instance_id: String) -> Result<(), String> {
    delete_instance_config(&instance_id)
        .map_err(|e| format!("Failed to delete instance config: {}", e))
}

/// 列出所有实例配置
#[command]
pub fn list_instance_configs_cmd() -> Result<Vec<String>, String> {
    list_instance_configs().map_err(|e| format!("Failed to list instance configs: {}", e))
}

/// 解密字符串
#[command]
pub fn decrypt_string_cmd(encrypted: String) -> Result<String, String> {
    decrypt_string(&encrypted).map_err(|e| format!("Failed to decrypt string: {}", e))
}

/// 加密字符串
#[command]
pub fn encrypt_string_cmd(plaintext: String) -> Result<String, String> {
    encrypt_string(&plaintext).map_err(|e| format!("Failed to encrypt string: {}", e))
}
