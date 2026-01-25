import { invoke } from "@tauri-apps/api/core";
import { CallResponse } from "@/types/tauri/IPC";

// 生成唯一ID的函数
let nextId = 1;
const generateId = (): number => {
    return nextId++;
};

// 基础IPC客户端类
export class IpcClient {
    /**
     * 调用Tauri命令的基础方法
     */
    static async invoke<T>(command: string, args?: Record<string, any>): Promise<T> {
        // 生成唯一ID
        const id = generateId();
        
        // 准备调用参数
        const invokeArgs = {
            id,
            ...args,
        };
        
        // 调用Tauri命令
        const response = await invoke<CallResponse<T>>(command, invokeArgs);
        
        // 处理响应
        if (response.status === "error" && response.error) {
            throw new Error(`${response.error.module}: ${response.error.user_message}`);
        }
        
        return response.payload as T;
    }
}
