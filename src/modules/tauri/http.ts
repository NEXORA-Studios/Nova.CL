import { invoke } from "@tauri-apps/api/core";

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

// HTTP 客户端类
export class HttpClient {
    /**
     * 通用请求方法
     */
    static async request(req: HttpRequest): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_request", { req });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }

    /**
     * GET 请求
     */
    static async get(url: string, headers?: Record<string, string>): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_get", { url, headers });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }

    /**
     * POST 请求
     */
    static async post(url: string, headers?: Record<string, string>, body?: any): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_post", { url, headers, body });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }

    /**
     * PUT 请求
     */
    static async put(url: string, headers?: Record<string, string>, body?: any): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_put", { url, headers, body });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }

    /**
     * DELETE 请求
     */
    static async delete(url: string, headers?: Record<string, string>): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_delete", { url, headers });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }

    /**
     * PATCH 请求
     */
    static async patch(url: string, headers?: Record<string, string>, body?: any): Promise<HttpResponse> {
        try {
            const response = await invoke<HttpResponse>("http_patch", { url, headers, body });
            return response;
        } catch (error) {
            throw error as HttpError;
        }
    }
}

// 导出便捷的请求方法
export const http = {
    request: HttpClient.request,
    get: HttpClient.get,
    post: HttpClient.post,
    put: HttpClient.put,
    delete: HttpClient.delete,
    patch: HttpClient.patch,
};
