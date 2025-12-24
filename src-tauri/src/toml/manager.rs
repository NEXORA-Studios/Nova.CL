use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use once_cell::sync::Lazy;
use toml::ser::to_string_pretty;

use crate::toml::crypto::init_encryption_key;
use crate::toml::error::{ConfigError, GlobalConfigError, InstanceConfigError, ProfileConfigError};
use crate::toml::types::{GlobalConfig, InstanceConfig, ProfileConfig};

/// 配置管理器
pub struct ConfigManager {
    /// 应用数据目录
    app_data_dir: PathBuf,
    /// 全局配置缓存
    global_config: RwLock<Option<GlobalConfig>>,
    /// 账户配置缓存
    profile_config: RwLock<Option<ProfileConfig>>,
}

/// 全局配置管理器实例
static CONFIG_MANAGER: Lazy<ConfigManager> = Lazy::new(|| {
    let app_data_dir = dirs::data_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("NovaCL");

    // 创建应用数据目录
    if let Err(e) = fs::create_dir_all(&app_data_dir) {
        eprintln!("Failed to create app data directory: {}", e);
    }

    ConfigManager {
        app_data_dir,
        global_config: RwLock::new(None),
        profile_config: RwLock::new(None),
    }
});

impl ConfigManager {
    /// 获取全局配置管理器实例
    pub fn instance() -> &'static Self {
        &CONFIG_MANAGER
    }

    /// 初始化配置系统
    pub fn init() -> Result<(), ConfigError> {
        // 初始化加密密钥
        init_encryption_key()?;

        // 预加载全局配置
        let _ = Self::instance().load_global_config();

        // 预加载账户配置
        let _ = Self::instance().load_profile_config();

        Ok(())
    }

    /// 获取应用数据目录
    #[allow(dead_code)]
    pub fn app_data_dir(&self) -> &Path {
        &self.app_data_dir
    }

    /// 获取全局配置文件路径
    pub fn global_config_path(&self) -> PathBuf {
        self.app_data_dir.join("Config.toml")
    }

    /// 获取账户配置文件路径
    pub fn profile_config_path(&self) -> PathBuf {
        self.app_data_dir.join("Profile.toml")
    }

    /// 获取实例配置目录
    pub fn instances_dir(&self) -> PathBuf {
        self.app_data_dir.join("instances")
    }

    /// 获取特定实例配置文件路径
    pub fn instance_config_path(&self, instance_id: &str) -> PathBuf {
        self.instances_dir().join(format!("{}.toml", instance_id))
    }

    /// 加载全局配置
    pub fn load_global_config(&self) -> Result<GlobalConfig, GlobalConfigError> {
        // 尝试从缓存获取
        if let Ok(guard) = self.global_config.read() {
            if let Some(config) = guard.as_ref() {
                return Ok(config.clone());
            }
        }

        let config_path = self.global_config_path();
        let config = if config_path.exists() {
            // 读取并解析配置文件
            let content = fs::read_to_string(&config_path).map_err(ConfigError::from)?;
            let config = toml::from_str(&content).map_err(ConfigError::from)?;
            config
        } else {
            // 返回默认配置
            GlobalConfig::default()
        };

        // 更新缓存
        if let Ok(mut guard) = self.global_config.write() {
            *guard = Some(config.clone());
        }

        Ok(config)
    }

    /// 保存全局配置
    pub fn save_global_config(&self, config: &GlobalConfig) -> Result<(), GlobalConfigError> {
        let config_path = self.global_config_path();
        let content = to_string_pretty(config).map_err(ConfigError::from)?;
        fs::write(&config_path, content).map_err(ConfigError::from)?;

        // 更新缓存
        if let Ok(mut guard) = self.global_config.write() {
            *guard = Some(config.clone());
        }

        Ok(())
    }

    /// 加载账户配置
    pub fn load_profile_config(&self) -> Result<ProfileConfig, ProfileConfigError> {
        // 尝试从缓存获取
        if let Ok(guard) = self.profile_config.read() {
            if let Some(config) = guard.as_ref() {
                return Ok(config.clone());
            }
        }

        let config_path = self.profile_config_path();
        let config = if config_path.exists() {
            // 读取并解析配置文件
            let content = fs::read_to_string(&config_path).map_err(ConfigError::from)?;
            let config = toml::from_str(&content).map_err(ConfigError::from)?;
            config
        } else {
            // 返回默认配置
            ProfileConfig::default()
        };

        // 更新缓存
        if let Ok(mut guard) = self.profile_config.write() {
            *guard = Some(config.clone());
        }

        Ok(config)
    }

    /// 保存账户配置
    pub fn save_profile_config(&self, config: &ProfileConfig) -> Result<(), ProfileConfigError> {
        let config_path = self.profile_config_path();
        let content = to_string_pretty(config).map_err(ConfigError::from)?;
        fs::write(&config_path, content).map_err(ConfigError::from)?;

        // 更新缓存
        if let Ok(mut guard) = self.profile_config.write() {
            *guard = Some(config.clone());
        }

        Ok(())
    }

    /// 加载实例配置
    pub fn load_instance_config(
        &self,
        instance_id: &str,
    ) -> Result<InstanceConfig, InstanceConfigError> {
        let config_path = self.instance_config_path(instance_id);

        if !config_path.exists() {
            return Err(InstanceConfigError::InstanceNotFound(
                instance_id.to_string(),
            ));
        }

        // 读取并解析配置文件
        let content = fs::read_to_string(&config_path).map_err(ConfigError::from)?;
        let config = toml::from_str(&content).map_err(ConfigError::from)?;

        Ok(config)
    }

    /// 保存实例配置
    pub fn save_instance_config(
        &self,
        instance_id: &str,
        config: &InstanceConfig,
    ) -> Result<(), InstanceConfigError> {
        let instances_dir = self.instances_dir();

        // 创建实例目录
        if let Err(e) = fs::create_dir_all(&instances_dir) {
            return Err(InstanceConfigError::ConfigError(ConfigError::PathError(
                format!("Failed to create instances directory: {}", e),
            )));
        }

        let config_path = self.instance_config_path(instance_id);
        let content = to_string_pretty(config).map_err(ConfigError::from)?;
        fs::write(&config_path, content).map_err(ConfigError::from)?;

        Ok(())
    }

    /// 删除实例配置
    pub fn delete_instance_config(&self, instance_id: &str) -> Result<(), InstanceConfigError> {
        let config_path = self.instance_config_path(instance_id);

        if !config_path.exists() {
            return Err(InstanceConfigError::InstanceNotFound(
                instance_id.to_string(),
            ));
        }

        fs::remove_file(&config_path).map_err(ConfigError::from)?;

        Ok(())
    }

    /// 列出所有实例配置
    pub fn list_instance_configs(&self) -> Result<Vec<String>, InstanceConfigError> {
        let instances_dir = self.instances_dir();

        if !instances_dir.exists() {
            return Ok(Vec::new());
        }

        let mut instance_ids = Vec::new();

        // 遍历实例目录
        for entry in fs::read_dir(&instances_dir).map_err(ConfigError::from)? {
            let entry = entry.map_err(ConfigError::from)?;
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "toml" {
                        // 获取文件名（不含扩展名）作为实例 ID
                        if let Some(file_stem) = path.file_stem() {
                            if let Some(instance_id) = file_stem.to_str() {
                                instance_ids.push(instance_id.to_string());
                            }
                        }
                    }
                }
            }
        }

        Ok(instance_ids)
    }
}

/// 获取全局配置
pub fn get_global_config() -> Result<GlobalConfig, GlobalConfigError> {
    ConfigManager::instance().load_global_config()
}

/// 保存全局配置
pub fn save_global_config(config: &GlobalConfig) -> Result<(), GlobalConfigError> {
    ConfigManager::instance().save_global_config(config)
}

/// 获取账户配置
pub fn get_profile_config() -> Result<ProfileConfig, ProfileConfigError> {
    ConfigManager::instance().load_profile_config()
}

/// 保存账户配置
pub fn save_profile_config(config: &ProfileConfig) -> Result<(), ProfileConfigError> {
    ConfigManager::instance().save_profile_config(config)
}

/// 获取实例配置
pub fn get_instance_config(instance_id: &str) -> Result<InstanceConfig, InstanceConfigError> {
    ConfigManager::instance().load_instance_config(instance_id)
}

/// 保存实例配置
pub fn save_instance_config(
    instance_id: &str,
    config: &InstanceConfig,
) -> Result<(), InstanceConfigError> {
    ConfigManager::instance().save_instance_config(instance_id, config)
}

/// 删除实例配置
pub fn delete_instance_config(instance_id: &str) -> Result<(), InstanceConfigError> {
    ConfigManager::instance().delete_instance_config(instance_id)
}

/// 列出所有实例配置
pub fn list_instance_configs() -> Result<Vec<String>, InstanceConfigError> {
    ConfigManager::instance().list_instance_configs()
}
