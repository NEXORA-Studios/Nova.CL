use async_trait::async_trait;
use std::collections::HashMap;
use std::path::PathBuf;
use tauri::api::path::app_data_dir;
use tauri::AppHandle;
use toml::Value as TomlValue;

use crate::services::config::types::{ConfigMap, ConfigValue};

/// 配置提供器接口
#[async_trait]
pub trait ConfigProvider: Send + Sync + 'static {
    /// 加载配置
    async fn load(&self, app: &AppHandle) -> Result<ConfigMap, Box<dyn std::error::Error>>;
    
    /// 保存配置
    async fn save(&self, app: &AppHandle, config: &ConfigMap) -> Result<(), Box<dyn std::error::Error>>;
    
    /// 获取提供器名称
    fn name(&self) -> &'static str;
    
    /// 获取提供器优先级（数字越小优先级越高）
    fn priority(&self) -> u8;
}

/// TOML 文件配置提供器
#[derive(Clone)]
pub struct TomlConfigProvider {
    pub(crate) file_name: String, // 不含扩展名的文件名
}

impl TomlConfigProvider {
    pub fn new(file_name: &str) -> Self {
        Self {
            file_name: file_name.to_string(),
        }
    }
    
    /// 获取配置文件完整路径
    fn get_config_path(&self, app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let app_data_dir = app_data_dir(app.config())
            .ok_or("Failed to get app data directory")?;
        
        let config_dir = app_data_dir.join("Config");
        std::fs::create_dir_all(&config_dir)?;
        
        Ok(config_dir.join(format!("{}.toml", self.file_name)))
    }
    
    /// 将 TOML 值转换为 ConfigValue
    fn toml_to_config_value(&self, toml_value: TomlValue) -> ConfigValue {
        match toml_value {
            TomlValue::String(s) => ConfigValue::String(s),
            TomlValue::Integer(i) => ConfigValue::Number(i as f64),
            TomlValue::Float(f) => ConfigValue::Number(f),
            TomlValue::Boolean(b) => ConfigValue::Boolean(b),
            TomlValue::Array(arr) => {
                ConfigValue::Array(arr.into_iter().map(|v| self.toml_to_config_value(v)).collect())
            },
            TomlValue::Table(table) => {
                ConfigValue::Object(
                    table.into_iter()
                        .map(|(k, v)| (k, self.toml_to_config_value(v)))
                        .collect()
                )
            },
            TomlValue::Datetime(dt) => ConfigValue::String(dt.to_string()),
        }
    }
    
    /// 将 ConfigValue 转换为 TOML 值
    fn config_value_to_toml(&self, config_value: ConfigValue) -> TomlValue {
        match config_value {
            ConfigValue::String(s) => TomlValue::String(s),
            ConfigValue::Number(n) => {
                if n.fract() == 0.0 {
                    TomlValue::Integer(n as i64)
                } else {
                    TomlValue::Float(n)
                }
            },
            ConfigValue::Boolean(b) => TomlValue::Boolean(b),
            ConfigValue::Array(arr) => {
                TomlValue::Array(arr.into_iter().map(|v| self.config_value_to_toml(v)).collect())
            },
            ConfigValue::Object(obj) => {
                TomlValue::Table(
                    obj.into_iter()
                        .map(|(k, v)| (k, self.config_value_to_toml(v)))
                        .collect()
                )
            },
            ConfigValue::Null => TomlValue::Table(toml::Table::new()),
        }
    }
}

#[async_trait]
impl ConfigProvider for TomlConfigProvider {
    async fn load(&self, app: &AppHandle) -> Result<ConfigMap, Box<dyn std::error::Error>> {
        let config_path = self.get_config_path(app)?;
        
        if !config_path.exists() {
            return Ok(ConfigMap::new());
        }
        
        let content = tokio::fs::read_to_string(config_path).await?;
        let toml_value = toml::from_str(&content)?;
        
        if let TomlValue::Table(table) = toml_value {
            let mut config_map = ConfigMap::new();
            
            for (key, value) in table {
                config_map.insert(key, self.toml_to_config_value(value));
            }
            
            Ok(config_map)
        } else {
            Ok(ConfigMap::new())
        }
    }
    
    async fn save(&self, app: &AppHandle, config: &ConfigMap) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.get_config_path(app)?;
        
        let mut toml_table = toml::Table::new();
        
        for (key, value) in config {
            toml_table.insert(key.clone(), self.config_value_to_toml(value.clone()));
        }
        
        let content = toml::to_string_pretty(&toml_table)?;
        tokio::fs::write(config_path, content).await?;
        
        Ok(())
    }
    
    fn name(&self) -> &'static str {
        "TomlConfigProvider"
    }
    
    fn priority(&self) -> u8 {
        1 // 中等优先级
    }
}