use std::fs;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use once_cell::sync::Lazy;
use toml::ser::to_string_pretty;

use crate::toml::crypto::init_encryption_key;
use crate::toml::error::{ConfigError, GlobalConfigError, InstanceConfigError, ProfileConfigError};
use crate::toml::types::{CollectionConfig, GlobalConfig, InstanceConfig, ProfileConfig};

/// 配置管理器
pub struct ConfigManager {
    /// 应用数据目录
    app_data_dir: PathBuf,
    /// 全局配置缓存
    global_config: RwLock<Option<GlobalConfig>>,
    /// 账户配置缓存
    profile_config: RwLock<Option<ProfileConfig>>,
    /// 集合配置缓存
    collection_config: RwLock<Option<CollectionConfig>>,
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
        collection_config: RwLock::new(None),
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

        // 预加载集合配置
        let _ = Self::instance().load_collection_config();

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

    /// 获取集合配置文件路径
    pub fn collection_config_path(&self) -> PathBuf {
        self.app_data_dir.join("Collection.toml")
    }

    /// 获取特定实例配置文件路径
    /// 配置文件存储在实例目录下的 .Nova/Instance.toml
    pub fn instance_config_path(&self, instance_path: &Path) -> PathBuf {
        instance_path.join(".Nova").join("Instance.toml")
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

    /// 加载集合配置
    pub fn load_collection_config(&self) -> Result<CollectionConfig, ConfigError> {
        // 尝试从缓存获取
        if let Ok(guard) = self.collection_config.read() {
            if let Some(config) = guard.as_ref() {
                return Ok(config.clone());
            }
        }

        let config_path = self.collection_config_path();
        let config = if config_path.exists() {
            // 读取并解析配置文件
            let content = fs::read_to_string(&config_path).map_err(ConfigError::from)?;
            let config = toml::from_str(&content).map_err(ConfigError::from)?;
            config
        } else {
            // 返回默认配置
            CollectionConfig::default()
        };

        // 更新缓存
        if let Ok(mut guard) = self.collection_config.write() {
            *guard = Some(config.clone());
        }

        Ok(config)
    }

    /// 保存集合配置
    pub fn save_collection_config(&self, config: &CollectionConfig) -> Result<(), ConfigError> {
        let config_path = self.collection_config_path();
        let content = to_string_pretty(config).map_err(ConfigError::from)?;
        fs::write(&config_path, content).map_err(ConfigError::from)?;

        // 更新缓存
        if let Ok(mut guard) = self.collection_config.write() {
            *guard = Some(config.clone());
        }

        Ok(())
    }

    /// 加载实例配置
    pub fn load_instance_config(
        &self,
        instance_path: &Path,
    ) -> Result<InstanceConfig, InstanceConfigError> {
        let config_path = self.instance_config_path(instance_path);

        if !config_path.exists() {
            return Err(InstanceConfigError::InstanceNotFound(
                instance_path.to_string_lossy().to_string(),
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
        instance_path: &Path,
        config: &InstanceConfig,
    ) -> Result<(), InstanceConfigError> {
        let config_path = self.instance_config_path(instance_path);
        let nova_dir = config_path.parent().unwrap_or(instance_path);

        // 创建 .Nova 目录
        if let Err(e) = fs::create_dir_all(nova_dir) {
            return Err(InstanceConfigError::ConfigError(ConfigError::PathError(
                format!("Failed to create .Nova directory: {}", e),
            )));
        }

        let content = to_string_pretty(config).map_err(ConfigError::from)?;
        fs::write(&config_path, content).map_err(ConfigError::from)?;

        Ok(())
    }

    /// 删除实例配置
    pub fn delete_instance_config(&self, instance_path: &Path) -> Result<(), InstanceConfigError> {
        let config_path = self.instance_config_path(instance_path);

        if !config_path.exists() {
            return Err(InstanceConfigError::InstanceNotFound(
                instance_path.to_string_lossy().to_string(),
            ));
        }

        fs::remove_file(&config_path).map_err(ConfigError::from)?;

        Ok(())
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

/// 获取集合配置
pub fn get_collection_config() -> Result<CollectionConfig, ConfigError> {
    ConfigManager::instance().load_collection_config()
}

/// 保存集合配置
pub fn save_collection_config(config: &CollectionConfig) -> Result<(), ConfigError> {
    ConfigManager::instance().save_collection_config(config)
}

/// 获取实例配置
pub fn get_instance_config(instance_path: &str) -> Result<InstanceConfig, InstanceConfigError> {
    let path = Path::new(instance_path);
    ConfigManager::instance().load_instance_config(path)
}

/// 保存实例配置
pub fn save_instance_config(
    instance_path: &str,
    config: &InstanceConfig,
) -> Result<(), InstanceConfigError> {
    let path = Path::new(instance_path);
    ConfigManager::instance().save_instance_config(path, config)
}

/// 删除实例配置
pub fn delete_instance_config(instance_path: &str) -> Result<(), InstanceConfigError> {
    let path = Path::new(instance_path);
    ConfigManager::instance().delete_instance_config(path)
}
