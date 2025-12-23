import { HttpClient } from "../tauri/Http";

export class Requester {
    private baseUrl?: string;

    constructor(options?: { baseUrl?: string }) {
        this.baseUrl = options?.baseUrl;
    }

    public async get<T>(url: string, config?: { headers?: Record<string, string> }) {
        const fullUrl = this.getFullUrl(url);
        const response = await HttpClient.get(fullUrl, config?.headers);
        return {
            data: response.body as T,
            status: response.status,
            statusText: this.getStatusText(response.status),
            headers: response.headers,
            config: config,
        };
    }

    public async post<T>(url: string, data?: any, config?: { headers?: Record<string, string> }) {
        const fullUrl = this.getFullUrl(url);
        const response = await HttpClient.post(fullUrl, config?.headers, data);
        return {
            data: response.body as T,
            status: response.status,
            statusText: this.getStatusText(response.status),
            headers: response.headers,
            config: config,
        };
    }

    public async put<T>(url: string, data?: any, config?: { headers?: Record<string, string> }) {
        const fullUrl = this.getFullUrl(url);
        const response = await HttpClient.put(fullUrl, config?.headers, data);
        return {
            data: response.body as T,
            status: response.status,
            statusText: this.getStatusText(response.status),
            headers: response.headers,
            config: config,
        };
    }

    public async delete<T>(url: string, config?: { headers?: Record<string, string> }) {
        const fullUrl = this.getFullUrl(url);
        const response = await HttpClient.delete(fullUrl, config?.headers);
        return {
            data: response.body as T,
            status: response.status,
            statusText: this.getStatusText(response.status),
            headers: response.headers,
            config: config,
        };
    }

    private getFullUrl(url: string): string {
        if (!this.baseUrl) return url;
        if (url.startsWith("http://") || url.startsWith("https://")) return url;

        const base = this.baseUrl.endsWith("/") ? this.baseUrl.slice(0, -1) : this.baseUrl;
        const path = url.startsWith("/") ? url : `/${url}`;
        return `${base}${path}`;
    }

    private getStatusText(status: number): string {
        const statusTexts: Record<number, string> = {
            200: "OK",
            201: "Created",
            202: "Accepted",
            204: "No Content",
            400: "Bad Request",
            401: "Unauthorized",
            403: "Forbidden",
            404: "Not Found",
            405: "Method Not Allowed",
            500: "Internal Server Error",
            501: "Not Implemented",
            502: "Bad Gateway",
            503: "Service Unavailable",
        };
        return statusTexts[status] || `Status ${status}`;
    }
}
