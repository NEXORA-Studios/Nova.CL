import { IpcClient } from "./IpcClient";
import { ITauriTypes } from "@/types";

/**
 * 日志管理类，用于调用后端日志命令
 */
export class TauriLogging {
    /**
     * 记录 TRACE 级别的日志
     *
     * @param options 日志选项
     * @returns Promise<void>
     */
    static async trace(options: ITauriTypes.Logging.LogOptions): Promise<void> {
        await IpcClient.invoke<void>("log_trace", {
            category: options.category,
            message: options.message,
        });
    }

    /**
     * 记录 DEBUG 级别的日志
     *
     * @param options 日志选项
     * @returns Promise<void>
     */
    static async debug(options: ITauriTypes.Logging.LogOptions): Promise<void> {
        await IpcClient.invoke<void>("log_debug", {
            category: options.category,
            message: options.message,
        });
    }

    /**
     * 记录 INFO 级别的日志
     *
     * @param options 日志选项
     * @returns Promise<void>
     */
    static async info(options: ITauriTypes.Logging.LogOptions): Promise<void> {
        await IpcClient.invoke<void>("log_info", {
            category: options.category,
            message: options.message,
        });
    }

    /**
     * 记录 WARN 级别的日志
     *
     * @param options 日志选项
     * @returns Promise<void>
     */
    static async warn(options: ITauriTypes.Logging.LogOptions): Promise<void> {
        await IpcClient.invoke<void>("log_warn", {
            category: options.category,
            message: options.message,
        });
    }

    /**
     * 记录 ERROR 级别的日志
     *
     * @param options 日志选项
     * @returns Promise<void>
     */
    static async error(options: ITauriTypes.Logging.LogOptions): Promise<void> {
        await IpcClient.invoke<void>("log_error", {
            category: options.category,
            message: options.message,
        });
    }
}

export const logging = {
    trace: TauriLogging.trace,
    debug: TauriLogging.debug,
    info: TauriLogging.info,
    warn: TauriLogging.warn,
    error: TauriLogging.error,
}