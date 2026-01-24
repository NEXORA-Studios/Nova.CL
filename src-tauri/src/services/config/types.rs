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