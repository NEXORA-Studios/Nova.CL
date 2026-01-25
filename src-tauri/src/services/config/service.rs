use async_trait::async_trait;
use log::{debug, info, trace, warn};
use serde_json::Value as SerdeJsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService, ServiceState};
use crate::services::config::{
    crypto::CryptoContext,
    provider::TomlConfigProvider,
    types::{ConfigDataType, ConfigMap, ConfigMetadataList, ConfigValue},
};
use crate::services::config::{ConfigMetadata, ConfigProvider};

/// 配置服务
#[derive(Clone)]
pub struct ConfigService {
    state: Arc<Mutex<ServiceState>>,
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    providers: Arc<Mutex<Vec<Arc<dyn crate::services::config::provider::ConfigProvider>>>>,
    config: Arc<Mutex<ConfigMap>>,
    metadata: ConfigMetadataList,
    crypto_context: Option<CryptoContext>,
}

impl ConfigService {
    pub fn new(providers: Vec<Arc<dyn crate::services::config::provider::ConfigProvider>>, metadata: ConfigMetadataList) -> Self {
        let random_key = CryptoContext::generate_random_key();
        let crypto_context = tokio::runtime::Runtime::new().unwrap().block_on(CryptoContext::new(random_key));

        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
            app_handle: Arc::new(Mutex::new(None)),
            providers: Arc::new(Mutex::new(providers)),
            config: Arc::new(Mutex::new(ConfigMap::new())),
            metadata,
            crypto_context: Some(crypto_context),
        }
    }

    /// 初始化默认配置
    fn init_default_config(&self) -> ConfigMap {
        let mut default_config = ConfigMap::new();
        for meta in self.metadata {
            let mut value = meta.default_value.clone();
            if meta.is_list && !matches!(value, ConfigValue::Array(_)) {
                value = ConfigValue::Array(vec![value]);
            }
            Self::set_by_path(self, &mut default_config, &meta.config_item, value, self.metadata);
        }
        default_config
    }

    fn merge_configs(&self, mut merged: ConfigMap, new: ConfigMap) -> ConfigMap {
        merged.extend(new);
        merged
    }

    fn merge_configs_if_exists(&self, mut merged: ConfigMap, new: ConfigMap) -> ConfigMap {
        for (key, value) in new {
            if let Some(existing) = merged.get_mut(&key) {
                match existing {
                    ConfigValue::Array(old) => {
                        if let ConfigValue::Array(mut new_array) = value {
                            old.append(&mut new_array);
                        } else {
                            *existing = value;
                        }
                    }
                    _ => {
                        *existing = value;
                    }
                }
            }
        }
        merged
    }

    fn validate_config_type(&self, path: &[&str], value: &ConfigValue) -> bool {
        for meta in self.metadata {
            if meta.config_item == path {
                match (&meta.data_type, value) {
                    (ConfigDataType::String, ConfigValue::String(_)) => return true,
                    (ConfigDataType::Number, ConfigValue::Number(_)) => return true,
                    (ConfigDataType::Boolean, ConfigValue::Boolean(_)) => return true,
                    (ConfigDataType::Array, ConfigValue::Array(_)) => return true,
                    (ConfigDataType::Object, ConfigValue::Object(_)) => return true,
                    _ => return false,
                }
            }
        }
        true
    }

    pub async fn get_config(&self, path: &[&str]) -> Option<ConfigValue> {
        let guard = self.config.lock().await;
        Self::get_by_path(&*guard, path).cloned()
    }

    pub async fn set_config(&self, path: &[&str], value: ConfigValue) -> Result<(), Box<dyn std::error::Error>> {
        // 校验类型
        if !self.validate_config_type(path, &value) {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::InvalidData, "类型不匹配")));
        }

        let need_encrypt = self.metadata.iter().find(|meta| meta.config_item == path).map(|m| m.need_encrypt).unwrap_or(false);

        let processed_value = if need_encrypt {
            if let ConfigValue::String(s) = &value {
                if let Some(crypto) = &self.crypto_context {
                    let encrypted = crypto.encrypt_value(s).await?;
                    ConfigValue::String(encrypted)
                } else {
                    value.clone()
                }
            } else {
                value.clone()
            }
        } else {
            value.clone()
        };

        // 更新内存配置
        let mut guard = self.config.lock().await;
        let file_config = Self::set_by_path(self, &mut *guard, path, processed_value.clone(), self.metadata);

        // 保存到文件，合并原有文件配置，避免丢失其它字段
        if let Some(meta) = self.metadata.iter().find(|m| m.config_item == path) {
            let provider = TomlConfigProvider::new(meta.toml_file);
            let app_guard = self.app_handle.lock().await;
            let app = app_guard.as_ref().ok_or("AppHandle not initialized")?;
            // 4. 保存合并后的文件
            provider.save(app, &file_config).await?;
        }

        Ok(())
    }

    pub async fn get_decrypted_config(&self, path: &[&str]) -> Result<Option<ConfigValue>, Box<dyn std::error::Error>> {
        if let Some(value) = self.get_config(path).await {
            let need_encrypt = self.metadata.iter().find(|meta| meta.config_item == path).map(|m| m.need_encrypt).unwrap_or(false);
            if need_encrypt {
                if let ConfigValue::String(s) = value {
                    if let Some(crypto) = &self.crypto_context {
                        let decrypted = crypto.decrypt_value(&s).await.map_err(|e| format!("解密失败: {}", e))?;
                        Ok(Some(ConfigValue::String(decrypted)))
                    } else {
                        Ok(Some(ConfigValue::String(s)))
                    }
                } else {
                    Ok(Some(value))
                }
            } else {
                Ok(Some(value))
            }
        } else {
            Ok(None)
        }
    }

    fn get_by_path<'a>(config: &'a ConfigMap, path: &[&str]) -> Option<&'a ConfigValue> {
        let mut current = config;

        for key in &path[..path.len() - 1] {
            match current.get(*key) {
                Some(ConfigValue::Object(obj)) => current = obj,
                _ => return None,
            }
        }

        current.get(*path.last().unwrap())
    }

    fn set_by_path(&self, config: &mut ConfigMap, path: &[&str], value: ConfigValue, metadata: &[ConfigMetadata]) -> HashMap<String, ConfigValue> {
        if path.is_empty() {
            return HashMap::new();
        }

        // 根据 metadata 判断是否要包成 Array
        let mut final_value = value;
        if let Some(meta) = metadata.iter().find(|m| m.config_item == path) {
            if meta.is_list && !matches!(&final_value, ConfigValue::Array(_)) {
                final_value = ConfigValue::Array(vec![final_value]);
            }
        }

        // 从最内层开始递归生成嵌套对象
        let mut nested_value = final_value;
        for _key in path[..path.len() - 1].iter().rev() {
            let mut map = HashMap::new();
            map.insert(path[path.len() - 1].to_string(), nested_value);
            nested_value = ConfigValue::Object(map);
        }

        // 最外层 key
        let top_key = path[0].to_string();
        let mut new_config = ConfigMap::new();
        new_config.insert(top_key, nested_value);

        // 使用已有 merge_configs_if_exists 合并
        let merged = Self::merge_configs_if_exists(self, config.clone(), new_config);
        *config = merged.clone();

        merged
    }
}

#[async_trait]
impl LifecycleService for ConfigService {
    fn name(&self) -> &'static str {
        "ConfigService"
    }
    fn priority(&self) -> i32 {
        90
    }

    async fn on_start(&self, app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        let mut app_guard = self.app_handle.lock().await;
        *app_guard = Some(app.clone());

        let default_config = self.init_default_config();

        let mut providers = self.providers.lock().await;
        providers.sort_by_key(|p| p.priority());

        let mut merged_config = default_config;

        for provider in providers.iter() {
            match provider.load(app).await {
                Ok(config) => {
                    let count = config.len();
                    merged_config = self.merge_configs(merged_config, config);
                    debug!("从 {} 加载了 {} 个配置项", provider.name(), count);
                }
                Err(e) => warn!("从 {} 加载配置失败: {}", provider.name(), e),
            }
        }

        let mut guard = self.config.lock().await;
        *guard = merged_config;

        // 收集缺失的文件和默认配置
        let mut missing_files: HashMap<&str, ConfigMap> = HashMap::new();
        for meta in self.metadata {
            let provider = TomlConfigProvider::new(meta.toml_file);
            let path = match provider.get_config_path(app) {
                Ok(p) => p,
                Err(e) => {
                    warn!("无法获取路径 {}: {}", meta.toml_file, e);
                    continue;
                }
            };

            if !path.exists() {
                let entry = missing_files.entry(meta.toml_file).or_insert_with(ConfigMap::new);
                entry.insert(meta.config_item.join("."), meta.default_value.clone());
            }
        }

        // 对于缺失的文件，写入默认配置
        for (file_name, default_map) in missing_files {
            if !default_map.is_empty() {
                let provider = TomlConfigProvider::new(file_name);
                if let Err(e) = provider.save(app, &default_map).await {
                    warn!("初始化缺失文件 {}.toml 失败: {}", file_name, e);
                } else {
                    info!("已创建缺失文件 {}.toml 并写入默认配置 ({} 项)", file_name, default_map.len());
                }
            }
        }

        *state_guard = ServiceState::Running;
        info!("配置服务启动成功，加载了 {} 个配置项", guard.len());
    }

    async fn on_stop(&self, app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;

        let guard = self.config.lock().await;

        for meta in self.metadata {
            let provider = TomlConfigProvider::new(meta.toml_file);
            let current_provider_config = provider.load(app).await.unwrap_or_default();
            // 只合并是目前 Toml 文件的配置
            let merged_config = self.merge_configs_if_exists(current_provider_config, guard.clone());
            if let Err(e) = provider.save(app, &merged_config).await {
                warn!("保存 {} 配置失败: {}", meta.toml_file, e);
            }
        }

        *state_guard = ServiceState::Stopped;
        info!("配置服务已停止");
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map: CommandHashMap = HashMap::new();

        let service_get = Arc::new(self.clone());
        map.insert(
            "get_config".to_string(),
            sync_cmd(move |args: CommandInput| -> Result<CommandOutput, CommandError> {
                if let CommandInput::Args(args) = args {
                    if args.is_empty() {
                        return Err(CommandError::Text("缺少 path 参数".to_string()));
                    }
                    let path_json = &args[0];
                    let path: Vec<String> = serde_json::from_str(path_json).map_err(|e| CommandError::Text(format!("解析 path 失败: {}", e)))?;
                    let path_ref: Vec<&str> = path.iter().map(|s| s.as_str()).collect();
                    let svc = service_get.clone();
                    let res = tokio::task::block_in_place(|| futures::executor::block_on(svc.get_decrypted_config(&path_ref)));
                    match res {
                        Ok(Some(v)) => Ok(CommandOutput::Text(serde_json::to_string(&v).unwrap_or("null".to_string()))),
                        Ok(None) => Ok(CommandOutput::Text("null".to_string())),
                        Err(e) => Err(CommandError::Text(e.to_string())),
                    }
                } else {
                    Err(CommandError::Text("参数格式错误".to_string()))
                }
            }),
        );

        let service_set = Arc::new(self.clone());
        map.insert(
            "set_config".to_string(),
            sync_cmd(move |args: CommandInput| -> Result<CommandOutput, CommandError> {
                trace!("set_config 命令参数: {:#?}", args);
                if let CommandInput::Args(args) = args {
                    if args.len() < 2 {
                        return Err(CommandError::Text("缺少 path 或 value 参数".to_string()));
                    }
                    let path_vec = (&args[0]).split(".").map(|s| s.to_string()).collect::<Vec<String>>();
                    let path_json = serde_json::to_string(&path_vec).unwrap_or("null".to_string());
                    let value_json = &args[1];
                    let path: Vec<String> = serde_json::from_str(&path_json).map_err(|e| CommandError::Text(format!("解析 path 失败: {}", e)))?;
                    let path_ref: Vec<&str> = path.iter().map(|s| s.as_str()).collect();
                    let raw_value: SerdeJsonValue = serde_json::from_str(value_json).map_err(|e| CommandError::Text(format!("解析 value 失败: {}", e)))?;
                    let value: ConfigValue = match raw_value {
                        SerdeJsonValue::String(s) => ConfigValue::String(s),
                        SerdeJsonValue::Number(n) => ConfigValue::Number(n.as_f64().unwrap_or(0.0)),
                        SerdeJsonValue::Bool(b) => ConfigValue::Boolean(b),
                        SerdeJsonValue::Array(a) => ConfigValue::Array(a.into_iter().map(|v: SerdeJsonValue| serde_json::from_str(&v.to_string()).unwrap_or(ConfigValue::Null)).collect()),
                        SerdeJsonValue::Object(o) => ConfigValue::Object(
                            o.into_iter()
                                .map(|(k, v): (String, SerdeJsonValue)| (k, serde_json::from_str(&v.to_string()).unwrap_or(ConfigValue::Null)))
                                .collect(),
                        ),
                        SerdeJsonValue::Null => ConfigValue::Null,
                    };
                    let svc = service_set.clone();
                    let res = tokio::task::block_in_place(|| futures::executor::block_on(svc.set_config(&path_ref, value)));
                    res.map(|_| CommandOutput::Text("ok".to_string())).map_err(|e| CommandError::Text(e.to_string()))
                } else {
                    Err(CommandError::Text("参数格式错误".to_string()))
                }
            }),
        );

        map
    }
}
