use crate::services::config::types::{ConfigDataType, ConfigValue};

/// 配置元数据结构体
#[derive(Debug, Clone)]
pub struct ConfigMetadata {
    pub toml_file: &'static str,      // TOML 文件名（不含扩展名）
    pub config_name: &'static str,     // 配置项名称
    pub data_type: ConfigDataType,     // 数据类型
    pub need_encrypt: bool,            // 是否需要加密
    pub default_value: ConfigValue,    // 默认值
}

/// 配置元数据列表类型
pub type ConfigMetadataList = &'static [ConfigMetadata];

/// 配置元数据宏，用于简化配置定义
#[macro_export]
macro_rules! config_metadata {
    (
        $(
            ($toml_file:expr, $config_name:expr, $data_type:expr, $need_encrypt:expr, $default_value:expr)
        ),*
        $(,
        )?
    ) => {
        &[
            $(
                $crate::services::config::metadata::ConfigMetadata {
                    toml_file: $toml_file,
                    config_name: $config_name,
                    data_type: $data_type,
                    need_encrypt: $need_encrypt,
                    default_value: $default_value,
                }
            ),*
        ]
    };
}