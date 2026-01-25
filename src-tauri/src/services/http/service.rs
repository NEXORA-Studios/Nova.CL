use async_trait::async_trait;
use log::{error, info};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{async_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService, ServiceState};
use crate::services::http::client::HttpClient;
use crate::services::http::server::{get_server_status, start_server, stop_server};

// ================== HTTP 客户端服务 ==================

/// HTTP 客户端服务
#[derive(Clone)]
pub struct HttpService {
    state: Arc<Mutex<ServiceState>>,
    client: Arc<Mutex<Option<HttpClient>>>,
}

impl HttpService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
            client: Arc::new(Mutex::new(None)),
        }
    }

    /// 获取 HTTP 客户端实例
    async fn get_client(&self) -> Result<HttpClient, CommandError> {
        let guard = self.client.lock().await;
        guard.as_ref().ok_or_else(|| CommandError::Text("HTTP 客户端未初始化".to_string())).cloned()
    }
}

#[async_trait]
impl LifecycleService for HttpService {
    fn name(&self) -> &'static str {
        "HttpService"
    }

    fn priority(&self) -> i32 {
        90 // 低于 LogService 和 EnvService
    }

    async fn on_start(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        // 初始化 HTTP 客户端
        let mut client_guard = self.client.lock().await;
        *client_guard = Some(HttpClient::new());

        info!("HTTP 客户端服务组件已启动");
        *state_guard = ServiceState::Running;
    }

    async fn on_stop(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;

        // 清理 HTTP 客户端资源
        let mut client_guard = self.client.lock().await;
        *client_guard = None;

        info!("HTTP 客户端服务组件已停止");
        *state_guard = ServiceState::Stopped;
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map = HashMap::new();

        let self_arc = self.clone(); // 现在 clone 的是 Arc<HttpService>

        map.insert(
            "http_request".to_string(),
            async_cmd({
                let self_arc = self.clone();
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.request(&args).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_get".to_string(),
            async_cmd({
                let self_arc = self_arc.clone(); // 又 clone 一次给第二个 handler
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.get(&args.url, args.headers).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_post".to_string(),
            async_cmd({
                let self_arc = self_arc.clone(); // 又 clone 一次给第二个 handler
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.post(&args.url, args.headers, args.body).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_put".to_string(),
            async_cmd({
                let self_arc = self_arc.clone(); // 又 clone 一次给第二个 handler
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.put(&args.url, args.headers, args.body).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_delete".to_string(),
            async_cmd({
                let self_arc = self_arc.clone(); // 又 clone 一次给第二个 handler
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.delete(&args.url, args.headers).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_patch".to_string(),
            async_cmd({
                let self_arc = self_arc.clone(); // 又 clone 一次给第二个 handler
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let client = self_arc.get_client().await?;

                        if let CommandInput::HttpClientReq(args) = input {
                            Ok(CommandOutput::HttpClientResp(client.patch(&args.url, args.headers, args.body).await.map_err(CommandError::HttpClientErr)?))
                        } else {
                            Err(CommandError::Text("需要 HttpReq 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map
    }
}

// ================== HTTP 服务器服务 ==================

/// HTTP 服务器服务
#[derive(Clone)]
pub struct HttpServerService {
    state: Arc<Mutex<ServiceState>>,
    server_port: Arc<Mutex<Option<u16>>>,
}

impl HttpServerService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
            server_port: Arc::new(Mutex::new(None)),
        }
    }

    /// 启动 HTTP 服务器
    pub async fn start(&self, app_handle: AppHandle, port: u16, lang: String) -> Result<(), String> {
        match start_server(app_handle, port, lang).await {
            Ok(_) => {
                let mut port_guard = self.server_port.lock().await;
                *port_guard = Some(port);
                info!("HTTP 服务器已启动，监听端口 {}", port);
                Ok(())
            }
            Err(e) => {
                error!("HTTP 服务器启动失败: {}", e);
                Err(e)
            }
        }
    }

    /// 停止 HTTP 服务器
    pub async fn stop(&self) -> Result<(), String> {
        match stop_server().await {
            Ok(_) => {
                let mut port_guard = self.server_port.lock().await;
                *port_guard = None;
                info!("HTTP 服务器已停止");
                Ok(())
            }
            Err(e) => {
                error!("HTTP 服务器停止失败: {}", e);
                Err(e)
            }
        }
    }

    /// 获取 HTTP 服务器状态
    pub async fn status(&self) -> Option<u16> {
        get_server_status().await
    }
}

#[async_trait]
impl LifecycleService for HttpServerService {
    fn name(&self) -> &'static str {
        "HttpServerService"
    }

    fn priority(&self) -> i32 {
        80 // 低于 HttpService
    }

    async fn on_start(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        // 服务器服务初始化，但不自动启动服务器
        // 服务器将通过命令手动启动
        info!("HTTP 服务器服务组件已启动");
        *state_guard = ServiceState::Running;
    }

    async fn on_stop(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;

        // 停止服务器（如果正在运行）
        if let Some(port) = self.status().await {
            info!("HTTP 服务器服务组件正在停止，将自动清理残留在端口 {} 上的 HTTP 服务", port);
            if let Err(e) = self.stop().await {
                error!("HTTP 服务器服务组件停止失败: {}", e);
            }
        }

        info!("HTTP 服务器服务组件已停止");
        *state_guard = ServiceState::Stopped;
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map = HashMap::new();

        map.insert(
            "http_server_start".to_string(),
            async_cmd({
                let self_arc = self.clone();
                move |input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        if let CommandInput::HttpServerStart(args) = input {
                            self_arc.start(args.app_handle, args.port, args.lang.clone()).await.map_err(CommandError::Text)?;
                            Ok(CommandOutput::Json(json!({
                                "status": "ok",
                                "message": "HTTP server started",
                                "port": args.port
                            })))
                        } else {
                            Err(CommandError::Text("需要 HttpServerStart 输入".to_string()))
                        }
                    }
                }
            }),
        );

        map.insert(
            "http_server_stop".to_string(),
            async_cmd({
                let self_arc = self.clone();
                move |_input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        self_arc.stop().await.map_err(CommandError::Text)?;
                        Ok(CommandOutput::Json(json!({
                            "status": "ok",
                            "message": "HTTP server stopped"
                        })))
                    }
                }
            }),
        );

        map.insert(
            "http_server_status".to_string(),
            async_cmd({
                let self_arc = self.clone();
                move |_input: CommandInput| {
                    let self_arc = self_arc.clone();
                    async move {
                        let status = self_arc.status().await;
                        match status {
                            Some(port) => Ok(CommandOutput::Json(json!({
                                "status": "running",
                                "port": port
                            }))),
                            None => Ok(CommandOutput::Json(json!({
                                "status": "stopped"
                            }))),
                        }
                    }
                }
            }),
        );

        map
    }
}
