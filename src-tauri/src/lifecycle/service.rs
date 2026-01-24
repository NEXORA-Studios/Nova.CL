use async_trait::async_trait;
use tauri::AppHandle;

use crate::lifecycle::{CommandHashMap, ServiceState};

#[async_trait]
pub trait LifecycleService: Send + Sync + 'static {
    fn name(&self) -> &'static str;

    fn priority(&self) -> i32 {
        0
    }

    async fn on_start(&self, app: &AppHandle);

    async fn on_stop(&self, app: &AppHandle);

    fn state(&self) -> ServiceState;

    async fn commands(&self) -> CommandHashMap;
}
