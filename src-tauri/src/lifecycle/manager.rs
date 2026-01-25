use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::service::LifecycleService;
use crate::lifecycle::{CommandError, CommandHandler, CommandInput, CommandOutput, ServiceState};

#[derive(Clone)]
pub struct LifecycleManager {
    services: Arc<Mutex<Vec<Arc<dyn LifecycleService>>>>,
    commands: Arc<Mutex<HashMap<String, (Arc<dyn LifecycleService>, Arc<CommandHandler>)>>>,
}

impl LifecycleManager {
    pub fn new() -> Self {
        Self {
            services: Arc::new(Mutex::new(Vec::new())),
            commands: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn register(&self, service: Arc<dyn LifecycleService>) {
        let mut services = futures::executor::block_on(self.services.lock());
        services.push(service);
    }

    pub async fn startup(&self, app: &AppHandle) {
        let mut services = self.services.lock().await;
        // 按优先级排序（高优先级先启动）
        services.sort_by_key(|s| std::cmp::Reverse(s.priority()));

        for svc in services.iter() {
            svc.on_start(app).await;
        }

        // 收集所有命令（只在 Running 状态收集）
        let mut cmd_map = self.commands.lock().await;
        for svc in services.iter() {
            if svc.state() == ServiceState::Running {
                for (cmd_name, cmd_fn) in svc.commands().await {
                    cmd_map.insert(cmd_name, (svc.clone(), cmd_fn));
                }
            }
        }
    }

    pub async fn shutdown(&self, app: &AppHandle) {
        let services = self.services.lock().await;
        // 反序停止
        for svc in services.iter().rev() {
            svc.on_stop(app).await;
        }
    }

    pub async fn invoke_command(&self, cmd_name: &str, args: CommandInput) -> Result<CommandOutput, CommandError> {
        let cmd_map = self.commands.lock().await;

        if let Some((svc, handler)) = cmd_map.get(cmd_name) {
            if svc.state() != ServiceState::Running {
                return Err(CommandError::Text(format!("服务 {} 未运行 (状态: {:?})", svc.name(), svc.state())));
            }

            // 直接 .await
            let result = handler(args).await;

            result
        } else {
            Err(CommandError::Text(format!("未找到命令 '{}'", cmd_name)))
        }
    }
}
