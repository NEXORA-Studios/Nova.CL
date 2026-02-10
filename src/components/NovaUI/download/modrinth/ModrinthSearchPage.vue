<script setup lang="ts">
    defineProps<{ offset: number; maximum: number }>();
    defineEmits(["previous", "next"]);
</script>

<template>
    <div class="card bg-base-100 outline-none">
        <div class="card-body">
            <div class="grid grid-cols-5 w-full">
                <button class="join-item btn text-center" @click="$emit('previous')">
                    <span class="text-lg -translate-y-0.5">«</span>
                </button>
                <button class="join-item btn col-span-3">
                    {{ $t("Main.Download/Public.Page", { offset: offset + 1, maximum: maximum }) }}
                </button>
                <button class="join-item btn text-center" @click="$emit('next')">
                    <span class="text-lg -translate-y-0.5">»</span>
                </button>
            </div>
        </div>
    </div>
</template>
