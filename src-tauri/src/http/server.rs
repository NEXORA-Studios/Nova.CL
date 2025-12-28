use axum::{
    extract::{Query, Request},
    http::StatusCode,
    response::IntoResponse,
    routing::get,
    Router,
};
use once_cell::sync::OnceCell;
use std::collections::HashMap;
use std::net::SocketAddr;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;
use tokio::sync::{mpsc, oneshot};
use tokio::task;

// 用于存储服务器任务的句柄
type ServerHandle = task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;
type ShutdownSender = mpsc::Sender<()>;

struct ServerState {
    handle: ServerHandle,
    shutdown_tx: ShutdownSender,
    port: u16,
    auth_code_tx: Option<oneshot::Sender<HashMap<String, String>>>,
    app_handle: Option<AppHandle>,
}

// 全局服务器状态，使用Mutex保护
static GLOBAL_SERVER: OnceCell<Mutex<Option<ServerState>>> = OnceCell::new();

// 简单的根路由处理函数
async fn root_handler() -> impl IntoResponse {
    (StatusCode::OK, "Nova.CL OAuth Redirect Server is running")
}

// OAuth 回调处理函数
async fn oauth_callback_handler(
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    // 提取授权代码和其他参数
    let _code = params.get("code").cloned();
    let _state = params.get("state").cloned();

    // 获取全局状态
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state_guard = global.lock().await;

    // 如果有等待的通道，发送授权代码
    if let Some(server_state) = &mut *state_guard {
        // 发送IPC事件到前端
        if let Some(app_handle) = &server_state.app_handle {
            // 发送OAuth授权代码事件
            app_handle
                .emit("oauth:code_received", &params)
                .unwrap_or_else(|e| {
                    eprintln!("Failed to emit oauth:code_received event: {}", e);
                });
        }

        if let Some(auth_code_tx) = server_state.auth_code_tx.take() {
            let _ = auth_code_tx.send(params.clone());

            // 发送关闭信号
            let _ = server_state.shutdown_tx.send(());
        }
    }

    // 返回一个简单的HTML响应，告知用户授权已完成
    (
        StatusCode::OK,
        axum::response::Html(
            r#"
            <!DOCTYPE html>
            <html>
            <head>
                <title>Authorization Complete</title>
                <style>
                    body { font-family: Arial, sans-serif; text-align: center; padding: 50px; }
                    h1 { color: #4CAF50; }
                    p { font-size: 18px; }
                </style>
            </head>
            <body>
                <h1>Authorization Complete</h1>
                <p>You can now close this window.</p>
            </body>
            </html>
        "#,
        ),
    )
}

// 处理所有其他路由的404处理函数
async fn not_found_handler(_req: Request) -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}

// 启动HTTP服务器，用于OAuth授权代码流
pub async fn start_server(
    app_handle: AppHandle,
    port: u16,
) -> Result<oneshot::Receiver<HashMap<String, String>>, String> {
    // 初始化全局状态
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state = global.lock().await;

    // 检查服务器是否已经在运行
    if state.is_some() {
        return Err("Server is already running".to_string());
    }

    // 创建关闭通道
    let (shutdown_tx, mut shutdown_rx) = mpsc::channel(1);

    // 创建授权代码接收通道
    let (auth_code_tx, auth_code_rx) = oneshot::channel();

    // 构建路由，添加OAuth回调路由
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/pclce_oauth", get(oauth_callback_handler))
        .route("/oauth/callback", get(oauth_callback_handler))
        .fallback(not_found_handler);

    // 绑定地址
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    // 创建服务器
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| e.to_string())?;

    // 保存服务器状态
    let handle = task::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                // 等待关闭信号
                let _ = shutdown_rx.recv().await;
            })
            .await
            .map_err(|e| e.into())
    });

    // 更新全局状态
    *state = Some(ServerState {
        handle,
        shutdown_tx,
        port,
        auth_code_tx: Some(auth_code_tx),
        app_handle: Some(app_handle),
    });

    // 返回接收授权代码的通道
    Ok(auth_code_rx)
}

// 停止HTTP服务器
pub async fn stop_server() -> Result<(), String> {
    // 初始化全局状态
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state = global.lock().await;

    // 检查服务器是否在运行
    let server_state = match state.take() {
        Some(s) => s,
        None => return Err("Server is not running".to_string()),
    };

    // 发送关闭信号
    if let Err(e) = server_state.shutdown_tx.send(()).await {
        return Err(format!("Failed to send shutdown signal: {}", e));
    }

    // 等待服务器停止
    if let Err(e) = server_state.handle.await {
        return Err(format!("Server task failed: {:?}", e));
    }

    Ok(())
}

// 检查服务器状态
pub async fn get_server_status() -> Option<u16> {
    // 初始化全局状态
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let state = global.lock().await;

    state.as_ref().map(|s| s.port)
}
