// 定义请求和响应类型
export interface HttpRequest {
    method: string;
    url: string;
    headers?: Record<string, string>;
    body?: any;
}

export interface HttpResponse<T extends any = any> {
    status: number;
    headers: Record<string, string>;
    body: T;
    text?: string;
}

export interface HttpError {
    message: string;
    code?: number;
}

// HTTP 服务器相关类型
// src/types/http.ts

export interface HttpServerStartResult {
    status: "ok";
    message: string;
    port: number;
}

export interface HttpServerStopResult {
    status: "ok";
    message: string;
}

export interface HttpServerStatusResult {
    status: "running" | "stopped";
    port?: number; // running 时才有
}

export interface OAuthCodeReceivedPayload {
    path: string; // 例如 "/auth/callback"
    query: Record<string, string>; // 查询参数，如 { code: "...", state: "..." }
}

export interface HttpServerEvent<T = any> {
    event: string;
    payload: T;
}
