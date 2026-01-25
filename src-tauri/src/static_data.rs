use std::sync::{Arc, LazyLock};

use crate::services::{
    config::{ConfigDataType, ConfigMetadata, ConfigProvider, ConfigValue, TomlConfigProvider},
    env::{EnvProvider, HardcodedProvider},
};

// ────────────────────────────────────────────────
// 环境变量提供器 - 改成 static LazyLock
// ────────────────────────────────────────────────
pub static ENV_PROVIDERS: LazyLock<Vec<Arc<dyn EnvProvider + 'static>>> = LazyLock::new(|| {
    let mut providers: Vec<Arc<dyn EnvProvider + 'static>> = Vec::new();

    // 硬编码字符串加载源 - 总是启用
    let hardcoded_provider = Arc::new(HardcodedProvider::from_pairs(&[
        ("NOVA_VERSION", "{{NOVA_VERSION}}"),
        ("NOVA_CHANNEL", "{{NOVA_CHANNEL}}"),
        ("APP_ENCRYPTION_KEY", "{{APP_ENCRYPTION_KEY}}"),
        ("OAUTH_REDIRECT_URI_BASE", "{{OAUTH_REDIRECT_URI_BASE}}"),
        ("OAUTH_MS_CLIENT_ID", "{{OAUTH_MS_CLIENT_ID}}"),
        ("OAUTH_MS_CLIENT_SECRET", "{{OAUTH_MS_CLIENT_SECRET}}"),
        ("OAUTH_MS_REDIRECT_URI", "{{OAUTH_MS_REDIRECT_URI}}"),
    ]));
    providers.push(hardcoded_provider);

    // Debug 模式: 启用 dotEnv 文件加载源
    #[cfg(debug_assertions)]
    {
        use crate::services::env::DotEnvProvider;
        let dotenv_provider = Arc::new(DotEnvProvider::new("../.env"));
        providers.push(dotenv_provider);
    }

    // Release 模式: 启用环境变量加载源
    #[cfg(not(debug_assertions))]
    {
        use crate::services::env::EnvVarProvider;
        let envvar_provider = Arc::new(EnvVarProvider);
        providers.push(envvar_provider);
    }

    providers
});

// ────────────────────────────────────────────────
// 配置文件提供器 - 改成 static LazyLock
// ────────────────────────────────────────────────
pub static CONFIG_PROVIDERS: LazyLock<Vec<Arc<dyn ConfigProvider + 'static>>> = LazyLock::new(|| {
    vec![
        Arc::new(TomlConfigProvider::new("Launch")),
        Arc::new(TomlConfigProvider::new("Customize")),
        Arc::new(TomlConfigProvider::new("Download")),
        Arc::new(TomlConfigProvider::new("Network")),
        Arc::new(TomlConfigProvider::new("Debug")),
        Arc::new(TomlConfigProvider::new("Profile")),
        Arc::new(TomlConfigProvider::new("Instance")),
        Arc::new(TomlConfigProvider::new("Collection")),
    ]
});

// ────────────────────────────────────────────────
// 配置元数据 - 直接用 LazyLock 包住宏展开的结果
// （这样里面就可以放心用 .to_string() / vec![] 了）
// ────────────────────────────────────────────────
pub static CONFIG_METADATA: LazyLock<Vec<ConfigMetadata>> = LazyLock::new(|| {
    vec![
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "VersionIndieType"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("disabled")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "LauncherVisibility"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("immediately_quit")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "PreferIpStack"],
            data_type: ConfigDataType::Number,
            need_encrypt: false,
            default_value: ConfigValue::Number(4.0),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "SelectedJava"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "Java"],
            data_type: ConfigDataType::Array,
            need_encrypt: false,
            default_value: ConfigValue::Array(vec![]),
            is_list: true,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Basic", "AutoRam"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(true),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Ram", "CustomRam"],
            data_type: ConfigDataType::Number,
            need_encrypt: false,
            default_value: ConfigValue::Number(8192.0),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Launch",
            config_item: vec!["Ram", "PreSwap"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(false),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Customize",
            config_item: vec!["UI", "Theme"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("auto")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Customize",
            config_item: vec!["UI", "Language"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("zh-CN")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["Source", "DownloadSource"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("offical")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["Source", "VersionSource"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("offical")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["Internet", "MaxConcurrent"],
            data_type: ConfigDataType::Number,
            need_encrypt: false,
            default_value: ConfigValue::Number(64.0),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["Internet", "MaxBandwidth"],
            data_type: ConfigDataType::Number,
            need_encrypt: false,
            default_value: ConfigValue::Number(-1.0),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["PostInstall", "SelectInstance"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(true),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Download",
            config_item: vec!["PostInstall", "UpdateAuthlib"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(true),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["DNS", "UseDoh"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(false),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["Proxy", "UseSystemProxy"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(false),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["Proxy", "UseCustomProxy"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(false),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["Proxy", "CustomProxyUri"],
            data_type: ConfigDataType::String,
            need_encrypt: false,
            default_value: ConfigValue::String(String::from("")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["Proxy", "CustomProxyAccount"],
            data_type: ConfigDataType::String,
            need_encrypt: true,
            default_value: ConfigValue::String(String::from("")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Network",
            config_item: vec!["Proxy", "CustomProxyPassword"],
            data_type: ConfigDataType::String,
            need_encrypt: true,
            default_value: ConfigValue::String(String::from("")),
            is_list: false,
        },
        ConfigMetadata {
            toml_file: "Debug",
            config_item: vec!["Root", "Enabled"],
            data_type: ConfigDataType::Boolean,
            need_encrypt: false,
            default_value: ConfigValue::Boolean(false),
            is_list: false,
        },
    ]
});
