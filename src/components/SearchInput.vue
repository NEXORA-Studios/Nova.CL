<script setup lang="ts">
    defineProps<{
        placeholder: string;
    }>();

    const model = defineModel<string>();
    defineEmits(["click"]);
</script>

<template>
    <div class="card w-full bg-base-100">
        <div class="card-body p-2! flex flex-row items-center relative">
            <input class="outline-none text-base h-8 ml-1 w-full" :placeholder="$props.placeholder" v-model="model" />
            <button
                class="btn aspect-square bg-transparent border-0 shadow-none absolute right-1"
                @click="$emit('click')">
                <i class="icon-[material-symbols--feature-search-outline-rounded] size-6 -mx-3"></i>
            </button>
        </div>
    </div>
</template>
