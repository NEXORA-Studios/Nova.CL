import { ref } from "vue";
import { EventBus } from "@/modules";

export function useTheme() {
    const THEME = ref<string>("");

    EventBus.on("theme:change", (theme: string) => {
        THEME.value = theme;
    });

    function getTheme() {
        const LIGHT_DEFAULT = "light";
        const DARK_DEFAULT = "dark";

        if (!THEME.value || THEME.value === "auto") {
            return window.matchMedia("(prefers-color-scheme: dark)").matches ? DARK_DEFAULT : LIGHT_DEFAULT;
        }
        return THEME.value;
    }

    return {
        getTheme,
        matchTheme: (theme: string) => getTheme().toLowerCase() === theme,
    };
}
