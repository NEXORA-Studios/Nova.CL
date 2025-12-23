import { invoke } from "@tauri-apps/api/core";
import { ITauriTypes } from "@/types";

// HTTP 客户端类
export class HttpClient {
    /**
     * 通用请求方法
     */
    static async request(req: ITauriTypes.HTTP.HttpRequest): Promise<ITauriTypes.HTTP.HttpResponse> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_request", { req });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * GET 请求
     */
    static async get(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_get", { url, headers });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * POST 请求
     */
    static async post(
        url: string,
        headers?: Record<string, string>,
        body?: any
    ): Promise<ITauriTypes.HTTP.HttpResponse> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_post", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * PUT 请求
     */
    static async put(
        url: string,
        headers?: Record<string, string>,
        body?: any
    ): Promise<ITauriTypes.HTTP.HttpResponse> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_put", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
        }
    }

    /**
     * DELETE 请求
     */
    static async delete(url: string, headers?: Record<string, string>): Promise<ITauriTypes.HTTP.HttpResponse> {
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
    static async patch(
        url: string,
        headers?: Record<string, string>,
        body?: any
    ): Promise<ITauriTypes.HTTP.HttpResponse> {
        try {
            const response = await invoke<ITauriTypes.HTTP.HttpResponse>("http_patch", { url, headers, body });
            return response;
        } catch (error) {
            throw error as ITauriTypes.HTTP.HttpError;
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
