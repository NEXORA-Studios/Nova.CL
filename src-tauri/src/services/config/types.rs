use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 配置值类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<ConfigValue>),
    Object(HashMap<String, ConfigValue>),
    Null,
}

impl ConfigValue {
    pub fn as_object_mut(&mut self) -> Option<&mut HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Object(map) => Some(map),
            _ => None,
        }
    }

    pub fn into_object(self) -> Option<HashMap<String, ConfigValue>> {
        match self {
            ConfigValue::Object(map) => Some(map),
            _ => None,
        }
    }
}

/// 配置映射类型
pub type ConfigMap = HashMap<String, ConfigValue>;

/// 配置数据类型枚举
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConfigDataType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

/// 配置元数据结构体
#[derive(Debug, Clone)]
pub struct ConfigMetadata {
    pub toml_file: &'static str,        // TOML 文件名（不含扩展名）
    pub config_item: Vec<&'static str>, // 配置项名称
    pub data_type: ConfigDataType,      // 数据类型
    pub need_encrypt: bool,             // 是否需要加密
    pub default_value: ConfigValue,     // 默认值
    pub is_list: bool,                  // 是否为 [[list]] 数组表
}

/// 配置元数据列表
pub type ConfigMetadataList = &'static [ConfigMetadata];
