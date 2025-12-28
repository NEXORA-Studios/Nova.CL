import { invoke } from "@tauri-apps/api/core";
import { ITauriTypes } from "@/types";

// HTTP 客户端类
export class HttpClient {
    /**
     * 通用请求方法
     */
    static async request<T>(req: ITauriTypes.HTTP.HttpRequest): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_request", { req });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * GET 请求
     */
    static async get<T>(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_get", { url, headers });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * POST 请求
     */
    static async post<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_post", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * PUT 请求
     */
    static async put<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_put", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * DELETE 请求
     */
    static async delete<T>(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_delete", { url, headers });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * PATCH 请求
     */
    static async patch<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_patch", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }
}

// HTTP 服务器类
export class HttpServer {
    /**
     * 启动 HTTP 服务器，用于 OAuth 授权代码流
     */
    static async start(port: number): Promise<ITauriTypes.HTTP.HttpServerStartResult> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpServerStartResult>("http_server_start", { port });
            return response;
        } catch (error) {
            throw error as Error;
        }
    }

    /**
     * 停止 HTTP 服务器
     */
    static async stop(): Promise<ITauriTypes.HTTP.HttpServerStopResult> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpServerStopResult>("http_server_stop");
            return response;
        } catch (error) {
            throw error as Error;
        }
    }

    /**
     * 获取 HTTP 服务器状态
     */
    static async getStatus(): Promise<ITauriTypes.HTTP.HttpServerStatusResult> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpServerStatusResult>("http_server_status");
            return response;
        } catch (error) {
            throw error as Error;
        }
    }
}

// 导出便捷的请求方法
export const httpClient = {
    request: HttpClient.request,
    get: HttpClient.get,
    post: HttpClient.post,
    put: HttpClient.put,
    delete: HttpClient.delete,
    patch: HttpClient.patch,
};

// 导出便捷的服务器方法
export const httpServer = {
    start: HttpServer.start,
    stop: HttpServer.stop,
    getStatus: HttpServer.getStatus,
};
