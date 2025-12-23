// 定义请求和响应类型
export interface HttpRequest {
    method: string;
    url: string;
    headers?: Record<string, string>;
    body?: any;
}

export interface HttpResponse {
    status: number;
    headers: Record<string, string>;
    body?: any;
    text?: string;
}

export interface HttpError {
    message: string;
    code?: number;
}
