import { createRouter, createWebHashHistory } from "vue-router";
import * as Pages from "@/pages";

function RouteItem(path: string, component: any, ...options: any) {
    return {
        path,
        component,
        options,
    };
}

const router = createRouter({
    history: createWebHashHistory(import.meta.env.BASE_URL),
    routes: [
        // 主页
        RouteItem("/", Pages.HomeIndex),
        // 独立子页面
        RouteItem("/account", Pages.HomeAccount),
        // 下载子页面
        RouteItem("/download/minecraft", Pages.DownloadMinecraft),
        RouteItem("/download/mod", Pages.DownloadMod),
        RouteItem("/download/modpack", Pages.DownloadModpack),
        RouteItem("/download/datapack", Pages.DownloadDatapack),
        RouteItem("/download/resourcepack", Pages.DownloadResourcepack),
        RouteItem("/download/shader", Pages.DownloadShader),
        // 更多子页面
        RouteItem("/more/about", Pages.MoreAbout),
        RouteItem("/more/tools", Pages.MoreTools),
    ],
});

export default router;
