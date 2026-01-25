use std::future::Future;
use std::sync::Arc;
use std::{collections::HashMap, pin::Pin};

use tauri::AppHandle;

use crate::services::http::types::{HttpError, HttpRequest, HttpResponse};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceState {
    Created,
    Starting,
    Running,
    Stopping,
    Stopped,
}

#[derive(Debug)]
pub enum CommandInput {
    Args(Vec<String>),
    Json(serde_json::Value),
    HttpClientReq(HttpRequest),
    HttpServerStart(HttpStartArgs),
}

#[derive(Debug)]
pub struct HttpStartArgs {
    pub app_handle: AppHandle,
    pub port: u16,
    pub lang: String,
}

#[derive(Debug)]
pub enum CommandOutput {
    Text(String),
    Json(serde_json::Value),
    HttpClientResp(HttpResponse),
}

#[derive(Debug)]
pub enum CommandError {
    Text(String),
    Json(serde_json::Value),
    HttpClientErr(HttpError),
}

pub type CommandHashMap = HashMap<String, Arc<CommandHandler>>;

pub type CommandFuture = Pin<Box<dyn Future<Output = Result<CommandOutput, CommandError>> + Send + 'static>>;

pub type CommandHandler = dyn Fn(CommandInput) -> CommandFuture + Send + Sync + 'static;
