<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useI18n } from "vue-i18n";
    import {
        ModrinthProjectItem,
        ModrinthSearchFilter,
        ModrinthSearchPage,
        SearchInput,
        Hint,
        Loading,
        LoadingNoResult,
    } from "@/components";
    import { useResourceFilters } from "@/composables";
    import { modrinthApiAdapter } from "@/modules";
    import { IMrApi } from "@/types";
    import { sortByOrderField } from "@/utils";

    const { t } = useI18n();

    // ---------- 滚动到顶部 ----------
    const scrollContainer = ref<HTMLDivElement | null>(null);
    function scrollToTop() {
        scrollContainer.value?.scrollTo({ top: 0, behavior: "smooth" });
    }

    // ---------- 导入过滤器功能 ----------
    const {
        searchString,
        VersionsFilter,
        ResourcepackOffsetFilter,
        ResourcepackCategoriesFilter,
        ResourcepackResolutionFilter,
        ResourcepackFeaturesFilter,
    } = useResourceFilters();

    // ---------- 响应式状态 ----------
    const isLoading = ref(true);
    const hits = ref<IMrApi.SingleProjectResult[]>([]);

    // ---------- 搜索 ----------
    async function onSearch() {
        isLoading.value = true;
        hits.value = [];
        const data = await modrinthApiAdapter.searchResourcepack(
            searchString.value,
            VersionsFilter.raw,
            ResourcepackCategoriesFilter.raw,
            ResourcepackResolutionFilter.raw,
            ResourcepackFeaturesFilter.raw,
            20,
            ResourcepackOffsetFilter.offset * 20
        );
        hits.value = data.hits;
        ResourcepackOffsetFilter.maximum = data.total_hits;
        isLoading.value = false;
    }

    async function onPreviousPage() {
        ResourcepackOffsetFilter.offset -= 1;
        scrollToTop();
        await onSearch();
    }

    async function onNextPage() {
        ResourcepackOffsetFilter.offset += 1;
        scrollToTop();
        await onSearch();
    }

    // ---------- 重整 ----------
    const resolutionOrder = ["8x-", "16x", "32x", "48x", "64x", "128x", "256x", "512x+"];

    // ---------- 挂载钩子：获取游戏版本 ----------
    onMounted(() => {
        // 并发执行
        onSearch();
        VersionsFilter.init();
        ResourcepackCategoriesFilter.init();
        ResourcepackFeaturesFilter.init();
        ResourcepackResolutionFilter.init();
    });
</script>

<template>
    <div class="w-full h-full p-6 overflow-hidden">
        <SearchInput
            v-model="searchString"
            :placeholder="t('Main.Download/Resourcepack.GlobalSearchPlaceholder')"
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
                    :items="ResourcepackCategoriesFilter.filtered.value" />

                <!-- Features -->
                <ModrinthSearchFilter
                    :is-open="true"
                    :title="t('Main.Download/Public.Features.__Title__')"
                    :items="ResourcepackFeaturesFilter.filtered.value" />

                <!-- Resolution -->
                <ModrinthSearchFilter
                    :is-open="true"
                    :title="t('Main.Download/Public.Resolution.__Title__')"
                    :items="sortByOrderField(ResourcepackResolutionFilter.filtered.value, resolutionOrder, 'name')" />

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

                <!-- Page -->
                <ModrinthSearchPage
                    :offset="ResourcepackOffsetFilter.offset"
                    :maximum="ResourcepackOffsetFilter.maximum"
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
