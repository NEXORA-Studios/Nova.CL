import { invoke } from "@tauri-apps/api/core";
import { ITauriTypes } from "@/types";

export class System {
    static async getRamInfo() {
        return await invoke<ITauriTypes.System.RamInfo>("get_ram_info");
    }
}

export const system = {
    getRamInfo: System.getRamInfo,
};
