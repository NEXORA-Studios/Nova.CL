use async_trait::async_trait;
use log::info;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService, ServiceState};
use crate::services::system::ram::get_ram_info;

/// 系统服务，提供系统信息查询功能
#[derive(Clone)]
pub struct SystemService {
    state: Arc<Mutex<ServiceState>>,
}

impl SystemService {
    /// 创建新的系统服务实例
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
        }
    }
}

#[async_trait]
impl LifecycleService for SystemService {
    fn name(&self) -> &'static str {
        "SystemService"
    }

    fn priority(&self) -> i32 {
        85 // 低于 LogService 和 EnvService，高于 HttpService
    }

    async fn on_start(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        info!("系统服务组件已启动");
        *state_guard = ServiceState::Running;
    }

    async fn on_stop(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;
        // 系统服务无需特殊清理
        *state_guard = ServiceState::Stopped;
        info!("系统服务组件已停止");
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map: CommandHashMap = HashMap::new();

        // 获取内存信息命令
        map.insert(
            "get_ram_info".to_string(),
            sync_cmd(|_args: CommandInput| -> Result<CommandOutput, CommandError> {
                match serde_json::to_value(&get_ram_info()) {
                    Ok(v) => Ok(CommandOutput::Json(v)),
                    Err(e) => Err(CommandError::Text(format!("获取 RAM 信息失败: {:?}", e))),
                }
            }),
        );

        map
    }
}
