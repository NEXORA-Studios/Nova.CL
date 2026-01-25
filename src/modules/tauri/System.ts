import { IpcClient } from "./IpcClient";
import { ITauriTypes } from "@/types";

export class System {
    static async getRamInfo() {
        return await IpcClient.invoke<ITauriTypes.System.RamInfo>("get_ram_info");
    }
}

export const system = {
    getRamInfo: System.getRamInfo,
};

// 配置管理类
export class Config {
    /**
     * 获取配置值
     */
    static async get(key: string): Promise<string> {
        return await IpcClient.invoke<string>("get_config", { key });
    }

    /**
     * 设置配置值
     */
    static async set(key: string, value: string): Promise<string> {
        return await IpcClient.invoke<string>("set_config", { key, value });
    }
}

export const config = {
    get: Config.get,
    set: Config.set,
};
