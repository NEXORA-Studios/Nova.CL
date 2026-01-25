import { TauriConfig } from "@/modules";
import { watch } from "vue";

export function watchAndSet(data: any, key: string, extraFn?: (new_value: any) => void) {
    watch(data, async (new_value) => {
        if (new_value === undefined || new_value === null) {
            return;
        }
        await TauriConfig.set(key, new_value);
        if (extraFn) {
            extraFn(new_value);
        }
    });
}
