use async_trait::async_trait;
use std::collections::HashMap;

/// 环境变量加载源的统一接口
#[async_trait]
pub trait EnvProvider: Send + Sync + 'static {
    /// 加载环境变量
    async fn load(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>>;

    /// 获取加载源名称
    fn name(&self) -> &'static str;

    /// 获取加载源优先级（数字越小优先级越高）
    fn priority(&self) -> u8;
}

/// 环境变量加载源实现 - 从系统环境变量加载
#[derive(Clone)]
#[allow(dead_code)]
pub struct EnvVarProvider;

#[async_trait]
impl EnvProvider for EnvVarProvider {
    async fn load(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        let mut env_vars = HashMap::new();
        for (key, value) in std::env::vars() {
            env_vars.insert(key, value);
        }
        Ok(env_vars)
    }

    fn name(&self) -> &'static str {
        "EnvVarProvider"
    }

    fn priority(&self) -> u8 {
        0 // 最高优先级
    }
}

/// 环境变量加载源实现 - 从 .env 文件加载
#[derive(Clone)]
pub struct DotEnvProvider {
    file_path: String,
}

impl DotEnvProvider {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

#[async_trait]
impl EnvProvider for DotEnvProvider {
    async fn load(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        // 使用 dotenvy 加载文件
        let mut env_map = HashMap::new();

        // 读取 .env 文件内容
        let content = tokio::fs::read_to_string(&self.file_path).await?;

        // 解析 .env 文件
        for line in content.lines() {
            let line = line.trim();
            // 跳过空行和注释
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // 解析键值对
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim().to_string();
                let value = value.trim().to_string();
                // 移除引号
                let value = value.strip_prefix('"').unwrap_or(&value);
                let value = value.strip_suffix('"').unwrap_or(value);
                env_map.insert(key, value.to_string());
            }
        }

        Ok(env_map)
    }

    fn name(&self) -> &'static str {
        "DotEnvProvider"
    }

    fn priority(&self) -> u8 {
        1 // 中等优先级
    }
}

/// 环境变量加载源实现 - 从硬编码字符串加载
#[derive(Clone)]
pub struct HardcodedProvider {
    env_vars: HashMap<String, String>,
}

impl HardcodedProvider {
    pub fn new(env_vars: HashMap<String, String>) -> Self {
        Self { env_vars }
    }

    // 或者更方便的构造方法：从 &[(&str, &str)] 创建
    pub fn from_pairs(pairs: &[(&str, &str)]) -> Self {
        let mut map = HashMap::new();
        for (k, v) in pairs {
            map.insert(k.to_string(), v.to_string());
        }
        Self { env_vars: map }
    }
}

#[async_trait]
impl EnvProvider for HardcodedProvider {
    async fn load(&self) -> Result<HashMap<String, String>, Box<dyn std::error::Error>> {
        Ok(self.env_vars.clone())
    }

    fn name(&self) -> &'static str {
        "HardcodedProvider"
    }

    fn priority(&self) -> u8 {
        2
    }
}
