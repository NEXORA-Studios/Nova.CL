// 定义请求和响应类型
export interface HttpRequest {
    method: string;
    url: string;
    headers?: Record<string, string>;
    body?: any;
}

export interface HttpResponse <T extends any = any> {
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
export interface HttpServerStartResult {
    status: string;
    message: string;
    params: Record<string, string>;
}

export interface HttpServerStopResult {
    status: string;
    message: string;
}

export interface HttpServerStatusResult {
    status: "running" | "stopped";
    port?: number;
}

export interface HttpServerEvent<T> {
    event: string;
    payload: T;
}

