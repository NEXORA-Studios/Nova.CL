use std::sync::{Arc, LazyLock};

use crate::services::env::{EnvProvider, HardcodedProvider};

// ────────────────────────────────────────────────
//                  环境变量提供器
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
