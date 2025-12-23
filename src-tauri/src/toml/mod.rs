pub mod command;
pub mod crypto;
/// 配置模块
///
/// 提供多类型 TOML 配置文件的管理功能，包括：
/// - 全局配置 (Config.toml)
/// - 账户配置 (Profile.toml)，含敏感字段加密
/// - 实例配置
///
/// 使用 ChaCha20-Poly1305 算法对敏感字段进行加密保护
// 导出子模块
pub mod error;
pub mod manager;
#[cfg(debug_assertions)]
pub mod test;
pub mod types;
pub mod utils;

// 重新导出常用类型和函数
pub use error::{ConfigError, GlobalConfigError, InstanceConfigError, ProfileConfigError};
pub use manager::{
    delete_instance_config, get_global_config, get_instance_config, get_profile_config,
    list_instance_configs, save_global_config, save_instance_config, save_profile_config,
    ConfigManager,
};
#[cfg(debug_assertions)]
pub use test::test_config_system;
pub use types::{GlobalConfig, InstanceConfig, Profile, ProfileConfig};
pub use utils::{
    backup_config_file, generate_instance_id, restore_config_backup, validate_channel,
    validate_download_source, validate_java_path, validate_language, validate_loader_type,
    validate_theme, validate_update_method,
};

/// 初始化配置系统
pub fn init_config() -> Result<(), ConfigError> {
    manager::ConfigManager::init()
}
