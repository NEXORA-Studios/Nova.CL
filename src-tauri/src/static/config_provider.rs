use std::sync::{Arc, LazyLock};

use crate::services::config::{ConfigProvider, TomlConfigProvider};

// ────────────────────────────────────────────────
// 配置文件提供器 - 改成 static LazyLock
// ────────────────────────────────────────────────
pub static CONFIG_PROVIDERS: LazyLock<Vec<Arc<dyn ConfigProvider + 'static>>> = LazyLock::new(|| {
    vec![
        Arc::new(TomlConfigProvider::new("Java")),
        Arc::new(TomlConfigProvider::new("Launch")),
        Arc::new(TomlConfigProvider::new("Customize")),
        Arc::new(TomlConfigProvider::new("Download")),
        Arc::new(TomlConfigProvider::new("Network")),
        Arc::new(TomlConfigProvider::new("Debug")),
        Arc::new(TomlConfigProvider::new("Profiles")),
        Arc::new(TomlConfigProvider::new("Instances")),
    ]
});
