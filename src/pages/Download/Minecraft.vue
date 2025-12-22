<script setup lang="ts">
    import { computed, onMounted, ref } from "vue";
    import { SearchInput, DownloadVersionEntry, VirtualDownloadVersionEntryList } from "@/components";
    import { Requester } from "@/modules";
    import { IOjngApi } from "@/types";

    const versions = ref<IOjngApi.VersionManifest["versions"]>([]);
    const openedTypes = ref<Record<string, boolean>>({});
    const searchInput = ref("");

    // ---------- 数据分类 ----------
    const normalVersions = computed(() =>
        versions.value.filter(
            (it) =>
                new Date(it.releaseTime).getMonth() !== 3 ||
                new Date(it.releaseTime).getDate() !== 1 ||
                it.id === "15w14a"
        )
    );

    const releaseVersions = computed(() => normalVersions.value.filter((it) => it.type === "release"));

    const snapshotVersions = computed(() => normalVersions.value.filter((it) => it.type === "snapshot"));

    const oldVersions = computed(() =>
        normalVersions.value.filter((it) => ["old_beta", "old_alpha"].includes(it.type))
    );

    const aprilfoolsVersions = computed(() =>
        versions.value.filter(
            (it) =>
                new Date(it.releaseTime).getMonth() === 3 &&
                new Date(it.releaseTime).getDate() === 1 &&
                it.id !== "15w14a"
        )
    );

    const allVersions = computed(() => ({
        "Main.Download/Minecraft.Cards.Release": releaseVersions.value,
        "Main.Download/Minecraft.Cards.Snapshot": snapshotVersions.value,
        "Main.Download/Minecraft.Cards.Old": oldVersions.value,
        "Main.Download/Minecraft.Cards.AprilFools": aprilfoolsVersions.value,
    }));

    // ---------- 搜索 ----------
    const searchVersions = computed(() => {
        const q = searchInput.value.toLowerCase();
        if (!q) return [];
        return versions.value.filter((it) => it.id.toLowerCase().includes(q));
    });

    // ---------- 折叠处理 ----------
    function onToggle(type: string) {
        openedTypes.value[type] = !openedTypes.value[type];
    }

    // ---------- 挂载 ----------
    onMounted(async () => {
        const requester = new Requester();
        versions.value = (
            await requester.get<IOjngApi.VersionManifest>(
                "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json"
            )
        ).data.versions;
    });
</script>

<template>
    <div class="w-full h-full p-6">
        <SearchInput :placeholder="$t('Main.Download/Minecraft.GlobalSearchPlaceholder')" v-model="searchInput" />

        <div class="w-full mt-4 max-h-[calc(100vh-128px-var(--spacing)*29)] rounded-md overflow-y-auto pr-2">
            <!-- 最新 -->
            <div
                v-if="searchInput.length === 0"
                class="collapse collapse-open bg-base-100 border-base-300 border w-full mb-2">
                <div class="collapse-title font-semibold">
                    {{ $t("Main.Download/Minecraft.Cards.Newest") }}
                </div>
                <div class="collapse-content text-sm">
                    <ul class="list bg-base-100 rounded-box shadow-md">
                        <DownloadVersionEntry
                            v-if="releaseVersions[0]"
                            :version="releaseVersions[0].id"
                            :version_type="releaseVersions[0].type"
                            :date="releaseVersions[0].releaseTime" />
                        <DownloadVersionEntry
                            v-if="snapshotVersions[0]"
                            :version="snapshotVersions[0].id"
                            :version_type="snapshotVersions[0].type"
                            :date="snapshotVersions[0].releaseTime" />
                    </ul>
                </div>
            </div>

            <!-- 分类列表（已优化） -->
            <div
                v-if="searchInput.length === 0"
                v-for="(list, type) in allVersions"
                :key="type"
                class="collapse collapse-arrow bg-base-100 border-base-300 border w-full mb-2">
                <input type="checkbox" @change="onToggle(type)" />

                <div class="collapse-title font-semibold">{{ $t(type) }} ({{ list.length }})</div>

                <div class="collapse-content text-sm">
                    <VirtualDownloadVersionEntryList v-if="openedTypes[type]" :items="list" />
                </div>
            </div>

            <!-- 搜索结果 -->
            <div v-else class="collapse collapse-open bg-base-100 border-base-300 border w-full mb-2">
                <div class="collapse-title font-semibold">
                    {{ $t("Main.Download/Minecraft.Cards.FilteredVersions") }}
                    ({{ searchVersions.length }})
                </div>

                <div class="collapse-content text-sm">
                    <VirtualDownloadVersionEntryList :items="searchVersions" :maxHeight="680" />
                </div>
            </div>
        </div>
    </div>
</template>
