# Internal IPC (Backend to Frontend)

此文件规定了 Nova.CL 主程序内部后端传前端

## 前端侧实现细节

类型定义

```typescript
/** IPC 协议版本 */
type IpcVersion = 1;

/** IPC 回复信息状态 */
type IpcStatus = "ok" | "error";

/** 错误响应体 */
interface IpcError {
    /** 错误码（模块内唯一） */
    code: number;

    /** 错误模块 */
    module: string;

    /** 面向用户的错误描述 */
    user_message: string;

    /** 面向开发者的错误描述 */
    dev_message: string;

    /** 详细错误信息 */
    detail?: string;

    /** 是否可重试 */
    retryable?: boolean;
}

/** 前端调用的响应 */
interface CallResponse<T = unknown> {
    /** 协议版本号 */
    version: IpcVersion;

    /** 对应 request 的 id，前端用来匹配 */
    id: number;

    /** 命令名（用于日志或调试） */
    topic: string;

    /** 成功或失败 */
    status: IpcStatus;

    /** 成功时的数据 */
    payload?: T;

    /** 错误信息，仅 status = "error" 时 */
    error?: IpcError;

    /** 时间戳（Unix ms） */
    timestamp: number;
}

/** 后端主动推送事件（支持错误状态） */
interface EmitEvent<T = unknown> {
    /** 协议版本号 */
    version: IpcVersion;

    /** 事件名 */
    topic: string;

    /** 成功或失败状态 */
    status: IpcStatus;

    /** 成功时的数据 */
    payload?: T;

    /** 错误信息，仅 status = "error" 时 */
    error?: IpcError;

    /** 时间戳（Unix ms） */
    timestamp: number;
}
```

保护函数

```typescript
/** 所有前端接收的 IPC 消息 */
type IpcMessage<T = unknown> = CallResponse<T> | EmitEvent<T>;

/** 类型保护函数：判断是否为响应 */
function isCallResponse<T>(msg: IpcMessage<T>): msg is CallResponse<T> {
    return (msg as CallResponse<T>).id !== undefined;
}

/** 类型保护函数：判断是否为后端主动事件 */
function isEmitEvent<T>(msg: IpcMessage<T>): msg is EmitEvent<T> {
    return (msg as EmitEvent<T>).id === undefined;
}
```

## 后端侧实现细节

```rust
use serde::{Deserialize, Serialize};

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
    pub code: u32,
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

/// 前端接收的统一消息类型
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum IpcMessage<T> {
    Call(CallResponse<T>),
    Event(EmitEvent<T>),
}
```
