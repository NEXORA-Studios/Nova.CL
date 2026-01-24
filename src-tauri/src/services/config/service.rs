use async_trait::async_trait;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService, ServiceState};
use crate::services::config::provider::ConfigProvider;
use crate::services::config::types::{ConfigDataType, ConfigMap, ConfigValue};
use crate::services::config::metadata::{ConfigMetadata, ConfigMetadataList};
use crate::services::config::crypto::CryptoContext;

/// 配置服务
#[derive(Clone)]
pub struct ConfigService {
    state: Arc<Mutex<ServiceState>>,
    app_handle: Arc<Mutex<Option<AppHandle>>>,
    providers: Arc<Mutex<Vec<Arc<dyn ConfigProvider>>>>,
    config: Arc<Mutex<ConfigMap>>,
    metadata: ConfigMetadataList,
    crypto_context: Option<CryptoContext>,
}

impl ConfigService {
    /// 创建新的配置服务实例
    pub fn new(
        providers: Vec<Arc<dyn ConfigProvider>>,
        metadata: ConfigMetadataList
    ) -> Self {
        // 初始加密密钥使用随机生成的值
        let random_key = CryptoContext::generate_random_key();
        let crypto_context = tokio::runtime::Runtime::new().unwrap().block_on(
            CryptoContext::new(random_key)
        );
        
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
            default_config.insert(meta.config_name.to_string(), meta.default_value.clone());
        }
        
        default_config
    }
    
    /// 合并配置
    fn merge_configs(&self, mut merged: ConfigMap, new: ConfigMap) -> ConfigMap {
        merged.extend(new);
        merged
    }
    
    /// 验证配置类型
    fn validate_config_type(&self, key: &str, value: &ConfigValue) -> bool {
        for meta in self.metadata {
            if meta.config_name == key {
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
        true // 未知配置项，不验证类型
    }
    
    /// 获取配置值
    async fn get_config(&self, key: &str) -> Option<ConfigValue> {
        let config_guard = self.config.lock().await;
        config_guard.get(key).cloned()
    }
    
    /// 设置配置值
    async fn set_config(&self, key: &str, value: ConfigValue) -> Result<(), Box<dyn std::error::Error>> {
        // 验证类型
        if !self.validate_config_type(key, &value) {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("配置项 {} 类型不匹配", key)
            )));
        }
        
        // 检查是否需要加密
        let need_encrypt = self.metadata.iter()
            .find(|meta| meta.config_name == key)
            .map(|meta| meta.need_encrypt)
            .unwrap_or(false);
        
        let processed_value = if need_encrypt {
            // 加密处理
            if let ConfigValue::String(plaintext) = &value {
                if let Some(crypto_context) = &self.crypto_context {
                    let encrypted = crypto_context.encrypt_value(plaintext).await?;
                    ConfigValue::String(encrypted)
                } else {
                    value // 加密上下文未初始化，直接返回
                }
            } else {
                value // 只加密字符串类型
            }
        } else {
            value
        };
        
        // 更新配置
        let mut config_guard = self.config.lock().await;
        config_guard.insert(key.to_string(), processed_value.clone());
        
        // 保存到对应的 TOML 文件
        if let Some(meta) = self.metadata.iter().find(|m| m.config_name == key) {
            let provider = super::provider::TomlConfigProvider::new(meta.toml_file);
            
            // 获取 AppHandle
            let app_handle_guard = self.app_handle.lock().await;
            let app_handle = app_handle_guard.as_ref()
                .ok_or("AppHandle not initialized")?;
            
            // 只保存当前文件的配置
            let mut file_config = ConfigMap::new();
            for (k, v) in config_guard.iter() {
                if let Some(m) = self.metadata.iter().find(|m| m.config_name == k) {
                    if m.toml_file == meta.toml_file {
                        file_config.insert(k.clone(), v.clone());
                    }
                }
            }
            
            provider.save(app_handle, &file_config).await?;
        }
        
        Ok(())
    }
    
    /// 获取解密后的配置值
    async fn get_decrypted_config(&self, key: &str) -> Result<Option<ConfigValue>, Box<dyn std::error::Error>> {
        if let Some(value) = self.get_config(key).await {
            // 检查是否需要解密
            let need_encrypt = self.metadata.iter()
                .find(|meta| meta.config_name == key)
                .map(|meta| meta.need_encrypt)
                .unwrap_or(false);
            
            if need_encrypt {
                // 解密处理
                if let ConfigValue::String(encrypted) = value {
                    if let Some(crypto_context) = &self.crypto_context {
                        let plaintext = crypto_context.decrypt_value(&encrypted).await?;
                        Ok(Some(ConfigValue::String(plaintext)))
                    } else {
                        Ok(Some(ConfigValue::String(encrypted)))
                    }
                } else {
                    Ok(Some(value)) // 非字符串类型，直接返回
                }
            } else {
                Ok(Some(value)) // 不需要解密，直接返回
            }
        } else {
            Ok(None)
        }
    }
    
    /// 从环境变量获取加密密钥盐
    fn get_encryption_key_salt(&self) -> String {
        // 从系统环境变量获取 APP_ENCRYPTION_KEY
        std::env::var("APP_ENCRYPTION_KEY").unwrap_or_else(|_| {
            // 如果环境变量不存在，使用默认值
            warn!("APP_ENCRYPTION_KEY 环境变量未设置，使用默认值");
            "novacl_default_encryption_key_salt".to_string()
        })
    }
}

#[async_trait]
impl LifecycleService for ConfigService {
    fn name(&self) -> &'static str {
        "ConfigService"
    }
    
    fn priority(&self) -> i32 {
        90 // 优先级高于 HttpService，低于 EnvService (EnvService 优先级为 100)
    }
    
    async fn on_start(&self, app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;
        
        // 保存 AppHandle
        let mut app_handle_guard = self.app_handle.lock().await;
        *app_handle_guard = Some(app.clone());
        
        // 从环境变量获取加密密钥盐
        let salt = self.get_encryption_key_salt();
        
        // 初始化默认配置
        let default_config = self.init_default_config();
        
        // 按优先级排序提供器
        let mut providers = self.providers.lock().await;
        providers.sort_by_key(|p| p.priority());
        
        let mut merged_config = default_config;
        let mut loaded_providers = Vec::new();
        
        // 加载所有配置
        for provider in providers.iter() {
            match provider.load(app).await {
                Ok(config) => {
                    let config_count = config.len();
                    merged_config = self.merge_configs(merged_config, config);
                    loaded_providers.push(provider.name());
                    debug!("从 {} 加载了 {} 个配置项", provider.name(), config_count);
                }
                Err(e) => {
                    warn!("从 {} 加载配置失败: {}", provider.name(), e);
                }
            }
        }
        
        // 另外，加载所有已知的 TOML 文件
        let mut file_providers = Vec::new();
        for meta in self.metadata {
            let provider = super::provider::TomlConfigProvider::new(meta.toml_file);
            if !file_providers.iter().any(|p: &super::provider::TomlConfigProvider| p.file_name == meta.toml_file) {
                file_providers.push(provider);
            }
        }
        
        for provider in file_providers {
            match provider.load(app).await {
                Ok(config) => {
                    let config_count = config.len();
                    merged_config = self.merge_configs(merged_config, config);
                    debug!("从 TOML 文件 {} 加载了 {} 个配置项", provider.file_name, config_count);
                }
                Err(e) => {
                    warn!("从 TOML 文件 {} 加载配置失败: {}", provider.file_name, e);
                }
            }
        }
        
        // 存储合并后的配置
        let mut config_guard = self.config.lock().await;
        *config_guard = merged_config;
        
        info!("配置服务启动成功，加载了 {} 个配置项", config_guard.len());
        *state_guard = ServiceState::Running;
    }
    
    async fn on_stop(&self, app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;
        
        // 按 TOML 文件分组保存配置
        let config_guard = self.config.lock().await;
        let mut file_configs: HashMap<&str, ConfigMap> = HashMap::new();
        
        for (key, value) in config_guard.iter() {
            if let Some(meta) = self.metadata.iter().find(|m| m.config_name == key) {
                let file_config = file_configs.entry(meta.toml_file).or_insert(ConfigMap::new());
                file_config.insert(key.clone(), value.clone());
            }
        }
        
        // 保存每个 TOML 文件
        for (file_name, config) in file_configs {
            let provider = super::provider::TomlConfigProvider::new(file_name);
            if let Err(e) = provider.save(app, &config).await {
                warn!("保存配置文件 {} 失败: {}", file_name, e);
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
        let service = self.clone();
        
        // 获取配置值命令
        map.insert(
            "get_config".to_string(),
            sync_cmd(
                move |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.is_empty() {
                            return Err(CommandError::Text("缺少 key 参数".to_string()));
                        }
                        let key = args[0].clone();
                        
                        let result = tokio::task::block_in_place(|| {
                            futures::executor::block_on(async move {
                                service.get_decrypted_config(&key).await
                            })
                        });
                        
                        match result {
                            Ok(Some(value)) => {
                                serde_json::to_string(&value)
                                    .map_err(|e| CommandError::Text(e.to_string()))
                                    .map(|v| CommandOutput::Text(v))
                            },
                            Ok(None) => Ok(CommandOutput::Text("null".to_string())),
                            Err(e) => Err(CommandError::Text(e.to_string())),
                        }
                    } else {
                        Err(CommandError::Text("参数格式错误".to_string()))
                    }
                },
            ),
        );
        
        // 设置配置值命令
        map.insert(
            "set_config".to_string(),
            sync_cmd(
                move |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text("缺少 key 或 value 参数".to_string()));
                        }
                        let key = args[0].clone();
                        let value_str = args[1].clone();
                        
                        let value: ConfigValue = serde_json::from_str(&value_str)
                            .map_err(|e| CommandError::Text(format!("解析 value 失败: {}", e)))?;
                        
                        let result = tokio::task::block_in_place(|| {
                            futures::executor::block_on(async move {
                                service.set_config(&key, value).await
                            })
                        });
                        
                        result
                            .map_err(|e| CommandError::Text(e.to_string()))
                            .map(|_| CommandOutput::Text("ok".to_string()))
                    } else {
                        Err(CommandError::Text("参数格式错误".to_string()))
                    }
                },
            ),
        );
        
        map
    }
}