<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { TauriTOML } from "@/modules";
    import { IMrApi } from "@/types";
    import { useI18n } from "vue-i18n";

    const props = defineProps<{ data: IMrApi.SingleProjectResult }>();
    const { t } = useI18n();
    const info = ref<string>();

    function formatDownloads(downloads: number) {
        if (downloads >= 1_000_000) {
            return (downloads / 1_000_000).toFixed(2) + "M";
        } else if (downloads >= 1_000) {
            return (downloads / 1_000).toFixed(2) + "K";
        } else {
            return downloads.toString();
        }
    }

    async function buildDisplayInfo(categories: string[], versions: string[]) {
        let ignore_quilt = (await TauriTOML.getGlobalConfig())["other"]["comp"]["ignore_quilt"];

        // ---------- Loader ----------
        const loaderMap: Record<string, string> = {
            fabric: "Fabric",
            forge: "Forge",
            neoforge: "NeoForge",
            quilt: "Quilt",
        };

        const loaders = new Set<string>();

        for (const c of categories.map((v) => v.toLowerCase())) {
            if (c === "quilt" && ignore_quilt) continue;
            if (loaderMap[c]) loaders.add(loaderMap[c]);
        }

        if (loaders.size >= 3) {
            loaders.clear();
        }

        const loader = loaders.size > 0 ? Array.from(loaders).sort().join(" / ") : "";

        // ---------- Versions ----------
        // 提取 x.y
        const minorSet = new Set<number>();

        for (const v of versions) {
            const m = /^(\d+)\.(\d+)/.exec(v);
            if (m) {
                const minor = Number(m[2]);
                if (minor >= 12) {
                    // 忽略 1.12 以下
                    minorSet.add(minor);
                }
            }
        }

        const minors = Array.from(minorSet).sort((a, b) => a - b);

        if (minors.length === 0) {
            return { loader, version: "" };
        }

        // 判断是否全版本（1.12 ~ 1.21）
        const FULL_START = 12;
        const FULL_END = 21;

        const isFull = minors.length === FULL_END - FULL_START + 1 && minors.every((v, i) => v === FULL_START + i);

        if (isFull) {
            return { loader, version: t("Components.ModrinthProjectItem.FullVersion") };
        }

        // 分段
        const ranges: Array<[number, number]> = [];
        let start = minors[0];
        let prev = minors[0];

        for (let i = 1; i < minors.length; i++) {
            const cur = minors[i];
            if (cur === prev + 1) {
                prev = cur;
            } else {
                ranges.push([start, prev]);
                start = cur;
                prev = cur;
            }
        }
        ranges.push([start, prev]);

        // 生成显示文本（新 → 旧）
        const parts = ranges
            .sort((a, b) => b[1] - a[1])
            .map(([s, e]) => {
                if (s === e) return `1.${s}`;
                if (e - s >= 1) return `1.${s}+`;
                return `1.${e}~1.${s}`;
            });

        return {
            loader,
            version: parts.join(", "),
        };
    }

    function formatDate(date: string): string {
        const input = new Date(date).getTime();
        const now = Date.now();

        if (isNaN(input)) return "";

        const diffMs = now - input;
        if (diffMs <= 0) return "刚刚";

        const minute = 60 * 1000;
        const hour = 60 * minute;
        const day = 24 * hour;
        const month = 30 * day; // 近似月
        const year = 365 * day; // 近似年

        if (diffMs >= year) {
            return t("Components.ModrinthProjectItem.YearsBefore", { years: Math.floor(diffMs / year) });
        }
        if (diffMs >= month) {
            return t("Components.ModrinthProjectItem.MonthsBefore", { months: Math.floor(diffMs / month) });
        }
        if (diffMs >= day) {
            return t("Components.ModrinthProjectItem.DaysBefore", { days: Math.floor(diffMs / day) });
        }
        if (diffMs >= hour) {
            return t("Components.ModrinthProjectItem.HoursBefore", { hours: Math.floor(diffMs / hour) });
        }

        return t("Components.ModrinthProjectItem.JustNow");
    }

    onMounted(async () => {
        let _info = await buildDisplayInfo(props.data.categories, props.data.versions);
        info.value = `${_info.loader} ${_info.version}`;
    });
</script>

<template>
    <li class="list-row">
        <div>
            <img class="size-10 rounded-box" :src="data.icon_url" v-if="data.icon_url" />
            <div class="bg-base-200 aspect-square size-10 rounded-box flex items-center justify-center" v-else>
                <i class="icon-[mynaui--archive] size-8 opacity-50" />
            </div>
        </div>
        <div>
            <div>{{ data.title }}</div>
            <div class="opacity-50 grid grid-cols-[1.5fr_1fr_1fr]">
                <div class="flex text-xs">
                    <i class="icon-[mynaui--git-branch] size-4 mr-1"></i>
                    {{ info }}
                </div>
                <div class="flex text-xs">
                    <i class="icon-[mynaui--download] size-4 mr-1"></i>
                    {{ formatDownloads(data.downloads) }}
                </div>
                <div class="flex text-xs">
                    <i class="icon-[material-symbols--arrow-warm-up-rounded] size-4 mr-1"></i>
                    {{ formatDate(data.date_modified) }}
                </div>
            </div>
        </div>
        <button class="btn btn-square btn-ghost">
            <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                    <path d="M6 3L20 12 6 21 6 3z"></path>
                </g>
            </svg>
        </button>
        <button class="btn btn-square btn-ghost">
            <svg class="size-[1.2em]" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                <g stroke-linejoin="round" stroke-linecap="round" stroke-width="2" fill="none" stroke="currentColor">
                    <path
                        d="M19 14c1.49-1.46 3-3.21 3-5.5A5.5 5.5 0 0 0 16.5 3c-1.76 0-3 .5-4.5 2-1.5-1.5-2.74-2-4.5-2A5.5 5.5 0 0 0 2 8.5c0 2.3 1.5 4.05 3 5.5l7 7Z"></path>
                </g>
            </svg>
        </button>
    </li>
</template>
