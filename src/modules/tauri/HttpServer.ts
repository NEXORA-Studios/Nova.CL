import { IpcClient } from "./IpcClient";
import { ITauriTypes } from "@/types";
import { listen, UnlistenFn, Event } from "@tauri-apps/api/event";

// HTTP 服务器类
export class HttpServer {
    private static _listener: UnlistenFn | null = null;

    /**
     * 启动 OAuth HTTP 服务器
     * @param port 监听端口
     * @param lang 可选语言
     */
    static async start(port: number, lang?: string): Promise<ITauriTypes.HTTP.HttpServerStartResult> {
        // 检查是否已经在运行
        const status = await this.getStatus();
        if (status.status === "running") {
            return {
                status: "ok",
                message: "HTTP Server already running",
                port: status.port!,
            };
        }

        return await IpcClient.invoke<ITauriTypes.HTTP.HttpServerStartResult>("http_server_start", {
            port,
            lang,
        });
    }

    /**
     * 停止 HTTP 服务器
     */
    static async stop(): Promise<ITauriTypes.HTTP.HttpServerStopResult> {
        const status = await this.getStatus();
        if (status.status === "stopped") {
            return { status: "ok", message: "HTTP Server already stopped" };
        }

        return await IpcClient.invoke<ITauriTypes.HTTP.HttpServerStopResult>("http_server_stop");
    }

    /**
     * 获取服务器运行状态
     */
    static async getStatus(): Promise<ITauriTypes.HTTP.HttpServerStatusResult> {
        try {
            return await IpcClient.invoke<ITauriTypes.HTTP.HttpServerStatusResult>("http_server_status");
        } catch (error) {
            console.error({ category: "HTTP Server", message: `Failed to get HTTP server status: ${error}` });
            // 返回一个合理的默认值，避免调用方崩溃
            return { status: "stopped" };
        }
    }

    /**
     * 监听 OAuth 授权码回调事件
     * @param callback 收到授权码时的回调
     * @returns 取消监听的函数
     */
    static async listenOAuthCode(callback: (payload: ITauriTypes.HTTP.OAuthCodeReceivedPayload) => void | Promise<void>): Promise<UnlistenFn> {
        if (this._listener) {
            // 防止重复监听
            this._listener();
        }

        const unlisten = await listen<ITauriTypes.HTTP.OAuthCodeReceivedPayload>(
            "oauth:code_received",
            (event: Event<ITauriTypes.HTTP.OAuthCodeReceivedPayload>) => {
                callback(event.payload);
            }
        );

        this._listener = unlisten;
        return unlisten;
    }

    /**
     * 取消 OAuth 事件监听（可选，手动清理）
     */
    static unlistenOAuthCode() {
        if (this._listener) {
            this._listener();
            this._listener = null;
        }
    }
}

// 导出便捷的服务器方法
export const httpServer = {
    start: HttpServer.start,
    stop: HttpServer.stop,
    getStatus: HttpServer.getStatus,
    listenOAuthCode: HttpServer.listenOAuthCode,
    unlistenOAuthCode: HttpServer.unlistenOAuthCode,
};
