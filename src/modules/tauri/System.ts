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
