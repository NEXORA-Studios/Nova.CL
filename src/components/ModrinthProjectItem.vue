<script setup lang="ts">
    import { IMrApi } from "@/types";

    defineProps<{ data: IMrApi.SingleProjectResult }>();
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
            <div class="text-xs opacity-60 hidden xl:block">
                {{ data.description.length > 120 ? data.description.substring(0, 120) + "..." : data.description }}
            </div>
            <div class="text-xs opacity-60 xl:hidden">
                {{ data.description.length > 32 ? data.description.substring(0, 32) + "..." : data.description }}
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
