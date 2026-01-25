import { invoke } from "@tauri-apps/api/core";
import { CallRequest, CallResponse } from "@/types/tauri/IPC";

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
    static async invoke<T>(command: string, payload?: Record<string, any>): Promise<T> {
        // 生成唯一ID
        const id = generateId();

        // 创建符合协议的CallRequest
        const callRequest: CallRequest = {
            version: 1,
            id,
            topic: command,
            payload,
            timestamp: Date.now(),
        };

        // 调用Tauri命令，将CallRequest作为request参数传递
        const response = await invoke<CallResponse<T>>(command, { request: callRequest });

        // 处理响应
        if (response.status === "error" && response.error) {
            console.tError({
                category: "IPC Client",
                message: `Error when calling IPC Command:\n    ├─ Topic -> ${response.topic}\n    ├─ Payload -> ${JSON.stringify(callRequest.payload)}\n    ├─ Module -> ${response.error.module}\n    ├─ Code -> ${response.error.code}\n    ├─ Dev Message -> ${response.error.dev_message}\n    └─ Retryable -> ${response.error.retryable}`,
            });
            throw new Error(`${response.error.module}: Code ${response.error.code} - ${response.error.dev_message}`);
        }

        return response.payload as T;
    }
}

