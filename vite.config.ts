import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig(async () => {
    const host = process.env.TAURI_DEV_HOST;
    const hmr = host ? { protocol: "ws", host, port: 1421 } : undefined;

    return {
        // Installed Plugin
        plugins: [vue(), tailwindcss()],

        // Alias
        resolve: {
            alias: {
                "@": fileURLToPath(new URL("./src", import.meta.url)),
            },
        },

        // Frontend Env Prefix
        envPrefix: ["NOVA_", "MS_"],

        // Tauri Configuration
        clearScreen: false,
        server: {
            port: 1420,
            strictPort: true,
            host: host || false,
            hmr,
            watch: {
                ignored: ["**/src-tauri/**"],
            },
        },
    };
});
