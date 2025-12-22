<script setup lang="ts">
    import { ref } from "vue";
    import ModrinthSearchOption from "./ModrinthSearchOption.vue";
    import type { IMrFilterItem } from "@/types";

    const props = defineProps<{
        isOpen?: boolean;
        title: string;
        items: IMrFilterItem[];
    }>();

    function removeSvgBorder(rawSvg: string) {
        return rawSvg.replace('<rect width="24" height="24" fill="none"/>', "").replace("M0 0h24v24H0z", "");
    }

    const isOpen = ref(props.isOpen ?? false);
</script>

<template>
    <div class="collapse collapse-arrow bg-base-100 outline-none">
        <input type="checkbox" v-model="isOpen" />
        <div class="collapse-title font-semibold">{{ title }}</div>
        <div class="collapse-content text-sm">
            <slot name="before"></slot>
            <div class="flex flex-col gap-1 max-h-64 overflow-auto pr-2 py-1 -mt-1">
                <div class="skeleton opacity-50 h-4 w-3/5" v-if="items.length === 0"></div>
                <div class="skeleton opacity-50 h-4 w-full" v-if="items.length === 0"></div>
                <ModrinthSearchOption
                    v-else
                    v-for="item in items"
                    :key="item.value"
                    v-model:checked="item.checked"
                    v-model:ignored="item.ignored"
                    :ignorable="item.ignorable">
                    <div v-if="item.icon" class="size-4 stroke-current" v-html="removeSvgBorder(item.icon)"></div>
                    {{ item.translate ? $t(item.translate) : item.label || `!+${item.value}` }}
                </ModrinthSearchOption>
            </div>
            <slot name="after"></slot>
        </div>
    </div>
</template>
