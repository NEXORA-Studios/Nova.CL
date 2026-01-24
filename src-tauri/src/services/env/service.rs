use async_trait::async_trait;
use log::{debug, info, warn};
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{
    sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService,
    ServiceState,
};
use crate::services::env::provider::EnvProvider;

#[derive(Clone)]
pub struct EnvService {
    state: Arc<Mutex<ServiceState>>,
    providers: Arc<Mutex<Vec<Arc<dyn EnvProvider>>>>,
    env_vars: Arc<Mutex<HashMap<String, String>>>,
}

impl EnvService {
    /// 创建新的 EnvService，支持依赖注入
    pub fn new(providers: Vec<Arc<dyn EnvProvider>>) -> Self {
        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
            providers: Arc::new(Mutex::new(providers)),
            env_vars: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 合并多个环境变量映射，根据优先级覆盖
    fn merge_env_vars(
        &self,
        mut merged_vars: HashMap<String, String>,
        new_vars: HashMap<String, String>,
    ) -> HashMap<String, String> {
        merged_vars.extend(new_vars);
        merged_vars
    }
}

#[async_trait]
impl LifecycleService for EnvService {
    fn name(&self) -> &'static str {
        "EnvService"
    }

    fn priority(&self) -> i32 {
        100
    }

    async fn on_start(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        // 获取所有加载源并按优先级排序（优先级高的在后，因为会覆盖前面的）
        let mut providers = self.providers.lock().await;
        providers.sort_by_key(|p| p.priority());

        let mut merged_vars = HashMap::new();
        let mut loaded_providers = Vec::new();

        // 按优先级加载环境变量
        for provider in providers.iter() {
            match provider.load().await {
                Ok(env_vars) => {
                    let var_count = env_vars.len();
                    merged_vars = self.merge_env_vars(merged_vars, env_vars);
                    loaded_providers.push(provider.name());
                    debug!("从 {} 来源加载了 {} 个环境变量", provider.name(), var_count);
                }
                Err(e) => {
                    warn!("从 {} 加载环境变量失败: {}", provider.name(), e);
                }
            }
        }

        // 更新系统环境变量
        for (key, value) in &merged_vars {
            std::env::set_var(key, value);
        }

        // 存储合并后的环境变量
        let mut env_vars_guard = self.env_vars.lock().await;
        *env_vars_guard = merged_vars;

        info!("环境变量服务组件启动成功, 来源: {:?}", loaded_providers);
        *state_guard = ServiceState::Running;
    }

    async fn on_stop(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;
        // 环境变量服务无需特殊清理
        *state_guard = ServiceState::Stopped;
        info!("环境变量服务组件已停止");
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map: CommandHashMap = HashMap::new();

        // 每个命令都用这种固定模式
        map.insert(
            "get_env_var".to_string(),
            sync_cmd(
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.is_empty() {
                            return Err(CommandError::Text("缺少 key 参数".to_string()));
                        }
                        let key = args[0].clone();
                        std::env::var(key)
                            .map_err(|e| CommandError::Text(e.to_string()))
                            .map(|v| CommandOutput::Text(v))
                    } else {
                        Err(CommandError::Text("参数格式错误".to_string()))
                    }
                },
            ),
        );

        map.insert(
            "get_all_env_vars".to_string(),
            sync_cmd(
                |_args: CommandInput| -> Result<CommandOutput, CommandError> {
                    let vars: Vec<(String, String)> = std::env::vars().collect();
                    serde_json::to_string(&vars)
                        .map_err(|e| CommandError::Text(e.to_string()))
                        .map(|v| CommandOutput::Text(v))
                },
            ),
        );

        map.insert(
            "has_env_var".to_string(),
            sync_cmd(
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.is_empty() {
                            return Err(CommandError::Text("缺少 key 参数".to_string()));
                        }
                        let key = args[0].clone();
                        Ok(CommandOutput::Text(
                            std::env::var_os(&key).is_some().to_string(),
                        ))
                    } else {
                        Err(CommandError::Text("参数格式错误".to_string()))
                    }
                },
            ),
        );

        map
    }
}
