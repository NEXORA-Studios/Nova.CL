/**
 * 日志级别类型
 */
export type LogLevel = "trace" | "debug" | "info" | "warn" | "error";

/**
 * 日志选项类型
 */
export interface LogOptions {
    /**
     * 日志分类
     */
    category: string;
    /**
     * 日志内容
     */
    message: string;
}
