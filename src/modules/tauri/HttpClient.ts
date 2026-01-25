import { IpcClient } from "./IpcClient";
import { ITauriTypes } from "@/types";

// HTTP 客户端类
export class HttpClient {
    /**
     * 通用请求方法
     */
    static async request<T>(req: ITauriTypes.HTTP.HttpRequest): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_request", { req });
    }

    /**
     * GET 请求
     */
    static async get<T>(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_get", { url, headers });
    }

    /**
     * POST 请求
     */
    static async post<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_post", { url, headers, body });
    }

    /**
     * PUT 请求
     */
    static async put<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_put", { url, headers, body });
    }

    /**
     * DELETE 请求
     */
    static async delete<T>(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_delete", { url, headers });
    }

    /**
     * PATCH 请求
     */
    static async patch<T>(url: string, headers?: Record<string, string>, body?: any): Promise<ITauriTypes.HTTP.HttpResponse<T>> {
        return await IpcClient.invoke<ITauriTypes.HTTP.HttpResponse<T>>("http_patch", { url, headers, body });
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
