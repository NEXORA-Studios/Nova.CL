use serde::{Deserialize, Serialize};

use crate::r#static::ErrCodes;

/// 协议版本
pub const IPC_VERSION: u8 = 1;

/// 消息状态
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum IpcStatus {
    Ok,
    Error,
}

/// 错误信息结构
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IpcError {
    /// 错误码（模块内唯一）
    pub code: ErrCodes,
    /// 错误模块
    pub module: String,
    /// 面向用户的错误描述
    pub user_message: String,
    /// 面向开发者的错误描述
    pub dev_message: String,
    /// 详细错误信息，可选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    /// 是否可重试
    #[serde(default)]
    pub retryable: bool,
}

/// 前端请求的响应
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallResponse<T> {
    /// 协议版本号
    pub version: u8,
    /// 对应 request 的 id
    pub id: u64,
    /// 命令名（用于日志/调试）
    pub topic: String,
    /// 状态
    pub status: IpcStatus,
    /// 成功时的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
    /// 错误信息，仅 status = Error 时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcError>,
    /// 时间戳（Unix ms）
    pub timestamp: u64,
}

/// 后端主动推送事件
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EmitEvent<T> {
    /// 协议版本号
    pub version: u8,
    /// 事件名
    pub topic: String,
    /// 状态
    pub status: IpcStatus,
    /// 成功时的数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
    /// 错误信息，仅 status = Error 时存在
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<IpcError>,
    /// 时间戳（Unix ms）
    pub timestamp: u64,
}

/// 前端请求后端的消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CallRequest<T> {
    /// 协议版本号
    pub version: u8,
    /// 唯一 ID
    pub id: u64,
    /// 命令名
    pub topic: String,
    /// 请求数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
    /// 时间戳（Unix ms）
    pub timestamp: u64,
}

/// 前端接收的统一消息类型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IpcMessage<T> {
    Call(CallResponse<T>),
    Event(EmitEvent<T>),
}

impl<T> CallResponse<T> {
    /// 创建成功响应
    pub fn ok(id: u64, topic: String, payload: T) -> Self {
        Self {
            version: IPC_VERSION,
            id,
            topic,
            status: IpcStatus::Ok,
            payload: Some(payload),
            error: None,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建错误响应
    pub fn error(id: u64, topic: String, error: IpcError) -> Self {
        Self {
            version: IPC_VERSION,
            id,
            topic,
            status: IpcStatus::Error,
            payload: None,
            error: Some(error),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}

impl<T> EmitEvent<T> {
    /// 创建成功事件
    pub fn ok(topic: String, payload: T) -> Self {
        Self {
            version: IPC_VERSION,
            topic,
            status: IpcStatus::Ok,
            payload: Some(payload),
            error: None,
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }

    /// 创建错误事件
    pub fn error(topic: String, error: IpcError) -> Self {
        Self {
            version: IPC_VERSION,
            topic,
            status: IpcStatus::Error,
            payload: None,
            error: Some(error),
            timestamp: chrono::Utc::now().timestamp_millis() as u64,
        }
    }
}
