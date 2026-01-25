// IPC 协议版本
export type IpcVersion = 1;

// IPC 回复信息状态
export type IpcStatus = "ok" | "error";

// 错误响应体
export interface IpcError {
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

// 前端调用的响应
export interface CallResponse<T = unknown> {
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

// 后端主动推送事件（支持错误状态）
export interface EmitEvent<T = unknown> {
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

// 前端调用后端的请求
export interface CallRequest<T = unknown> {
    /** 协议版本号 */
    version: IpcVersion;

    /** 唯一 ID，由前端生成 */
    id: number;

    /** 命令名 */
    topic: string;

    /** 请求数据 */
    payload?: T;

    /** 时间戳（Unix ms） */
    timestamp: number;
}

// 所有前端接收的 IPC 消息
export type IpcMessage<T = unknown> = CallResponse<T> | EmitEvent<T>;

// 所有前端发出的 IPC 消息
export type IpcSendMessage<T = unknown> = CallRequest<T>;
