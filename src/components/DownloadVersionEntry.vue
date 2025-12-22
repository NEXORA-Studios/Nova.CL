<script setup lang="ts">
    import { computed } from "vue";
    import { useI18n } from "vue-i18n";
    import type { IOjngApi } from "@/types";
    import { unifyString } from "@/utils";
    import { openUrl } from "@tauri-apps/plugin-opener"

    const { locale } = useI18n();

    const props = defineProps<{
        version: IOjngApi.VersionManifest["versions"][0]["id"];
        version_type: IOjngApi.VersionManifest["versions"][0]["type"];
        date: IOjngApi.VersionManifest["versions"][0]["releaseTime"];
    }>();

    defineEmits(["start-download"]);

    const infoLink = computed(() => {
        let baseUrl: string;
        let versionPrefix: string;
        switch (locale.value) {
            case "en-US":
                baseUrl = "http://minecraft.wiki/w/";
                versionPrefix = "Java_Edition_";
                break;
            default:
                baseUrl = "http://zh.minecraft.wiki/w/";
                versionPrefix = "Java版";
                break;
        }
        return baseUrl + versionPrefix + props.version;
    });
</script>

<template>
    <li class="list-row py-2 flex items-center">
        <div></div>
        <div class="self-center -ml-2">
            <div>{{ version }}</div>
            <div class="flex gap-2 items-center">
                <div class="badge badge-xs badge-soft badge-secondary rounded-sm uppercase">
                    <span class="translate-y-[0.5px]">
                        {{ $t(`Main.Download/Minecraft.Types.${unifyString(version_type)}`) }}
                    </span>
                </div>
                <div class="text-xs uppercase opacity-60">
                    {{ $t("Main.Download/Minecraft.PublishAt", { time: new Date(date).toLocaleString() }) }}
                </div>
            </div>
        </div>
        <i
            class="icon-[material-symbols--dashboard-customize-outline-rounded] size-5 ml-auto cursor-pointer transition-all hover:text-primary active:translate-y-0.5" />
        <i
            class="icon-[material-symbols--info-outline-rounded] size-5.5 cursor-pointer transition-all hover:text-primary active:translate-y-0.5" @click="openUrl(infoLink)" />
    </li>
</template>
