<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useI18n } from "vue-i18n";
    import {
        Hint,
        ModrinthProjectItem,
        ModrinthSearchFilter,
        ModrinthSearchPage,
        SearchInput,
        Loading,
        LoadingNoResult,
    } from "@/components";
    import { useResourceFilters } from "@/composables";
    import { modrinthApiAdapter } from "@/modules";
    import { IMrApi } from "@/types";

    const { t } = useI18n();

    const scrollContainer = ref<HTMLDivElement | null>(null);
    function scrollToTop() {
        scrollContainer.value?.scrollTo({ top: 0, behavior: "smooth" });
    }

    // ---------- 导入过滤器功能 ----------
    const { searchString, VersionsFilter, ModpackCategoriesFilter, ModpackLoadersFilter, ModpackOffsetFilter } =
        useResourceFilters();

    // ---------- 响应式状态 ----------
    const isLoading = ref(true);
    const hits = ref<IMrApi.SingleProjectResult[]>([]);

    // ---------- 搜索 ----------
    async function onSearch() {
        isLoading.value = true;
        hits.value = [];
        const data = await modrinthApiAdapter.searchModpack(
            searchString.value,
            VersionsFilter.raw,
            ModpackCategoriesFilter.raw,
            ModpackLoadersFilter.raw,
            20,
            ModpackOffsetFilter.offset * 20
        );
        hits.value = data.hits;
        ModpackOffsetFilter.maximum = data.total_hits;
        isLoading.value = false;
    }

    async function onPreviousPage() {
        ModpackOffsetFilter.offset -= 1;
        scrollToTop();
        await onSearch();
    }

    async function onNextPage() {
        ModpackOffsetFilter.offset += 1;
        scrollToTop();
        await onSearch();
    }

    // ---------- 挂载钩子：获取游戏版本 ----------
    onMounted(() => {
        // 并发执行
        onSearch();
        VersionsFilter.init();
        ModpackCategoriesFilter.init();
        ModpackLoadersFilter.init();
    });
</script>

<template>
    <div class="w-full h-full p-6 overflow-hidden">
        <SearchInput
            v-model="searchString"
            :placeholder="t('Main.Download/Modpack.GlobalSearchPlaceholder')"
            @click="onSearch" />

        <div
            class="w-full grid grid-cols-3 grid-rows-[auto_1fr] gap-2 mt-4 max-h-[calc(100vh-128px-var(--spacing)*29)] rounded-box overflow-auto pr-2 beautiful-scrollbar"
            ref="scrollContainer">
            <Hint type="warning" class="col-span-3">
                {{ t("Main.Download/Public.__Hint__") }}
            </Hint>

            <section class="flex flex-col gap-2">
                <!-- Categories -->
                <ModrinthSearchFilter
                    :is-open="true"
                    :title="t('Main.Download/Public.Categories.__Title__')"
                    :items="ModpackCategoriesFilter.filtered.value" />

                <!-- Game Versions -->
                <ModrinthSearchFilter
                    :is-open="false"
                    :title="t('Main.Download/Public.Versions.__Title__')"
                    :items="VersionsFilter.filtered.value">
                    <template #before>
                        <input
                            type="text"
                            class="input input-sm outline-none mb-3 w-full"
                            :placeholder="t('Main.Download/Public.Versions.__SearchPlaceholder__')"
                            v-model="VersionsFilter.search.value" />
                    </template>
                    <template #after>
                        <div class="flex items-center gap-2 mt-2">
                            <input
                                id="modversion-checkbox"
                                type="checkbox"
                                class="checkbox checkbox-sm rounded-md"
                                v-model="VersionsFilter.showAll.value" />
                            <label for="modversion-checkbox" class="text-sm">
                                {{ t("Main.Download/Public.Versions.__ShowAll__") }}
                            </label>
                        </div>
                    </template>
                </ModrinthSearchFilter>

                <!-- Loader -->
                <ModrinthSearchFilter
                    :is-open="true"
                    :title="t('Main.Download/Public.Loaders.__Title__')"
                    :items="ModpackLoadersFilter.filtered.value">
                </ModrinthSearchFilter>

                <!-- Page -->
                <ModrinthSearchPage
                    :offset="ModpackOffsetFilter.offset"
                    :maximum="ModpackOffsetFilter.maximum"
                    @previous="onPreviousPage"
                    @next="onNextPage" />
            </section>

            <!-- 结果 -->
            <div class="w-full col-span-2 flex justify-center">
                <Loading v-if="isLoading" />
                <LoadingNoResult v-if="!hits || (hits.length === 0 && !isLoading)" />
                <ul class="list bg-base-100 rounded-box shadow-md w-full" v-if="hits && hits.length > 0 && !isLoading">
                    <ModrinthProjectItem v-for="item in hits" :key="item.project_id" :data="item" />
                </ul>
            </div>
        </div>
    </div>
</template>
