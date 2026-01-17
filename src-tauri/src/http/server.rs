use axum::extract::State;
use axum::{extract::Request, http::StatusCode, response::IntoResponse, routing::get, Router};
use once_cell::sync::OnceCell;
use serde::Serialize;
use std::collections::HashMap;
use std::net::SocketAddr;
use tauri::path::BaseDirectory;
use tauri::{AppHandle, Emitter, Manager};
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio::task;
use url;

// 类型定义
type ServerHandle = task::JoinHandle<Result<(), Box<dyn std::error::Error + Send + Sync>>>;
type ShutdownSender = mpsc::Sender<()>;

struct AuthLaunchContext {
    lang: String,
}

struct ServerState {
    handle: ServerHandle,
    shutdown_tx: ShutdownSender,
    port: u16,
    app_handle: Option<AppHandle>,
    auth_context: Option<AuthLaunchContext>,
}

#[derive(Serialize, Clone)]
struct CodeReceivedPayload {
    path: String,
    query: HashMap<String, String>,
}

static GLOBAL_SERVER: OnceCell<Mutex<Option<ServerState>>> = OnceCell::new();

// ================== Handler ==================

async fn auth_callback_handler(
    State(app_handle): State<AppHandle>,
    req: Request,
) -> impl IntoResponse {
    let path = req.uri().path().to_string();

    let query = req
        .uri()
        .query()
        .map(|q| {
            url::form_urlencoded::parse(q.as_bytes())
                .into_owned()
                .collect::<HashMap<String, String>>()
        })
        .unwrap_or_default();

    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state_guard = global.lock().await;

    // 只在有 code 时 emit 事件（不关闭服务器）
    if let Some(server_state) = &mut *state_guard {
        if let Some(app_handle) = &server_state.app_handle {
            if query.contains_key("code") {
                let _ = app_handle.emit(
                    "oauth:code_received",
                    CodeReceivedPayload {
                        path: path.clone(),
                        query: query.clone(),
                    },
                );
            }
        }
    }

    // 返回 addons/auth/index.html
    let html_content = match &*state_guard {
        Some(server_state) => {
            // 先 resolve 资源路径
            let index_path_result = app_handle
                .path()
                .resolve("addons/auth/index.html", BaseDirectory::Resource);

            match index_path_result {
                Ok(index_path) => {
                    // 成功解析路径后，再读取文件
                    match std::fs::read_to_string(&index_path) {
                        Ok(mut content) => {
                            let lang = server_state
                                .auth_context
                                .as_ref()
                                .map(|ctx| ctx.lang.as_str())
                                .unwrap_or("en");

                            // 注入语言 meta 标签
                            content = content.replace(
                                "</head>",
                                &format!("<meta name=\"lang\" content=\"{}\"></head>", lang),
                            );

                            content // 成功返回修改后的 HTML
                        }
                        Err(e) => {
                            format!(
                            "<html><body><h1>Error</h1><p>Failed to read auth/index.html: {}</p></body></html>",
                            e
                        )
                        }
                    }
                }
                Err(e) => {
                    format!(
                    "<html><body><h1>Error</h1><p>Failed to resolve resource path: {}</p></body></html>",
                    e
                )
                }
            }
        }
        None => {
            "<html><body><h1>Error</h1><p>Server state not available</p></body></html>".to_string()
        }
    };

    (StatusCode::OK, axum::response::Html(html_content))
}

// 新增：前端调用此路由关闭服务器
async fn close_handler() -> impl IntoResponse {
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let state_guard = global.lock().await;

    if let Some(server_state) = &*state_guard {
        let tx = server_state.shutdown_tx.clone();
        tokio::spawn(async move {
            let _ = tx.send(()).await;
        });
    }

    (StatusCode::OK, "Server is shutting down...")
}

async fn assets_handler(
    State(app_handle): State<AppHandle>,
    axum::extract::Path(path): axum::extract::Path<String>,
) -> impl IntoResponse {
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let state_guard = global.lock().await;

    match &*state_guard {
        Some(_server_state) => {
            // 目标资源在打包后的相对路径：addons/auth/assets/xxx
            // path 是用户请求的路径部分，例如 "logo.png" 或 "styles/main.css"
            let resource_relative_path = format!("addons/auth/assets/{}", path);

            // 先 resolve 资源路径
            let asset_path_result = app_handle
                .path()
                .resolve(&resource_relative_path, BaseDirectory::Resource);

            match asset_path_result {
                Ok(asset_path) => {
                    // 成功解析到打包资源路径，再读取文件内容（二进制）
                    match std::fs::read(&asset_path) {
                        Ok(content) => {
                            let mime = mime_guess::from_path(&path)
                                .first_or_octet_stream()
                                .to_string();

                            (
                                StatusCode::OK,
                                [(axum::http::header::CONTENT_TYPE, mime)],
                                content,
                            )
                        }
                        Err(e) => {
                            // 读取失败（极少发生，除非资源损坏）
                            let error_html = format!(
                        "<html><body><h1>500 Internal Error</h1><p>Failed to read asset: {}</p></body></html>",
                        e
                    );
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                [(axum::http::header::CONTENT_TYPE, "text/html".to_string())],
                                error_html.into_bytes(),
                            )
                        }
                    }
                }
                Err(_) => {
                    // 资源未打包进去或路径错误 → 404
                    let error_html = "<html><body><h1>404 Not Found</h1></body></html>".to_string();
                    (
                        StatusCode::NOT_FOUND,
                        [(axum::http::header::CONTENT_TYPE, "text/html".to_string())],
                        error_html.into_bytes(),
                    )
                }
            }
        }
        None => (
            StatusCode::INTERNAL_SERVER_ERROR,
            [(axum::http::header::CONTENT_TYPE, "text/html".to_string())],
            "Server state not available".to_string().into_bytes(),
        ),
    }
}

async fn not_found_handler(_req: Request) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        axum::response::Html("<html><body><h1>404 Not Found</h1></body></html>".to_string()),
    )
}

// ================== Public API ==================

pub async fn start_server(app_handle: AppHandle, port: u16, lang: String) -> Result<(), String> {
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state = global.lock().await;

    if state.is_some() {
        return Err("Server is already running".to_string());
    }

    let (shutdown_tx, mut shutdown_rx) = mpsc::channel::<()>(1);

    let auth_context = AuthLaunchContext { lang };

    let app_handle_clone = app_handle.clone();

    let app = Router::new()
        .route("/oauth/callback", get(auth_callback_handler))
        .route("/close", get(close_handler)) // 新增
        .route("/assets/*path", get(assets_handler))
        .fallback(not_found_handler)
        .with_state(app_handle_clone);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .map_err(|e| e.to_string())?;

    let handle = task::spawn(async move {
        axum::serve(listener, app)
            .with_graceful_shutdown(async move {
                let _ = shutdown_rx.recv().await;
            })
            .await
            .map_err(|e| e.into())
    });

    *state = Some(ServerState {
        handle,
        shutdown_tx,
        port,
        app_handle: Some(app_handle),
        auth_context: Some(auth_context),
    });

    Ok(())
}

pub async fn stop_server() -> Result<(), String> {
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let mut state = global.lock().await;

    let server_state = state.take().ok_or("Server is not running")?;

    let _ = server_state.shutdown_tx.send(()).await;
    let _ = server_state.handle.await;

    Ok(())
}

pub async fn get_server_status() -> Option<u16> {
    let global = GLOBAL_SERVER.get_or_init(|| Mutex::new(None));
    let state = global.lock().await;
    state.as_ref().map(|s| s.port)
}
