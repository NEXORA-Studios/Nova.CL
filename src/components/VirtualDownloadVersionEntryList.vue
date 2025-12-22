<script setup lang="ts">
    import { useVirtualList } from "@vueuse/core";
    import DownloadVersionEntry from "@/components/DownloadVersionEntry.vue";
    import { toRef } from "vue";

    const props = defineProps<{
        items: {
            id: string;
            type: string;
            releaseTime: string;
        }[];
        maxHeight?: number;
    }>();

    const itemsRef = toRef(props, "items");

    const ITEM_HEIGHT = 52;

    const { list, containerProps, wrapperProps } = useVirtualList(itemsRef, {
        itemHeight: ITEM_HEIGHT,
    });
</script>

<template>
    <div v-bind="containerProps" class="overflow-y-auto" :style="{ maxHeight: (maxHeight || 288) + 'px' }">
        <ul v-bind="wrapperProps" class="list bg-base-100 rounded-box shadow-md">
            <li v-for="row in list" :key="row.data.id" :style="{ height: ITEM_HEIGHT + 'px' }">
                <DownloadVersionEntry
                    :version="row.data.id"
                    :version_type="row.data.type"
                    :date="row.data.releaseTime" />
            </li>
        </ul>
    </div>
</template>
