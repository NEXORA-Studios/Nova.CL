use thiserror::Error;

/// 配置相关的错误类型
#[derive(Error, Debug)]
pub enum ConfigError {
    /// 配置文件读取错误
    #[error("Failed to read config file: {0}")]
    ReadError(std::io::Error),

    /// 配置文件写入错误
    #[error("Failed to write config file: {0}")]
    WriteError(std::io::Error),

    /// 配置解析错误
    #[error("Failed to parse config file: {0}")]
    ParseError(toml::de::Error),

    /// 配置序列化错误
    #[error("Failed to serialize config: {0}")]
    SerializeError(toml::ser::Error),

    /// UTF-8 解码错误
    #[error("Failed to decode UTF-8: {0}")]
    Utf8Error(std::string::FromUtf8Error),

    /// 路径错误
    #[error("Invalid config path: {0}")]
    PathError(String),

    /// 配置项缺失错误
    #[error("Missing required config field: {0}")]
    MissingField(String),

    /// 配置项无效错误
    #[error("Invalid config value for field {0}: {1}")]
    InvalidValue(String, String),

    /// 配置版本不兼容错误
    #[error("Config version mismatch: expected {0}, got {1}")]
    VersionMismatch(u32, u32),

    /// 加密错误
    #[error("Encryption error: {0}")]
    EncryptionError(String),

    /// 解密错误
    #[error("Decryption error: {0}")]
    DecryptionError(String),

    /// Base64 编码错误
    #[error("Base64 encoding error: {0}")]
    Base64EncodeError(base64::EncodeSliceError),

    /// Base64 解码错误
    #[error("Base64 decoding error: {0}")]
    Base64DecodeError(base64::DecodeError),

    /// 密钥生成错误
    #[error("Failed to generate encryption key: {0}")]
    KeyGenerationError(String),

    /// 配置类型不支持错误
    #[error("Unsupported config type: {0}")]
    UnsupportedType(String),

    /// 其他配置错误
    #[error("Other config error: {0}")]
    Other(String),
}

// 实现从各种错误类型到 ConfigError 的转换
impl From<std::io::Error> for ConfigError {
    fn from(e: std::io::Error) -> Self {
        ConfigError::ReadError(e)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(e: toml::de::Error) -> Self {
        ConfigError::ParseError(e)
    }
}

impl From<toml::ser::Error> for ConfigError {
    fn from(e: toml::ser::Error) -> Self {
        ConfigError::SerializeError(e)
    }
}

impl From<std::string::FromUtf8Error> for ConfigError {
    fn from(e: std::string::FromUtf8Error) -> Self {
        ConfigError::Utf8Error(e)
    }
}

impl From<base64::EncodeSliceError> for ConfigError {
    fn from(e: base64::EncodeSliceError) -> Self {
        ConfigError::Base64EncodeError(e)
    }
}

impl From<base64::DecodeError> for ConfigError {
    fn from(e: base64::DecodeError) -> Self {
        ConfigError::Base64DecodeError(e)
    }
}

/// 全局配置特定错误
#[derive(Error, Debug)]
pub enum GlobalConfigError {
    /// 通用配置错误
    #[error("Global config error: {0}")]
    ConfigError(#[from] ConfigError),

    /// 特定于全局配置的错误
    #[error("Invalid global config: {0}")]
    InvalidConfig(String),
}

/// 账户配置特定错误
#[derive(Error, Debug)]
pub enum ProfileConfigError {
    /// 通用配置错误
    #[error("Profile config error: {0}")]
    ConfigError(#[from] ConfigError),

    /// 账户不存在错误
    #[error("Profile not found: {0}")]
    ProfileNotFound(String),

    /// 无效的账户类型错误
    #[error("Invalid profile type: {0}")]
    InvalidProfileType(String),

    /// 账户已存在错误
    #[error("Profile already exists: {0}")]
    ProfileExists(String),
}

/// 实例配置特定错误
#[derive(Error, Debug)]
pub enum InstanceConfigError {
    /// 通用配置错误
    #[error("Instance config error: {0}")]
    ConfigError(#[from] ConfigError),

    /// 实例不存在错误
    #[error("Instance not found: {0}")]
    InstanceNotFound(String),

    /// 无效的实例配置错误
    #[error("Invalid instance config: {0}")]
    InvalidInstanceConfig(String),

    /// 实例已存在错误
    #[error("Instance already exists: {0}")]
    InstanceExists(String),
}
