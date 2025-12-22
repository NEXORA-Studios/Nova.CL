import { onUnmounted } from "vue";
import { Timer } from "@/modules";
import type { HookFunction } from "@/types";

export function useTimer() {
    // Always get the same singleton instance
    const timer = Timer.getInstance();

    // --- Register interval hooks ---
    function onShortInterval(hook: HookFunction) {
        timer.addShortIntervalHook(hook);
        onUnmounted(() => timer.removeShortIntervalHook(hook));
    }

    function onMediumInterval(hook: HookFunction) {
        timer.addMediumIntervalHook(hook);
        onUnmounted(() => timer.removeMediumIntervalHook(hook));
    }

    function onLongInterval(hook: HookFunction) {
        timer.addLongIntervalHook(hook);
        onUnmounted(() => timer.removeLongIntervalHook(hook));
    }

    return {
        onShortInterval,
        onMediumInterval,
        onLongInterval,
    };
}
