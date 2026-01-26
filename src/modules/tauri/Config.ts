import { IpcClient } from "./IpcClient";

/**
 * 配置管理类，用于调用后端的配置命令
 */
export class Config {
    /**
     * 获取配置值
     * @param key 配置项名称
     * @returns 配置值
     */
    static async get<T extends Object>(key: string): Promise<T> {
        const result = await IpcClient.invoke<string>("get_config", { key: key.split(".") });
        // 根据需要解析返回值
        try {
            let resp = JSON.parse(result) as T;
            return Object.values(resp)[0];
        } catch {
            return result as unknown as T;
        }
    }

    /**
     * 设置配置值
     * @param key 配置项名称
     * @param value 配置值
     * @returns 设置后的配置值
     */
    static async set<T>(key: string, value: T): Promise<T> {
        // 根据需要序列化值
        const stringValue = typeof value === "string" ? value : JSON.stringify(value);
        const result = await IpcClient.invoke<string>("set_config", { key: key.split("."), value: stringValue });
        // 返回原始值或解析后的结果
        return typeof value === "string" ? (result as unknown as T) : value;
    }
}

// 导出便捷的配置管理对象
export const config = {
    get: Config.get,
    set: Config.set,
};
