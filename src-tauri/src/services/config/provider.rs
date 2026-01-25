use async_trait::async_trait;
use log::error;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use toml::Value as TomlValue;

use crate::services::config::types::{ConfigMap, ConfigValue};

#[async_trait]
pub trait ConfigProvider: Send + Sync + 'static {
    async fn load(&self, app: &AppHandle) -> Result<ConfigMap, Box<dyn std::error::Error>>;
    async fn save(&self, app: &AppHandle, config: &ConfigMap) -> Result<(), Box<dyn std::error::Error>>;
    fn name(&self) -> &str;
    fn priority(&self) -> u8;
}

#[derive(Clone)]
pub struct TomlConfigProvider {
    pub(crate) file_name: String,
    id: String,
}

impl TomlConfigProvider {
    pub fn new(file_name: &str) -> Self {
        let id = format!("TomlConfigProvider/{}", file_name);
        Self { file_name: file_name.to_string(), id }
    }

    pub fn get_config_path(&self, app: &AppHandle) -> Result<PathBuf, Box<dyn std::error::Error>> {
        let app_data_dir = app.path().app_data_dir().map_err(|e| {
            error!("Failed to get app data directory: {:?}", e);
            Box::new(e) as Box<dyn std::error::Error>
        })?;
        let config_dir = app_data_dir.join("Config");
        std::fs::create_dir_all(&config_dir)?;
        Ok(config_dir.join(format!("{}.toml", self.file_name)))
    }

    fn toml_to_config_value(&self, toml_value: TomlValue) -> ConfigValue {
        match toml_value {
            TomlValue::String(s) => ConfigValue::String(s),
            TomlValue::Integer(i) => ConfigValue::Number(i as f64),
            TomlValue::Float(f) => ConfigValue::Number(f),
            TomlValue::Boolean(b) => ConfigValue::Boolean(b),
            TomlValue::Array(arr) => ConfigValue::Array(arr.into_iter().map(|v| self.toml_to_config_value(v)).collect()),
            TomlValue::Table(table) => ConfigValue::Object(table.into_iter().map(|(k, v)| (k, self.toml_to_config_value(v))).collect()),
            TomlValue::Datetime(dt) => ConfigValue::String(dt.to_string()),
        }
    }

    fn config_value_to_toml(&self, config_value: &ConfigValue) -> TomlValue {
        match config_value {
            ConfigValue::String(s) => TomlValue::String(s.clone()),
            ConfigValue::Number(n) => {
                if n.fract() == 0.0 {
                    TomlValue::Integer(*n as i64)
                } else {
                    TomlValue::Float(*n)
                }
            }
            ConfigValue::Boolean(b) => TomlValue::Boolean(*b),
            ConfigValue::Array(arr) => TomlValue::Array(arr.iter().map(|v| self.config_value_to_toml(v)).collect()),
            ConfigValue::Object(obj) => TomlValue::Table(obj.iter().map(|(k, v)| (k.clone(), self.config_value_to_toml(v))).collect()),
            ConfigValue::Null => TomlValue::String("null".to_string()),
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
        let toml_value: TomlValue = toml::from_str(&content)?;
        let mut config_map = ConfigMap::new();

        if let TomlValue::Table(table) = toml_value {
            // 直接递归转换成 ConfigValue::Object
            for (k, v) in table {
                config_map.insert(k, self.toml_to_config_value(v));
            }
        }
        Ok(config_map)
    }

    async fn save(&self, app: &AppHandle, config: &ConfigMap) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = self.get_config_path(app)?;
        let mut root = toml::value::Table::new();

        for (key, value) in config {
            let parts: Vec<&str> = key.split('.').collect();
            if parts.is_empty() {
                continue;
            }

            let mut current: &mut toml::value::Table = &mut root;

            // 逐层向下走，直到倒数第二层
            for &part in &parts[..parts.len() - 1] {
                let part_str = part.to_string();

                // 使用 entry API，但只借用一次
                let entry = current.entry(part_str).or_insert_with(|| TomlValue::Table(toml::value::Table::new()));

                // 强制转为 &mut Table（因为我们刚插入的肯定是 Table）
                current = match entry {
                    TomlValue::Table(table) => table,
                    _ => {
                        // 如果冲突，覆盖为新表（根据你的需求决定）
                        *entry = TomlValue::Table(toml::value::Table::new());
                        if let TomlValue::Table(table) = entry {
                            table
                        } else {
                            unreachable!("刚刚插入的应该是 Table")
                        }
                    }
                };
            }

            // 现在 current 是最后一层 table 的 &mut
            let last_key = parts.last().unwrap().to_string();
            let toml_value = self.config_value_to_toml(value);

            // 直接插入（这里不会和前面的 entry 冲突）
            current.insert(last_key, toml_value);
        }

        let content = toml::to_string_pretty(&toml::Value::Table(root))?;
        tokio::fs::write(config_path, content).await?;
        Ok(())
    }

    fn name(&self) -> &str {
        &self.id
    }
    fn priority(&self) -> u8 {
        1
    }
}
